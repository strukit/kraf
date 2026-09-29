---
name: docs
description: Create, edit, and review any Markdown doc in this repo against the conventions in BUILDING.md and /docs/README.md. Use whenever a Markdown doc is created, edited, or reviewed anywhere in the repo.
user-invocable: true
---

You work on documentation against two sources of truth:

- [`BUILDING.md`](../../../BUILDING.md) — the conventions that apply to every
  doc in the repo.
- [`docs/README.md`](../../../docs/README.md) — the conventions for a module's
  feature docs.

Do not rely on memory and do not restate their rules here — they change, and a
copy would drift out of sync. Every run, re-read the relevant source in full and
apply it directly. The phases below say only what to do; the rules are theirs.

## On create

Re-read [`BUILDING.md`](../../../BUILDING.md) (and, for a feature doc,
[`docs/README.md`](../../../docs/README.md)) and make the new doc satisfy
everything they define.

## On edit

Re-read the same source(s) and keep the doc satisfying them after your change —
nothing you add or remove may break what they require.

## On review

Re-read the source(s) in full, then check the doc(s) under review against every
rule and template they define — a module's `README.md` and `docs/`, or the
illustrative set under `/docs` if none is given. Follow every relative link and
confirm the target, and any identifier anchor, exists. If a source disagrees
with itself, report that as its own finding.

## What NOT to do

- Don't flag files under a root `/docs` folder when that's the illustrative `##
  Example` set referenced by [`docs/README.md`](../../../docs/README.md) — an
  intentional exception.
- Don't redesign the conventions while reviewing; if one has a gap, note it as
  an observation, separate from conformance findings.
- On review, don't touch files — it's a review, not an edit pass.

## Output

For a review: a short list of findings (file, line if any, what's wrong, what
the source says instead). If all checks out, say so — don't invent findings.
