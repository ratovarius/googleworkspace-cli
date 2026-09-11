"""Exercise release policy with synthetic GitHub events and real git fixtures."""

import copy
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / ".github/release_policy.py"
REPO = "ratovarius/googleworkspace-cli"
BOOTSTRAP = "25e01ffa27da00fe18cfbd28f20cd6dca1c0528b"


def release_pr():
    return {
        "base": {"ref": "main", "repo": {"full_name": REPO}},
        "head": {"ref": "develop", "repo": {"full_name": REPO}},
        "merged_at": "2026-09-11T20:00:00Z",
        "merge_commit_sha": "a" * 40,
    }


class ReleasePolicyTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        fixtures = {
            "package.json": '{"version": "0.22.5"}',
            "npm/package.json": '{"version": "0.22.5"}',
            "crates/google-workspace/Cargo.toml": '[package]\nversion = "0.22.5"\n',
            "crates/google-workspace-cli/Cargo.toml": (
                '[package]\nversion = "0.22.5"\n[dependencies]\n'
                'google-workspace = { version = "0.22.5", path = "../google-workspace" }\n'
            ),
            "Cargo.lock": (
                '[[package]]\nname = "google-workspace"\nversion = "0.22.5"\n'
                '[[package]]\nname = "google-workspace-cli"\nversion = "0.22.5"\n'
            ),
        }
        for filename, content in fixtures.items():
            destination = self.root / filename
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(content)
        self.git("init", "-q")
        self.git("add", ".")
        self.git(
            "-c", "user.name=Test", "-c", "user.email=test@example.invalid",
            "commit", "-qm", "baseline",
        )
        self.base = self.git("rev-parse", "HEAD").strip()
        self.git("tag", "v0.22.5")

    def git(self, *args):
        return subprocess.check_output(
            ["git", *args], cwd=self.root, text=True, stderr=subprocess.PIPE
        )

    def run_policy(self, *args):
        return subprocess.run(
            [sys.executable, str(SCRIPT), *args],
            cwd=self.root, text=True, capture_output=True,
        )

    def event(self, data, name="event.json"):
        path = self.root / name
        path.write_text(json.dumps(data))
        return str(path)

    def bump(self):
        for path in self.root.rglob("*"):
            if path.is_file() and ".git" not in path.parts:
                path.write_text(path.read_text().replace("0.22.5", "0.23.0"))

    def test_accepts_same_repository_develop_release_source(self):
        result = self.run_policy(
            "source", "--event", self.event({"pull_request": release_pr()}),
            "--repository", REPO,
        )
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_rejects_other_branches_and_fork_named_develop(self):
        for field, value in (
            ("branch", "feat/new-feature"),
            ("repository", "someone/googleworkspace-cli"),
            ("deleted", None),
        ):
            with self.subTest(field=field):
                pr = release_pr()
                if field == "branch":
                    pr["head"]["ref"] = value
                elif field == "deleted":
                    pr["head"]["repo"] = value
                else:
                    pr["head"]["repo"]["full_name"] = value
                result = self.run_policy(
                    "source", "--event", self.event({"pull_request": pr}),
                    "--repository", REPO,
                )
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("develop", result.stderr)

    def test_release_requires_a_new_version_even_for_maintenance(self):
        result = self.run_policy("version", "--base", self.base)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("newer", result.stderr)

    def test_accepts_synchronized_version_bump_with_fork_tag(self):
        self.bump()
        result = self.run_policy("version", "--base", self.base)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("tag=fork-v0.23.0", result.stdout)
        self.assertIn("bootstrap=false", result.stdout)

    def test_rejects_version_behind_existing_release(self):
        self.bump()
        self.git("tag", "fork-v0.24.0")
        result = self.run_policy("version", "--base", self.base)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("newer", result.stderr)

    def test_release_retry_accepts_existing_tag_only_at_same_commit(self):
        self.bump()
        self.git("add", ".")
        self.git(
            "-c", "user.name=Test", "-c", "user.email=test@example.invalid",
            "commit", "-qm", "release",
        )
        sha = self.git("rev-parse", "HEAD").strip()
        self.git("tag", "fork-v0.23.0")
        result = self.run_policy(
            "version", "--base", self.base, "--release-sha", sha,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        result = self.run_policy(
            "version", "--base", self.base, "--release-sha", "b" * 40,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("different commit", result.stderr)

    def test_older_release_can_retry_after_a_newer_release(self):
        self.bump()
        self.git("add", ".")
        self.git(
            "-c", "user.name=Test", "-c", "user.email=test@example.invalid",
            "commit", "-qm", "first release",
        )
        sha = self.git("rev-parse", "HEAD").strip()
        self.git("tag", "fork-v0.23.0")
        (self.root / "later.txt").write_text("A later release")
        self.git("add", ".")
        self.git(
            "-c", "user.name=Test", "-c", "user.email=test@example.invalid",
            "commit", "-qm", "later release",
        )
        self.git("tag", "fork-v0.24.0")
        result = self.run_policy(
            "version", "--base", self.base, "--release-sha", sha,
        )
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_rejects_each_stale_version_location(self):
        self.bump()
        paths = (
            "npm/package.json", "crates/google-workspace/Cargo.toml",
            "crates/google-workspace-cli/Cargo.toml", "Cargo.lock",
        )
        for filename in paths:
            with self.subTest(filename=filename):
                path = self.root / filename
                original = path.read_text()
                path.write_text(original.replace("0.23.0", "0.22.5"))
                result = self.run_policy("version", "--base", self.base)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("version", result.stderr)
                path.write_text(original)

    def test_rejects_stale_internal_dependency(self):
        self.bump()
        path = self.root / "crates/google-workspace-cli/Cargo.toml"
        path.write_text(path.read_text().replace(
            'google-workspace = { version = "0.23.0"',
            'google-workspace = { version = "0.22.5"',
        ))
        result = self.run_policy("version", "--base", self.base)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("version", result.stderr)

    def test_rejects_nonstable_or_malformed_versions(self):
        for version in ("0.23.0-rc.1", "01.23.0", "v0.23.0", "0.23", "0.23.0\n"):
            with self.subTest(version=version):
                path = self.root / "package.json"
                data = json.loads(path.read_text())
                data["version"] = version
                path.write_text(json.dumps(data))
                result = self.run_policy("version", "--base", self.base)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("version", result.stderr)

    def test_only_exact_migration_base_can_skip_initial_release(self):
        # Use the real repository to resolve the historical bootstrap commit.
        result = subprocess.run(
            [sys.executable, str(SCRIPT), "version", "--base", BOOTSTRAP],
            cwd=ROOT, text=True, capture_output=True,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("bootstrap=true", result.stdout)
        result = self.run_policy("version", "--base", self.base)
        self.assertNotEqual(result.returncode, 0)

    def test_accepts_exact_merged_release_pull_request(self):
        result = self.run_policy(
            "merge", "--pull-requests", self.event([release_pr()]),
            "--sha", "a" * 40, "--repository", REPO,
        )
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_rejects_direct_push_unmerged_and_unrelated_pull_requests(self):
        pr = release_pr()
        unmerged = copy.deepcopy(pr)
        unmerged["merged_at"] = None
        unrelated = copy.deepcopy(pr)
        unrelated["merge_commit_sha"] = "b" * 40
        wrong_source = copy.deepcopy(pr)
        wrong_source["head"]["ref"] = "feature/example"
        for prs in ([], [unmerged], [unrelated], [wrong_source]):
            with self.subTest(prs=prs):
                result = self.run_policy(
                    "merge", "--pull-requests", self.event(prs),
                    "--sha", "a" * 40, "--repository", REPO,
                )
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("merged", result.stderr)


if __name__ == "__main__":
    unittest.main()
