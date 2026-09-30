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

## Sources of truth

- [`BUILDING.md`](../../../BUILDING.md) — repo-wide documentation conventions.
- [`docs/README.md`](../../../docs/README.md) — additional conventions for
  feature docs and their illustrative examples.

Before any create, edit, or review, you MUST read the applicable sources in full:
the repo-wide baseline always applies; the feature-doc conventions also apply
to a module's `README.md`, its `docs/`, and the illustrative example set.
You MUST apply every relevant rule and template directly from those sources.
You MUST NOT rely on memory or duplicate their conventions in this skill.

## On create

You MUST make the new document conform to the applicable conventions.

## On edit

You MUST keep the document conforming after the change. You MUST NOT introduce
changes that violate the applicable conventions.

## On review

You MUST check the requested documents against the applicable conventions.
For a module review, you MUST include its `README.md` and `docs/`; if no module
is specified, you MUST review the illustrative example set.

You MUST inventory the Markdown files within the review scope and check that
every document is reachable by following documentation links from the relevant
entry point. You MAY use file search to build the inventory and content search
to verify links or patterns. These searches MUST NOT replace following the
documentation links to read and understand the documents.

You MUST verify every relative link's target and any referenced identifier
anchor. You MUST report contradictions within a source as separate findings.

You MUST NOT edit files or redesign conventions during review. Gaps in the
conventions MUST be reported as observations, separately from conformance
findings. You MUST respect the documented exception for the illustrative
example set's location under root `docs/`.

## Review output

You MUST provide a short list of findings, each with the file, line if
applicable, issue, and the source rule that defines the expected behavior.
If everything conforms, you MUST state that plainly. You MUST NOT invent
findings.
