---
description: Standards for architecture decision records, their location, identifiers, status, immutability, and layout.
tags: [adr, decision, architecture, documentation]
type: ["standard"]
scope: ["type:adr"]
language: en
---

# ADR Standards

> [!NOTE]
> The keywords follow RFC 2119 and RFC 8174 across three levels of force:
>
> - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
> - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
> - **Optional:** MAY, OPTIONAL
>
> See [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#normative-language).

</br>

> [!IMPORTANT]
> This is a standard document:
>
> - Works like a constitution
> - Rules and Instructions MUST be mandatory and normative
> - Rules and Instructions MUST NOT be ignored
>
> See [DOC_STANDARDS.md](./DOC_STANDARDS.md#standard-documents).

## Overview

This document defines how this repository records architecture decisions: where
an ADR lives, how it is identified, which statuses it has, how a decision
changes, and how an ADR is laid out. It defines the protocol for recording a
decision, not any decision.

### Terms

- **ADR**: an architecture decision record, a document that records one
  decision, the context that required it, the alternatives considered, and its
  consequences.
- **Architecture ADR**: an ADR whose decision crosses module boundaries and that
  no single module owns.
- **Module ADR**: an ADR whose decision a single module owns.

### Rules

- The rules of this document MUST be applied in addition to
  [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md), and MUST NOT waive those
  standards.

## Location

This section defines where an ADR lives.

### Rules

- An architecture ADR MUST live in `<repo>/docs/adr/`, next to
  [ARCHITECTURE.md](../ARCHITECTURE.md).
- A module ADR MUST live in `<module>/docs/<module>-adr/`, where `<module>` is
  the module that owns the decision.
- A decision that affects several modules and that one of them owns MUST be a
  module ADR of that module, and the other modules MUST link to it.
- A decision that affects several modules and that none of them owns MUST be an
  architecture ADR.
- Each folder of ADRs MUST have a `README.md` index that lists its ADRs.

## Identifiers

This section defines the identifier and the file name of an ADR.

```text
ARCH-ADR-0001                          <repo>/docs/adr/arch-adr-0001-<slug>.md
<MODULE>-ADR-0001       <module>/docs/<module>-adr/<module>-adr-0001-<slug>.md
```

### Rules

- An architecture ADR MUST use the prefix `ARCH`, and a module ADR MUST use its
  module name in uppercase, followed by `-ADR-` and a four-digit sequence
  number, as [Identifiers](#identifiers) shows.
- The file name MUST be the identifier in lowercase, followed by a short
  kebab-case slug of the decision.
- A sequence number MUST NOT be reused, even after its ADR is superseded or
  deprecated.

## Status

This section defines the status of an ADR and how a decision changes.

### Terms

- **Doc status**: one of `Accepted`, `Deprecated`, or `Superseded by <ID>`.

### Rules

- Each ADR MUST declare one doc status and the date the status was set.
- An ADR MUST be `Accepted` in the pull request that adds it to the repository,
  because the approval of that pull request is the approval of its decision.
- A decision that is not yet approved MUST be discussed outside the repository
  (e.g. in an issue or a discussion), and MUST NOT be added as an ADR.
- The content of an ADR MUST NOT change, except its doc status, its date, and
  its Links section.
- A change to an accepted decision MUST be recorded in a new ADR that supersedes
  it, and the old ADR's status MUST change to `Superseded by <ID>`, linking to
  the new ADR.
- A new ADR that supersedes another MUST link to it in its Links section.

## Links between documents

This section defines how an ADR links into the document graph.

### Rules

- An ADR MUST be tree-linked with its index, as
  [Link Navigation](./GENERAL_STANDARDS.md#link-navigation) requires.
- An architecture ADR MUST be listed in
  [the Decisions section of ARCHITECTURE.md](../ARCHITECTURE.md#decisions), and
  a module ADR MUST be linked from its module's `README.md` or intent.
- An ADR MUST NOT link to a requirement, so that an accepted ADR stays unchanged
  when requirements change.
- A requirement MAY link to the ADR that decided its behavior.

## Layout

This section defines the layout of an ADR.

```md
---
description: <One-line summary of the decision.>
tags: [adr, decision, <tag>, ...]
type: ["adr"]
scope: [<type:name | module:name | glob>, ...]
language: <language-code>
---

# <ID> - <Decision title>

<normative-language note>

## Overview

<One paragraph that states the decision.>

**Status**:

- Doc status: <doc status>
- Date: <YYYY-MM-DD>

## Context

<The problem and the forces that required a decision.>

## Decision

<What was decided, stated plainly.>

## Alternatives

- <Alternative>: <why it was not chosen>.
- ...

## Consequences

<What becomes easier, harder, or required because of the decision.>

## Links

<links>
```

### Rules

- Every ADR MUST follow the template of [Layout](#layout).
- The Context, Decision, Alternatives, and Consequences sections MUST describe
  the decision as it was made, and MUST NOT be rewritten later to describe a
  different one.

## Links

- [README.md](./README.md): Index of the standards that govern this repository,
  to help people and agents navigate it on their own through the document graph.
- [ARCHITECTURE.md](../ARCHITECTURE.md): Cross-domain map of Kraf's system
  context, modules, and how they depend on each other.
- [SPEC_STANDARDS.md](./SPEC_STANDARDS.md): Standards for documenting module
  intent, requirements, and acceptance scenarios.
