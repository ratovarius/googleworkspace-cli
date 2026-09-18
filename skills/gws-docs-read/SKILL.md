---
name: gws-docs-read
description: "Google Docs: Read a document as compact structured content."
metadata:
  version: 0.23.0
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws docs +read --help"
---

# docs +read

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

Read a document as compact structured content

## Usage

```bash
gws docs +read --document <ID>
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--document` | ✓ | — | Document ID |
| `--params` | — | — | Additional documents.get API parameters as JSON |
| `--include-comments` | — | — | Include comments and their referenced text |

## Examples

```bash
gws docs +read --document DOC_ID
gws docs +read --document DOC_ID --format yaml
gws docs +read --document DOC_ID --params '{"fields":"*"}' --dry-run
gws docs +read --document DOC_ID | jq '.outline'
gws docs +read --document DOC_ID | jq '.. | objects | select(.paragraphStyle?.headingId? == "HEADING_ID")'
```

## Tips

- Requests all tabs with includeTabsContent=true and suggestionsViewMode=SUGGESTIONS_INLINE.
- Only those tab/suggestion options are supported; fields must be absent or exactly "*".
- Other documents.get options pass through --params; alt must be json. $fields is rejected.
- JSON/YAML preserve the structured view; table/CSV use the global formatter's array summary.
- tabs[].blocks and childTabs keep API order; outline lists headings with tab IDs and JSON Pointer paths.
- Paragraph text concatenates text runs only. elements retain styles, links, suggestion IDs and reference markers.
- Tables contain rows[].cells[].blocks recursively. Headers, footers and footnotes have separate blocks.
- figures contain image/drawing metadata, including alt text and URIs when returned; images are never downloaded.
- Unknown blocks, inline elements and tab types retain type=unknown markers and raw data.
- startIndex/endIndex are the API's UTF-16 offsets, scoped to each tab/segment; never offsets into extracted text.
- revisionId and suggestionsViewMode are retained when returned. Missing revisionId is not synthesized.
- source=legacyBody indicates a fallback response without populated tabs; all-tab coverage cannot be confirmed.
- This is a content view, not a layout renderer or lossless API round trip. Inherited styles are not resolved.
- Suggestions remain inline, including proposed deletions; this helper does not accept or reject suggestions.
- --include-comments requests comment threads and resolves each comment anchor to referenced text.
- Use raw documents get for unsupported views or field masks. Missing body content produces an error.
- --dry-run validates and prints a request plan without acquiring credentials or fetching document content.
- --sanitize uses the existing Model Armor policy before normalization and retains _sanitization metadata.

## See Also

- [gws-shared](../gws-shared/SKILL.md) — Global flags and auth
- [gws-docs](../gws-docs/SKILL.md) — All read and write google docs commands
