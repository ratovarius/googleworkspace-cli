# google-workspace-cli

**One CLI for all of Google Workspace — built for humans and AI agents.**

`gws` dynamically generates its command surface at runtime by reading Google's [Discovery Service](https://developers.google.com/discovery). Drive, Gmail, Calendar, and every Workspace API — zero boilerplate, structured JSON output, 40+ agent skills included.

## Install this fork

This is the independently maintained [ratovarius fork](https://github.com/ratovarius/googleworkspace-cli)
of [googleworkspace/cli](https://github.com/googleworkspace/cli).
Original authorship and the Apache-2.0 license are retained.

```bash
cargo install --git https://github.com/ratovarius/googleworkspace-cli --branch main --locked google-workspace-cli
```

Upstream registry packages and binary releases do not include fork-only changes.
See the [fork ledger](https://github.com/ratovarius/googleworkspace-cli/blob/main/FORK.md)
for improvements and upstream PRs.

## Quick Start

```bash
gws auth login
gws drive files list --params '{"pageSize": 5}'
gws gmail users.messages list --params '{"maxResults": 3}'
```

## Documentation

See the [full README](https://github.com/ratovarius/googleworkspace-cli#readme) for authentication setup, helper commands, agent skills, and more.

## License

Apache-2.0 — see [LICENSE](https://github.com/ratovarius/googleworkspace-cli/blob/main/LICENSE).
