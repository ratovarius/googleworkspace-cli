# About this fork

This is **ratovarius/cli**, an independently maintained public fork of
[googleworkspace/cli](https://github.com/googleworkspace/cli). The original
project and its contributors built `gws`; this fork builds on their work.
Original history, copyright notices, and the [Apache-2.0 license](LICENSE)
are retained. This fork is not an official Google product and does not speak
for the upstream maintainers.

The fork exists to improve Google Docs writing and review workflows and keep
those improvements available while upstream contributions are reviewed.
Development here can continue independently. A quiet upstream branch is not
treated as an announcement that the original project has been abandoned.

## Included improvements and upstream contributions

The initial fork is based on upstream commit
[`a3768d0`](https://github.com/googleworkspace/cli/commit/a3768d0e82ad83cca2da97724e46bea4ff0e6dbd).
The improvements below are included on this fork's `main`. The upstream PR
links show their current acceptance status; inclusion here does not imply
upstream acceptance.

| Improvement | Workflow benefit | Fork issue | Upstream PR |
| --- | --- | --- | --- |
| Explicit `--allow-unknown-fields` | Send supported preview request fields missing from Discovery, while retaining known-field validation. Google preview enrollment still applies. | [#1](https://github.com/ratovarius/cli/issues/1) | [#931](https://github.com/googleworkspace/cli/pull/931) |
| `gws docs +read` | Read text, formatting, tables, images, and all tabs in a compact structured response. | [#2](https://github.com/ratovarius/cli/issues/2) | [#932](https://github.com/googleworkspace/cli/pull/932) |
| Reviewed text patch companion | Review one precise replacement, bind it to the document revision, and verify the result. | [#3](https://github.com/ratovarius/cli/issues/3) | [#933](https://github.com/googleworkspace/cli/pull/933) |
| Visual review bundle companion | Collect document structure, PDF, DOCX, Markdown, assets, and an HTML review view in one local folder. | [#4](https://github.com/ratovarius/cli/issues/4) | [#934](https://github.com/googleworkspace/cli/pull/934) |
| Explicit trusted file root | Export or upload within a chosen working directory with path checks. | [#5](https://github.com/ratovarius/cli/issues/5) | [#935](https://github.com/googleworkspace/cli/pull/935) |
| Credential-free request previews | Validate raw requests and Docs appends without opening credentials or sending the request. Offline use requires a fresh cached Discovery schema. | [#6](https://github.com/ratovarius/cli/issues/6) | [#936](https://github.com/googleworkspace/cli/pull/936) |
| Preserve credentials after decryption failure | Keep saved credentials intact and report the error instead of silently falling back to another account. | [#7](https://github.com/ratovarius/cli/issues/7) | [#937](https://github.com/googleworkspace/cli/pull/937) |
| Current Clippy compatibility | Keep the required Rust lint check passing. | [#8](https://github.com/ratovarius/cli/issues/8) | [#930](https://github.com/googleworkspace/cli/pull/930) |

The review and bundle tools are Python standard-library examples for POSIX
systems, not built-in `gws` subcommands. Their setup and limits are documented
in [docs-review](examples/docs-review/README.md) and
[docs-review-bundle](examples/docs-review-bundle/README.md). Exports may span
different document revisions; the bundle reports those limits instead of
claiming an atomic snapshot.

The initial publication is tracked in [fork issue #9](https://github.com/ratovarius/cli/issues/9).

## Branches and distribution

- `main` is the maintained fork, including integrated improvements and fork
  documentation and CI.
- `upstream-main` records the last imported upstream `main` unchanged. It is
  updated deliberately during upstream sync, not automatically.
- Individual `feat/*` and `fix/*` branches back the focused upstream PRs.
  They stay separate from this fork's publication and maintenance changes.

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

## Contributing in both places

Open issues and PRs [in this fork](https://github.com/ratovarius/cli/issues)
for work maintained here. Generally useful improvements can also be offered
upstream as focused PRs based on upstream `main`. Keep cross-links so users
can follow both paths.

The [contribution guide](CONTRIBUTING.md) gives the commands for fork changes,
upstream submissions, testing, and importing upstream changes. If upstream
does not respond, reviewed changes can still land here and remain available
to the community. The original PRs remain available for later review.
