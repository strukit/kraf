---
title: Specification and documentation conventions
summary: Conventions for documenting feature intent, requirements, and acceptance scenarios.
tags: [specification, requirements, acceptance, ears, documentation]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
>
> This document MUST use these terms, in uppercase, when expressing normative
> requirements, permissions, recommendations, or prohibitions. It MUST NOT use
> alternative words or lowercase variants as normative keywords.

## Summary

This document defines how this repo records feature intent, behavioral
requirements, and acceptance scenarios.

It defines the documentation protocol, not the behavior of a specific feature.
These type-specific rules MUST be applied in addition to the repo-wide
documentation standards in [BUILDING.md](../BUILDING.md); they do not waive
those standards unless an exception is stated explicitly.

### Example

A minimal, illustrative application of this convention:

- [Example intent](./example-intent.md)
- [Example area specification](./example/example.spec.md)
- [Example area acceptance](./example/example.acceptance.md)

## Docs layout

Documentation MUST live with the feature it describes; it MUST NOT be in root
`/docs`.

> The feature-doc layout MUST NOT be applied to the [illustrative files in the
> root `docs/` example set](#example). They demonstrate the documentation
> conventions; they are not feature documentation and do not need a feature
> folder with a nested `docs/` directory. Repo-wide standards in
> [BUILDING.md](../BUILDING.md) still apply unless they state an exception.

```text
<feature>/
└── docs/
    ├── <feature>-intent.md
    ├── <feature>-adr/
    │   └── <feature>-adr-0001-<slug>.md
    └── <feature>-<area>/
        ├── <feature>-<area>.spec.md
        └── <feature>-<area>.acceptance.md
```

Documentation MAY map one-to-one to source files, platform files, or tests. A
document SHOULD be created when a domain behavior needs a durable contract.

## Docs Identifiers

Identifiers MUST use an uppercase feature area, followed by a type and a
four-digit sequence number.

```text
<FEATURE>_<AREA>-REQ-0001
<FEATURE>_<AREA>-ACC-0001
```

For example:

- Feature: ENV-VAR
- Area: Declarations

```text
ENV-VAR_DECLARATIONS-REQ-0001
ENV-VAR_DECLARATIONS-ACC-0001
```

## Docs graph

Every document MUST link to the documents adjacent to it, so the full set forms
a navigable graph instead of files nobody points into. This lets graph-based
retrieval (graph RAG) walk from any document to related context without a
full-text search.

- A **Readme** MUST link forward to its **Intent** and to each
  **Specification**.
- An **Intent** MUST link back to its **Readme** and forward to every
  **Specification** it introduces and every **ADR** that shapes it.
- A **Specification** MUST link back to its **Intent** and forward to its
  matching **Acceptance** scenarios. A requirement MAY also link to the **ADR**
  that decided its behavior.
- An **Acceptance** document MUST link back to the **Specification** requirement
  it verifies.
- An **ADR** MUST link back to its **Intent** in its `Links` section. If it
  supersedes another **ADR**, it MUST link to that ADR in the same section.
  If it is superseded, its `Doc status` MUST link to the superseding **ADR**.
  An ADR MUST NOT link to requirements — so an accepted ADR remains unchanged
  when requirements change.

No document is reachable only by knowing its file path.

Repo-level documents that provide closing navigation links MUST group them
under a `## Related docs` heading instead of writing them as a closing sentence.
Feature documents MUST use the relationship sections defined by their type.

## Docs types

### Readme (*/README.md)

Every module/feature folder MUST have a `README.md` that orients a reader before
they open its `docs/`. Unlike the root `README.md`, a module `README.md` MUST
have [markdown front-matter](../BUILDING.md#markdown-front-matter).

**Layout**:

```md
---
title: <Feature name>
summary: <One-line summary of what the module does.>
tags: [<tag1>, <tag2>, ...]
---

<One or two paragraphs: what the module does and the concepts it introduces.>

> **Note:** this document describes <feature>'s model, including concepts and
> sources planned for its evolution. It does not imply that all of these
> capabilities are already implemented.

## Overview

<Free-form, detailed description of the module: its model, concepts, and how
they relate. May use subsections (###).>

## Documentation

- [Intent](./docs/<feature>-intent.md): purpose, boundaries, and evolution of
  this module.
- [<Area>](./docs/<feature>-<area>/<feature>-<area>.spec.md): <area>
  requirements and acceptance scenarios.
```

### Intent (*-intent.md)

An intent explains why a feature exists, the problem it addresses, its
boundaries, and its non-goals. It is not normative.

**Layout**:

```md
---
title: <Feature name> intent
summary: <One-line summary of why the feature exists.>
tags: [<tag1>, <tag2>, ...]
---

## Problem

<What is difficult, inconsistent, or unsafe without this feature.>

## Intent

<What the feature is intended to make possible or guarantee.>

## Boundaries

<What this feature does not own; where responsibility moves elsewhere.>

## Documentation

- [Readme](../README.md): module overview.
- [<Area>](./<feature>-<area>/<feature>-<area>.spec.md): <area> Specification.
- [<FEATURE>-ADR-0001](./<feature>-adr/<feature>-adr-0001-<slug>.md): <decision
  title>.
```

### Specification (*.spec.md)

A specification defines normative, traceable behavioral requirements. Each
requirement MUST have a stable identifier, a name, statuses, and links to its
acceptance scenarios.

**Status:**

Each requirement MUST declare two independent statuses:

- **Doc status:** `Draft`, `Accepted`, or `Superseded`.
- **Implementation status:** `Not implemented`, `Partially implemented`,
  `Implemented` or `Unknown`

**Language:**

Specifications MUST use EARS notation and the normative terms defined by RFC
2119 and RFC 8174.

- **SHALL** and **SHALL NOT** define mandatory behavior.
- **SHOULD** and **SHOULD NOT** define expected behavior that requires an
  explicit justification to deviate from.
- **MAY** defines optional behavior.

A requirement MUST use the EARS pattern that matches the behavior:

```text
The software SHALL <behavior>.

WHEN <event>,
THE SOFTWARE SHALL <behavior>.

IF <condition>,
THEN THE SOFTWARE SHALL <behavior>.
```

**Layout:**

````md
---
title: <Area> - SPEC
summary: <One-line summary of the requirements.>
tags: [<feature>, <area>, specification, spec]
---

> [!IMPORTANT]
> Requirements in this specification use EARS notation. **SHALL** and **SHALL
> NOT** define mandatory behavior; **SHOULD** and **SHOULD NOT** define expected
> behavior that requires an explicit justification to deviate from; **MAY**
> defines optional behavior.
>
> These terms follow RFC 2119 and RFC 8174.

## Requirements

### <FEATURE>_<AREA>-REQ-0001

Name: <Requirement name>

**Links:**

- [<FEATURE>_<AREA>-ACC-0001](./<feature>-<area>.acceptance.md#<feature>_<area>-acc-0001)
- [Intent](../<feature>-intent.md)

**Status**:

- Doc status: Draft
- Implementation status: Not implemented

```text
WHEN <event>,
THE SOFTWARE SHALL <behavior>.
```
````

### Acceptance (*.acceptance.md)

An acceptance document defines observable scenarios derived from requirements.
Scenarios MUST use embedded Gherkin and MUST link back to the requirement they
verify.

Each acceptance scenario MUST declare its own acceptance and verification
status. Implementation status belongs to the requirement, not to each scenario.

A requirement MAY have multiple acceptance scenarios. An acceptance scenario
MUST belong to one primary requirement.

**Status:**

Each acceptance MUST declare two independent statuses:

- **Doc status:** `Draft`, `Accepted`, or `Superseded`.
- **Verification status:** `Not Meets`, `Partially Meets`, `Meets` or `Unknown`

**Language:**

Acceptance scenarios MUST use Gherkin. A `Feature` groups scenarios for one
area, a `Rule` scopes scenarios to the requirement they verify, and a `Scenario`
states one observable behavior as `Given`/`When`/`Then` steps.

```gherkin
Feature: <name>

  Rule: <requirement name>

    Scenario: <scenario name>
      Given <precondition>
      When <action>
      Then <expected outcome>
```

**Layout:**

````md
---
title: <Area> - Acceptance
summary: <One-line summary of the acceptance scenarios.>
tags: [<feature>, <area>, acceptance, gherkin]
---

## <FEATURE>_<AREA>-ACC-0001

Name: <Scenario name>

**Links:**

- [<FEATURE>_<AREA>-REQ-0001](./<feature>-<area>.spec.md#<feature>_<area>-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: <FEATURE>_<AREA>-FEAT-0001 — <Feature description>

  Rule: <FEATURE>_<AREA>-REQ-0001 — <Requirement name>

    Scenario: <FEATURE>_<AREA>-ACC-0001 — <Scenario name>
      Given <precondition>
      When <action>
      Then <expected outcome>
```
````

### ADR (*-adr-*.md)

An Architecture Decision Record (ADR) records one architectural decision: the
context that required it, what was decided, the alternatives considered, and its
consequences. It explains why the module is built the way it is.

ADRs MUST live inside the module that owns the decision; they MUST NOT be in
root `/docs`. A decision that affects several modules lives in the module that
owns it, and the other modules MUST link to it.

**Identifier:**

ADRs MUST be numbered per feature, with no area.

```text
<FEATURE>-ADR-0001
```

**Status:**

Each ADR MUST declare one status:

- **Doc status:** `Proposed`, `Accepted`, `Deprecated`, or `Superseded by
  <FEATURE>-ADR-XXXX`.

An `Accepted` ADR's content is immutable, except for its status. To change a
decision, a new ADR MUST supersede it. Only the old ADR's status is updated,
and it MUST link to the superseding ADR.

For example:

```md
- Doc status: Superseded by [<FEATURE>-ADR-0002](./<feature>-adr-0002-<slug>.md).
```

**Layout:**

```md
---
title: <FEATURE>-ADR-0001 - <Decision title>
summary: <One-line summary of the decision.>
tags: [<feature>, adr, decision]
---

## <FEATURE>-ADR-0001

Name: <Decision title>

**Links:**

- [Intent](../<feature>-intent.md)

**Status**:

- Doc status: Proposed

## Context

<The problem and the forces that require a decision.>

## Decision

<What was decided, stated plainly.>

## Alternatives

- <Alternative>: <why it was not chosen>.

## Consequences

<What becomes easier, harder, or required because of this decision.>
```

## Related docs

- [README.md](../README.md): what Kraf is.
- [BUILDING.md](../BUILDING.md): repo conventions.
