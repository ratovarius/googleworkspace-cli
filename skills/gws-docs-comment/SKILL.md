---
name: gws-docs-comment
description: "Google Docs: Create a comment anchored to document text."
metadata:
  version: 0.23.0
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws docs +comment --help"
---

# docs +comment

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

Create a comment anchored to document text

## Usage

```bash
gws docs +comment
```

## Examples

```bash
gws docs +comment create --document DOC_ID --text 'Please review this.' --start-index 1 --end-index 20
```

## Tips

- Indexes are UTF-16 document indexes.
- Comment creation is a Google Workspace Developer Preview feature.
- Use --dry-run to validate without authentication or sending the request.

## See Also

- [gws-shared](../gws-shared/SKILL.md) — Global flags and auth
- [gws-docs](../gws-docs/SKILL.md) — All read and write google docs commands
