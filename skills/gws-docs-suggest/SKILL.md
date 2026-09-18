---
name: gws-docs-suggest
description: "Google Docs: Create and manage Docs suggestions."
metadata:
  version: 0.23.0
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws docs +suggest --help"
---

# docs +suggest

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

Create and manage Docs suggestions

## Usage

```bash
gws docs +suggest
```

## Examples

```bash
gws docs +suggest insert --document DOC_ID --text 'Suggested text'
gws docs +suggest replace --document DOC_ID --find 'old' --text 'new'
gws docs +suggest list --document DOC_ID
gws docs +suggest accept --document DOC_ID --suggestion-id SUGGESTION_ID
```

## Tips

- Suggestion writes are a Google Workspace Developer Preview feature.
- The helper opts into unknown preview fields internally; raw commands remain strict.
- Use --dry-run to preview insert, delete-text, accept, reject, and delete requests.
- replace reads the document to locate exactly one matching text run before writing.

## See Also

- [gws-shared](../gws-shared/SKILL.md) — Global flags and auth
- [gws-docs](../gws-docs/SKILL.md) — All read and write Google Docs commands
