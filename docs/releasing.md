# Development and releases

The fork uses `feature/fix branch → develop → main`.

`develop` contains integrated, unreleased work. Create feature and fix branches
from `develop` and target PRs there. `main` contains the release baseline.
Only a PR from **this repository's** `develop` branch may merge into `main`.
Direct pushes, force pushes, and deleting either long-lived branch are disabled.
The source check also rejects a fork's branch named `develop`.

## Prepare a release

1. Complete feature PRs into `develop` and verify CI there.
2. On `develop`, choose the next stable version and consume the pending changesets.
   Run `pnpm install --frozen-lockfile`, then `pnpm run version-sync`.
   Review the generated changelog and version changes before committing.
   The existing version script also regenerates skills and may refresh lockfiles;
   inspect those diffs and run the checks in `CONTRIBUTING.md`.
3. Confirm `package.json`, `npm/package.json`, both crate package versions, the
   CLI's `google-workspace` dependency, and the two local packages in `Cargo.lock`
   agree. CI requires the version to exceed both `main` and existing stable
   release tags. This applies to every release PR, including maintenance.
4. Commit and push `develop`, then open a PR with head `develop` and base `main`.
   Explain the release changes and validation in its description.
5. Wait for the source, version/provenance, policy, Rust, and companion checks.
   Merge using a **merge commit**. Do not squash or delete `develop`.
6. The main workflow verifies the exact merged PR, reruns CI, and creates the
   tag and GitHub Release at that tested merge SHA, with generated release notes.
   Fast-forward `develop` to the main merge afterwards if it has no newer work;
   otherwise merge `main` into `develop`.

For example, package version `0.23.0` produces `fork-v0.23.0`. The `fork-` prefix
distinguishes this fork's releases from inherited upstream `v*` tags.
These are GitHub **source releases**; the workflow does not publish the inherited
Google-owned npm/crates packages or run upstream binary distribution workflows.
The release job never commits or pushes changelog changes to protected `main`.
Retrying a failed workflow is safe: an existing tag must point to the same commit.

The first version remains `0.22.5` until a release is deliberately prepared.
No release PR for the deferred feature set is opened by the branch migration.

## Enforced repository settings

Both branches disallow force pushes and deletion, including for administrators.
`main` requires a PR, resolved conversations, and these checks:

- `Require develop release source` (trusted base workflow).
- `Release version and provenance`.
- `quality / Release policy tests`.
- `quality / Rust formatting and Clippy`.
- `quality / Rust tests (ubuntu-latest)`.
- `quality / Rust tests (macos-latest)`.

`develop` requires PRs with its Rust, policy, and companion checks. Merge commits
are enabled; squash/rebase merging and automatic branch deletion are disabled.
The source workflow uses `pull_request_target` with read-only permissions and
checks out only the trusted base commit. It never executes PR-head code.

## One-time migration

The September 11 migration preserves all existing history. A setup PR from
`develop` to `main` removes the prematurely integrated feature code and installs
this flow. The rollback is then reverted on `develop`, restoring every feature
for continued work there.

Only the exact pre-migration main commit
`25e01ffa27da00fe18cfbd28f20cd6dca1c0528b` is exempt from the version bump and release.
That one-time setup merge publishes nothing. Every subsequent release PR must
bump the version, and every successful merge into `main` produces a release.
