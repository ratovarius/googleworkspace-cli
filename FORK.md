# About this fork

This is **ratovarius/googleworkspace-cli**, an independently maintained public fork of
[googleworkspace/cli](https://github.com/googleworkspace/cli). The original
project and its contributors built `gws`; this fork builds on their work.
Original history, copyright notices, and the [Apache-2.0 license](LICENSE)
are retained. This fork is not an official Google product and does not speak
for the upstream maintainers.

The fork exists to improve Google Docs writing and review workflows. Development
here continues independently. A quiet upstream branch is not treated as an
announcement that the original project has been abandoned.

## Current upstream pilot

The sole active upstream contribution is
[#937: preserve saved credentials after authentication failures](https://github.com/googleworkspace/cli/pull/937).
This basic reliability fix benefits all Workspace commands. Its branch includes
the small Clippy cleanup as a separate commit, so it requires no other PR.

The other seven proposals were closed by author request to focus upstream review.
Their code is retained on this fork's `develop`; their closure does not indicate
rejection by the original maintainers. Further development of those improvements
continues through issues and PRs in this fork. Additional upstream submissions
are outside the current pilot. This policy is tracked in
[fork issue #11](https://github.com/ratovarius/googleworkspace-cli/issues/11).

## Unreleased improvements and upstream contributions

The initial fork is based on upstream commit
[`a3768d0`](https://github.com/googleworkspace/cli/commit/a3768d0e82ad83cca2da97724e46bea4ff0e6dbd).
The features below are integrated on `develop` and are **not yet released on
`main`**. Only the Clippy compatibility maintenance remains on both branches.
The upstream PR
links show their current acceptance status; inclusion here does not imply
upstream acceptance.

| Improvement | Workflow benefit | Fork issue | Upstream proposal |
| --- | --- | --- | --- |
| Explicit `--allow-unknown-fields` | Send supported preview request fields missing from Discovery, while retaining known-field validation. Google preview enrollment still applies. | [#1](https://github.com/ratovarius/googleworkspace-cli/issues/1) | [#931](https://github.com/googleworkspace/cli/pull/931) — closed |
| `gws docs +read` | Read text, formatting, tables, images, and all tabs in a compact structured response. | [#2](https://github.com/ratovarius/googleworkspace-cli/issues/2) | [#932](https://github.com/googleworkspace/cli/pull/932) — closed |
| Reviewed text patch companion | Review one precise replacement, bind it to the document revision, and verify the result. | [#3](https://github.com/ratovarius/googleworkspace-cli/issues/3) | [#933](https://github.com/googleworkspace/cli/pull/933) — closed |
| Visual review bundle companion | Collect document structure, PDF, DOCX, Markdown, assets, and an HTML review view in one local folder. | [#4](https://github.com/ratovarius/googleworkspace-cli/issues/4) | [#934](https://github.com/googleworkspace/cli/pull/934) — closed |
| Explicit trusted file root | Export or upload within a chosen working directory with path checks. | [#5](https://github.com/ratovarius/googleworkspace-cli/issues/5) | [#935](https://github.com/googleworkspace/cli/pull/935) — closed |
| Credential-free request previews | Validate raw requests and Docs appends without opening credentials or sending the request. Offline use requires a fresh cached Discovery schema. | [#6](https://github.com/ratovarius/googleworkspace-cli/issues/6) | [#936](https://github.com/googleworkspace/cli/pull/936) — closed |
| Preserve credentials after decryption failure | Keep saved credentials intact and report the error instead of silently falling back to another account. | [#7](https://github.com/ratovarius/googleworkspace-cli/issues/7) | [#937](https://github.com/googleworkspace/cli/pull/937) — active pilot |
| Current Clippy compatibility | Keep the required Rust lint check passing. | [#8](https://github.com/ratovarius/googleworkspace-cli/issues/8) | [#930](https://github.com/googleworkspace/cli/pull/930) — closed; fix included in #937 |

The review and bundle tools are Python standard-library examples for POSIX
systems, not built-in `gws` subcommands. Their setup and limits are documented
in `examples/docs-review/README.md` and
`examples/docs-review-bundle/README.md` on `develop`. Exports may span
different document revisions; the bundle reports those limits instead of
claiming an atomic snapshot.

The initial publication is tracked in [fork issue #9](https://github.com/ratovarius/googleworkspace-cli/issues/9).

## Branches and distribution

- `develop` integrates feature and fix PRs for testing before release.
- `main` is the release baseline, plus fork documentation and CI. Its only PR
  source is this repository's `develop`; merging a versioned release PR creates
  a `fork-v<version>` GitHub release after all checks pass.
- `upstream-main` records the last imported upstream `main` unchanged. It is
  updated deliberately during upstream sync, not automatically.
- Individual `feat/*` and `fix/*` branches preserve the original proposals.
  `fix/preserve-credentials` backs the active upstream pilot. Start further fork
  development from this fork's current `develop`, keeping the pilot branch focused.

The [release guide](docs/releasing.md) documents the enforced branch rules,
version preparation, and the one-time migration that removed unreleased features
from `main` without rewriting history.

Install this fork from source using the [README](README.md#installation).
The upstream npm, crates.io, Homebrew, and binary releases do not include
fork-only changes. Package names and version `0.22.5` currently retain their
upstream values for source compatibility; identify a fork build by its Git
commit. The inherited `npm/` downloader remains an upstream distribution tool,
not an installer for this fork.

Before distributing fork binaries or registry packages, choose a distinct
fork release version and owned package namespace, update download metadata,
and validate the intended platforms. The initial publication provides source
on GitHub; it does not publish packages under Google's namespace.

## Contributing here

Open issues and PRs [in this fork](https://github.com/ratovarius/googleworkspace-cli/issues)
for work maintained here. Only the credential-preservation pilot is currently
being offered upstream. Keep its issue and PR cross-linked so users can follow
both paths.

The [contribution guide](CONTRIBUTING.md) gives the commands for fork changes,
testing, importing upstream changes, and any future decision to resume broader
upstream contributions. If upstream does not respond, reviewed changes can still
land here and remain available to the community. Closed proposal discussions
remain available as history.
