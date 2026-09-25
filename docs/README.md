---
title: Specifications
summary: Conventions for documenting feature intent, requirements, and acceptance scenarios.
tags: [specification, requirements, acceptance, ears, documentation]
---

## Purpose

This document defines how this repo records feature intent, behavioral
requirements, and acceptance scenarios.

It defines the documentation protocol, not the behavior of a specific feature.

### Example

A minimal, illustrative application of this convention:

- [Example intent](./example-intent.md)
- [Example area specification](./example/example.spec.md)
- [Example area acceptance](./example/example.acceptance.md)

## Docs layout

Documentation lives with the feature it describes. Never in root /docs

```text
<feature>/
└── docs/
    ├── <feature>-intent.md
    └── <area>/
        ├── <area>.spec.md
        └── <area>.acceptance.md
```

Documentation does not need to map one-to-one to source files, platform files,
or tests. Create a document when a domain behavior needs a durable contract.

## Docs Identifiers

Identifiers use an uppercase feature area, followed by a type and a four-digit
sequence number.

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

Every document links to the documents adjacent to it, so the full set forms a
navigable graph instead of files nobody points into. This lets graph-based
retrieval (graph RAG) walk from any document to related context without a
full-text search.

- A **Readme** links forward to its **Intent** and to each **Specification**.
- An **Intent** links back to its **Readme** and forward to every
  **Specification** it introduces.
- A **Specification** links back to its **Intent** and forward to its matching
  **Acceptance** scenarios.
- An **Acceptance** document links back to the **Specification** requirement it
  verifies.

No document is reachable only by knowing its file path.

## Docs types

### Readme (*/README.md)

Every module/feature folder has a `README.md` that orients a reader before they
open its `docs/`. Unlike the root `README.md`, a module `README.md` requires
[markdown front-matter](../BUILDING.md#markdown-front-matter)

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

## Documentation

- [Intent](./docs/<feature>-intent.md): purpose, boundaries, and evolution of
  this module.
- [<Area>](./docs/<area>/<area>.spec.md): <area> requirements and acceptance
  scenarios.
```

### Intent (*.intent.md)

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

<What the feature must make possible or guarantee.>

## Boundaries

<What this feature does not own; where responsibility moves elsewhere.>

## Documentation

- [Readme](../README.md): module overview.
- [<Area>](./<area>/<area>.spec.md): <area> Specification.
```

### Specification (*.spec.md)

A specification defines normative, traceable behavioral requirements. Each
requirement has a stable identifier, a name, statuses, and links to its
acceptance scenarios.

**Status:**

Each requirement declares two independent statuses:

- **Doc status:** `Draft`, `Accepted`, or `Superseded`.
- **Implementation status:** `Not implemented`, `Partially implemented`,
  `Implemented` or `Unknown`

**Language:**

Specifications use EARS notation and the normative terms defined by RFC 2119 and
RFC 8174.

- **SHALL** and **SHALL NOT** define mandatory behavior.
- **SHOULD** and **SHOULD NOT** define expected behavior that requires an
  explicit justification to deviate from.
- **MAY** defines optional behavior.

Use the EARS pattern that matches the behavior:

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

- [<FEATURE>_<AREA>-ACC-0001](./<area>.acceptance.md#<feature>_<area>-acc-0001)
- [Intent](../<feature>-intent.md)

**Status**:

- Doc status: Draft
- Implementation status: Not implemented

```text
**WHEN:** <event>,

**THE SOFTWARE SHALL:** <behavior>.
```
````

### Acceptance (*.acceptance.md)

An acceptance document defines observable scenarios derived from requirements.
Scenarios use embedded Gherkin and link back to the requirement they verify.

Each acceptance scenario declares its own acceptance and verification status.
Implementation status belongs to the requirement, not to each scenario.

A requirement may have multiple acceptance scenarios. An acceptance scenario
belongs to one primary requirement.

**Status:**

Each acceptance declares two independent statuses:

- **Doc status:** `Draft`, `Accepted`, or `Superseded`.
- **Verification status:** `Not Meets`, `Partially Meets`, `Meets` or `Unknown`

**Language:**

Acceptance scenarios use Gherkin. A `Feature` groups scenarios for one area, a
`Rule` scopes scenarios to the requirement they verify, and a `Scenario` states
one observable behavior as `Given`/`When`/`Then` steps.

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

- [<FEATURE>_<AREA>-REQ-0001](./<area>.spec.md#<feature>_<area>-req-0001)

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
