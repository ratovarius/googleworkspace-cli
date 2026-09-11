# Contributing

This repository is the independently maintained
[ratovarius/cli fork](FORK.md) of
[googleworkspace/cli](https://github.com/googleworkspace/cli).
The fork welcomes bug reports and focused improvements. Read [AGENTS.md](AGENTS.md)
for architecture, validation, testing, and changeset conventions.

## Develop a change in this fork

1. Search [fork issues](https://github.com/ratovarius/cli/issues) and
   [upstream issues](https://github.com/googleworkspace/cli/issues) first.
   Create a fork issue describing the problem, expected behavior, and relevant
   upstream links.
2. Branch from this fork's current `main`. Keep one concern per branch.
3. Add a regression test, implement the change, and add a `.changeset/*.md`
   entry. Existing package names in changesets are retained for upstream
   portability.
4. Run the relevant checks below and open a PR against `ratovarius/cli:main`.
   Link the fork issue with `Fixes #NUMBER`, describe the behavior and test
   evidence, and disclose any untested platform or live-API assumptions.
5. Review the diff and CI results before merging. Keep branches backing open
   upstream PRs until those PRs are resolved.

For a checkout with push access to this fork:

```bash
git clone https://github.com/ratovarius/cli.git
cd cli
git remote add upstream https://github.com/googleworkspace/cli.git
git fetch origin
git switch -c feat/my-change origin/main
# Make and test a focused change, then commit it.
git push -u origin feat/my-change
gh pr create --repo ratovarius/cli --base main --head feat/my-change
```

Other contributors can fork `ratovarius/cli` into their own account and open
a cross-fork PR. Use Conventional Commits, subjects under 72 characters, and
no `Co-Authored-By` trailers.

## Checks

From the repository root, with stable Rust and Python 3.10+ available:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --locked
python3 -B -m unittest discover -s examples/docs-review -p 'test_*.py' -v
GWS_TEST_BINARY="$PWD/target/debug/gws" \
  python3 -B -m unittest discover -s examples/docs-review-bundle -p 'test_*.py' -v
```

If `CARGO_TARGET_DIR` is set, point `GWS_TEST_BINARY` at that directory's
`debug/gws` instead. This enables the real-CLI export regression rather than
skipping it. Tests use synthetic data and local stub servers. They do not need
a Google account; dependency/toolchain downloads may need network access.
Live API behavior, preview enrollment, and actual OS keyring interactions
need separate, explicit validation.

The active workflows run Rust and companion checks on Linux and macOS.
The [archived upstream automation](.github/upstream-workflows/README.md)
documents which upstream checks and release jobs are not enabled here.

## Current upstream pilot and future submissions

The current policy is to keep
[googleworkspace/cli#937](https://github.com/googleworkspace/cli/pull/937)
as the sole active upstream contribution. All other feature development continues
in this fork. Proposals #930–#936 were closed by author request, with their code
and history retained here. No additional upstream PRs are planned during this
pilot.

The remaining instructions in this section are a reference for a future
maintainer decision to resume broader upstream contributions.

Maintain two PRs when a change belongs in both projects: one for inclusion
here and one for upstream acceptance. A fork-local merge does not merge the
upstream PR.

Use a clean branch based on **upstream `main`** for the upstream submission.
This keeps fork branding, CI policy, and unrelated features out of its diff.
For a change already committed in this fork:

```bash
git fetch upstream
git switch -c upstream/my-change upstream/main
# Replace COMMIT_SHA with the focused implementation commit(s).
git cherry-pick COMMIT_SHA
# Resolve any differences, include necessary prerequisites, and retest.
git push -u origin upstream/my-change
gh pr create --repo googleworkspace/cli --base main \
  --head ratovarius:upstream/my-change
```

Check the complete diff against `upstream/main`, search existing upstream
PRs, and link the fork issue and any prerequisite PR. Include a changeset and
the test evidence required by upstream. If dependencies would make the PR
large, submit the prerequisite separately and explain the dependency.

Google's CLA check and approval to run workflows from a fork are controlled
by upstream. The contributor must complete any required CLA themselves;
upstream maintainers decide whether to approve CI and merge. Development and
merges in this fork can continue while that review is pending.

## Bring upstream changes into the fork

Keep `upstream-main` as a pristine record and import changes through a
reviewed sync PR:

```bash
git fetch origin
git fetch upstream
git push origin upstream/main:refs/heads/upstream-main
git switch -c chore/sync-upstream origin/main
git merge upstream/main
# Resolve conflicts, inspect the changes, and run the checks above.
git push -u origin chore/sync-upstream
gh pr create --repo ratovarius/cli --base main --head chore/sync-upstream
```

Choose a new branch name if a sync PR is already open. If the pristine branch
push is rejected, investigate the changed upstream history before proceeding.
Do not force-sync or reset the maintained `main` to upstream: it contains
the fork's improvements.

During the merge, check `.github/workflows/` for newly introduced upstream
automation. Retain the fork's own publisher destinations, ownership, and
credential-independent CI. Upstream workflows preserved under
`.github/upstream-workflows/` are reference copies, not active checks.

If an upstream PR is merged, reconcile its accepted form in the sync PR and
link it from the fork issue. Preserve attribution and avoid reapplying an
equivalent patch twice.

GitHub documents [PRs from forks](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request-from-a-fork)
and [syncing a fork](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/syncing-a-fork).
