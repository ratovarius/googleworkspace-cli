#!/usr/bin/env python3
"""Validate release provenance and synchronized versions without credentials."""

import argparse
import json
from pathlib import Path
import re
import subprocess
import sys
import tomllib


# Only the one-time branch migration skips publication. This immutable old main
# commit cannot become main again under the installed no-force-push protection.
BOOTSTRAP_BASE = "25e01ffa27da00fe18cfbd28f20cd6dca1c0528b"
STABLE_VERSION = re.compile(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)")


def git(*args):
    return subprocess.check_output(["git", *args], text=True).strip()


def parse_version(value):
    if not isinstance(value, str) or not STABLE_VERSION.fullmatch(value):
        raise ValueError(f"Expected a stable major.minor.patch version, got {value!r}")
    return tuple(int(part) for part in value.split("."))


def is_release_source(pr, repository):
    base = pr.get("base") or {}
    head = pr.get("head") or {}
    return (
        base.get("ref") == "main"
        and head.get("ref") == "develop"
        and (base.get("repo") or {}).get("full_name") == repository
        and (head.get("repo") or {}).get("full_name") == repository
    )


def synchronized_version():
    version = json.loads(Path("package.json").read_text())["version"]
    parse_version(version)
    versions = {"npm/package.json": json.loads(Path("npm/package.json").read_text())["version"]}
    for name in ("google-workspace", "google-workspace-cli"):
        filename = f"crates/{name}/Cargo.toml"
        data = tomllib.loads(Path(filename).read_text())
        versions[filename] = data["package"]["version"]
        if name == "google-workspace-cli":
            versions["internal dependency"] = data["dependencies"]["google-workspace"]["version"]
    packages = tomllib.loads(Path("Cargo.lock").read_text())["package"]
    for name in ("google-workspace", "google-workspace-cli"):
        matches = [p["version"] for p in packages if p["name"] == name and "source" not in p]
        if len(matches) != 1:
            raise ValueError(f"Expected exactly one local {name} version in Cargo.lock")
        versions[f"Cargo.lock:{name}"] = matches[0]
    for location, actual in versions.items():
        if actual != version:
            raise ValueError(f"Unsynchronized version in {location}: {actual!r}; expected {version}")
    return version


def version_plan(base, release_sha=None):
    # Resolve revisions before constructing git show object names.
    base_sha = git("rev-parse", "--verify", f"{base}^{{commit}}")
    version = synchronized_version()
    tag = f"fork-v{version}"
    if base_sha == BOOTSTRAP_BASE:
        return {"version": version, "tag": tag, "bootstrap": "true"}
    base_version = json.loads(git("show", f"{base_sha}:package.json"))["version"]
    previous = [parse_version(base_version)]
    tags = git("tag", "--list").splitlines()
    if release_sha:
        # A retry may happen after later releases. Validate this merge against
        # its own history; never replace a tag attached to another commit.
        if tag in tags and git("rev-parse", f"refs/tags/{tag}^{{commit}}") != release_sha:
            raise ValueError(f"Release version {version} already tags a different commit")
        tags = git("tag", "--merged", release_sha).splitlines()
    for existing in tags:
        match = re.fullmatch(r"(?:fork-)?v(.+)", existing)
        if not match or not STABLE_VERSION.fullmatch(match[1]):
            continue
        if existing == tag and release_sha:
            if git("rev-parse", f"refs/tags/{tag}^{{commit}}") != release_sha:
                raise ValueError(f"Release version {version} already tags a different commit")
            continue
        previous.append(parse_version(match[1]))
    if parse_version(version) <= max(previous):
        raise ValueError("Release version must be newer than main and all existing release tags")
    return {"version": version, "tag": tag, "bootstrap": "false"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    source = commands.add_parser("source")
    source.add_argument("--event", required=True)
    source.add_argument("--repository", required=True)
    version = commands.add_parser("version")
    version.add_argument("--base", required=True)
    version.add_argument("--release-sha")
    merge = commands.add_parser("merge")
    merge.add_argument("--pull-requests", required=True)
    merge.add_argument("--sha", required=True)
    merge.add_argument("--repository", required=True)
    args = parser.parse_args()
    try:
        if args.command == "source":
            event = json.loads(Path(args.event).read_text())
            if not is_release_source(event.get("pull_request") or {}, args.repository):
                raise ValueError("Only this repository's develop branch may open a release PR to main")
        elif args.command == "merge":
            prs = json.loads(Path(args.pull_requests).read_text())
            if not any(
                is_release_source(pr, args.repository)
                and pr.get("merged_at")
                and pr.get("merge_commit_sha") == args.sha
                for pr in prs
            ):
                raise ValueError("Main must be the exact result of a merged develop release PR")
        else:
            for key, value in version_plan(args.base, args.release_sha).items():
                print(f"{key}={value}")
    except (ValueError, KeyError, OSError, subprocess.CalledProcessError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
