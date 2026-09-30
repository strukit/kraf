---
name: docs
description: Create, edit, and review any Markdown doc in this repo against the conventions in BUILDING.md and /docs/README.md. Use whenever a Markdown doc is created, edited, or reviewed anywhere in the repo.
user-invocable: true
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
>
> This document MUST use these terms, in uppercase, when expressing normative
> requirements, permissions, recommendations, or prohibitions. It MUST NOT use
> alternative words or lowercase variants as normative keywords.

You work on documentation against two sources of truth:

- [`BUILDING.md`](../../../BUILDING.md) — the conventions that apply to every
  doc in the repo.
- [`docs/README.md`](../../../docs/README.md) — the conventions for a module's
  feature docs.

You MUST NOT rely on memory or restate their rules here — they change, and a
copy would drift out of sync. Every run, you MUST re-read the relevant source in
full and apply it directly. The phases below say only what to do; the rules are
theirs.

## On create

You MUST re-read [`BUILDING.md`](../../../BUILDING.md) (and, for a feature doc,
[`docs/README.md`](../../../docs/README.md)) and make the new doc satisfy
everything they define.

## On edit

You MUST re-read the same source(s) and keep the doc satisfying them after your
change. You MUST NOT add or remove anything that breaks what they require.

## On review

You MUST re-read the source(s) in full, then check the doc(s) under review
against every rule and template they define — a module's `README.md` and
`docs/`, or the illustrative set under `/docs` if none is given. You MUST follow
every relative link and confirm the target, and any identifier anchor, exists.
If a source disagrees with itself, you MUST report that as its own finding.

You MUST apply both sources: [`BUILDING.md`](../../../BUILDING.md) sets the
repo-wide baseline, and [`docs/README.md`](../../../docs/README.md) adds
feature-documentation rules. You MUST attribute each finding to the source that
defines the relevant rule.

## What NOT to do

- You MUST NOT flag files under a root `/docs` folder when that's the
  illustrative `## Example` set referenced by
  [`docs/README.md`](../../../docs/README.md) — an intentional exception.
- You MUST NOT redesign the conventions while reviewing; if one has a gap, note
  it as an observation, separate from conformance findings.
- During review, you MUST NOT touch files — it's a review, not an edit pass.

## Output

For a review, you MUST provide a short list of findings (file, line if any,
what's wrong, and what the source says instead). If everything conforms, you
MUST state that plainly. You MUST NOT invent findings.
