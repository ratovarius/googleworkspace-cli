# Upstream automation reference

These eleven workflow files are preserved unchanged from
`googleworkspace/cli` at `a3768d0e82ad83cca2da97724e46bea4ff0e6dbd`.
GitHub Actions only loads workflows from `.github/workflows`, so these archived
copies do not run in this fork.

The upstream workflows include Google-specific CLA, bot, package publishing,
live-account smoke tests, and policy integrations. The fork instead runs
the reusable `fork-ci.yml` with read-only permissions and synthetic test data.
It tests available Docs companions on both platforms. The fork's `release.yml`
calls those checks and publishes source releases after a versioned
`develop` → `main` merge; `release-source.yml` enforces the PR source.
They need no Google account credentials.

This initial fork CI covers Rust tests/builds and the two Python companions on
Linux and macOS, plus Rust formatting and Clippy. It does not reproduce the
upstream release matrix, Nix checks, coverage reporting, dependency audit, or
scheduled skill regeneration.

When syncing upstream, review changes here and any newly introduced active
workflows. Port useful checks deliberately; restore publishing only after
configuring a distinct fork release identity and destinations. See
[`CONTRIBUTING.md`](../../CONTRIBUTING.md).
