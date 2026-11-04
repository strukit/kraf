---
title: Specification and documentation conventions
summary: Conventions for documenting feature intent, requirements, and acceptance scenarios.
tags: [specification, requirements, acceptance, ears, documentation]
applies_to: [spec]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md#normative-language). This
> document MUST NOT use alternative words or lowercase variants as normative
> keywords.

---

> [!IMPORTANT]
> This is a convention document, as defined in
> [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md#convention-documents): its
> rules are mandatory and non-negotiable. Treat them like a constitution: a rule
> changes only through a pull request that edits this document and MUST NOT be
> bypassed by an ad-hoc exception in code.

## Summary

This document defines how this repo records feature intent, behavioral
requirements, and acceptance scenarios.

It defines the documentation protocol, not the behavior of a specific feature.
These type-specific rules MUST be applied in addition to the repo-wide
documentation standards in [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md);
they do not waive those standards unless an exception is stated explicitly.

### Example

A minimal, illustrative application of this convention:

- [INTENT](./example/example-intent.md)
- [SPECIFICATION](./example/example.spec.md)
- [ACCEPTANCE](./example/example.acceptance.md)

## Docs layout

Documentation MUST live with the feature it describes; it MUST NOT be in root
`/docs`.

> The feature-doc layout MUST NOT be applied to the
> [illustrative files in the root `docs/` example set](#example). They
> demonstrate the documentation conventions; they are not feature documentation
> and do not need a feature folder with a nested `docs/` directory. Repo-wide
> standards in [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md) still apply
> unless they state an exception.

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
- An **Intent** MUST link back to the document that links to it and forward to
  every **Specification** it introduces and every **ADR** that shapes it.
- A **Specification** MUST link back to its **Intent** and forward to its
  matching **Acceptance** scenarios. A requirement MAY also link to the **ADR**
  that decided its behavior. A requirement MUST link to each other requirement
  whose behavior it depends on or constrains, and the latter MUST link back.
- An **Acceptance** document MUST link back to the **Specification** requirement
  it verifies.
- An **ADR** MUST link back to its **Intent** in its `Links` section. If it
  supersedes another **ADR**, it MUST link to that ADR in the same section. If
  it is superseded, its `Doc status` MUST link to the superseding **ADR**. An
  ADR MUST NOT link to requirements — so an accepted ADR remains unchanged when
  requirements change.

No document is reachable only by knowing its file path.

Feature documents MUST use the relationship sections defined by their type.
Repo-level documents follow the
[Docs graph](./GENERAL_CONVENTIONS.md#docs-graph) rule.

## Docs types

### Readme (*/README.md)

Every module/feature folder MUST have a `README.md` that orients a reader before
they open its `docs/`. Unlike the [root `README.md`](../README.md), a module
`README.md` MUST have
[markdown front-matter](./GENERAL_CONVENTIONS.md#markdown-front-matter).

**Layout**:

```md
---
title: <Feature name>
summary: <One-line summary of what the module does.>
tags: [<tag1>, <tag2>, ...]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_CONVENTIONS.md](<path-to-docs>/GENERAL_CONVENTIONS.md#normative-language).
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords.

<One or two paragraphs: what the module does and the concepts it introduces.>

> **Note:** this document describes <feature>'s model, including concepts and
> sources planned for its evolution. It does not imply that all of these
> capabilities are already implemented.

## Overview

<Free-form, detailed description of the module: its model, concepts, and how
they relate. MAY use subsections (###).>

## Documentation

- [Intent](./docs/<feature>-intent.md): purpose, boundaries, and evolution of
  this module.
- [<Area>](./docs/<feature>-<area>/<feature>-<area>.spec.md): <area>
  requirements and acceptance scenarios.
- [README.md](<path-to-root>/README.md): what Kraf is.
```

### Intent (*-intent.md)

An intent explains why a feature exists, the problem it addresses, its
boundaries, and its non-goals. It is not normative: implementing, specifying, or
accepting what it describes is OPTIONAL, now or later. A principle becomes
normative only through a requirement or an ADR that covers it.

An intent MUST NOT carry the normative-language note and MUST NOT use normative
keywords.

An intent is an idea that still needs refinement. An intent, or any part of one,
can lack a requirement, an ADR, code, or any other document that specifies or
details it. This is expected and MUST NOT be treated as an error, a defect, or a
problem.

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
- **Implementation status:** `Not Implemented`, `Partially Implemented`,
  `Implemented` or `Unknown`

**Language:**

Sources:

- [EARS](https://en.wikipedia.org/w/index.php?title=Easy_Approach_to_Requirements_Syntax&oldid=1342972975)
  (Wikipedia, revision of 2026-03-11).

Specifications MUST use EARS notation. **SHALL** and **SHALL NOT** define
mandatory behavior, as RFC 2119 and RFC 8174 define them.

An EARS requirement is made of, in this order: zero or more preconditions, zero
or one trigger, the system name, and one or more system responses. A
requirement MUST use `THE SOFTWARE` as the system name.

A requirement MUST use the EARS pattern that matches the behavior:

| Pattern           | Keyword         | Use when                                                                                  |
| ----------------- | --------------- | ----------------------------------------------------------------------------------------- |
| Ubiquitous        | (none)          | The behavior is always active.                                                            |
| Event-driven      | `WHEN`          | The behavior responds to a triggering event.                                              |
| State-driven      | `WHILE`         | The behavior is active while a precondition holds.                                        |
| Optional feature  | `WHERE`         | The behavior applies only where a feature is included.                                    |
| Unwanted behavior | `IF`/`THEN`     | The behavior responds to an undesired situation, such as a fault, a failure, or an error. |
| Complex           | More than one   | The behavior uses more than one EARS keyword.                                             |

```text
THE SOFTWARE SHALL <system response>.

WHEN <trigger>,
THE SOFTWARE SHALL <system response>.

WHILE <precondition>,
THE SOFTWARE SHALL <system response>.

WHERE <feature is included>,
THE SOFTWARE SHALL <system response>.

IF <trigger>,
THEN THE SOFTWARE SHALL <system response>.

WHILE <precondition>,
WHEN <trigger>,
THE SOFTWARE SHALL <system response>.
```

**Differences from the source:**

The differences below are deliberate decisions of this repo.

- **SHALL NOT and RFC terms:** the source uses only SHALL. Specifications also
  MAY use SHALL NOT for prohibitions, and cite RFC 2119 and RFC 8174 because
  [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md#normative-language) makes
  every document cite them.
- **System name:** the source leaves `<system name>` free. Specifications fix it
  as `THE SOFTWARE`, so every requirement names the system the same way.
- **EARS for every requirement:** the source notes that EARS does not suit
  every requirement, such as formulas, decision tables, or constraints that are
  not conditional behavior. Specifications use EARS for all their requirements.
- **Pattern choice:** the source describes the patterns without making the
  choice mandatory. This repo makes the matching pattern mandatory.
- **Presentation:** a line break after each keyword clause, uppercase `THE` in
  every pattern (the source uses it only in the ubiquitous template), and the
  fenced `text` block are layout choices that the source does not define.

**Layout:**

````md
---
title: <Area> - SPEC
summary: <One-line summary of the requirements.>
tags: [<feature>, <area>, specification, spec]
---

> [!IMPORTANT]
> Requirements in this specification use EARS notation. **SHALL** and **SHALL
> NOT** define mandatory behavior.
>
> These terms follow RFC 2119 and RFC 8174.

## Requirements

### <FEATURE>_<AREA>-REQ-0001

Name: <Requirement name>

**Links:**

- [<FEATURE>_<AREA>-ACC-0001](./<feature>-<area>.acceptance.md#<feature>_<area>-acc-0001)
- [Intent](../<feature>-intent.md)
- [<FEATURE>_<AREA>-REQ-0002](./<feature>-<area>.spec.md#<feature>_<area>-req-0002):
  related requirement.

**Status**:

- Doc status: Draft
- Implementation status: Not Implemented

```text
WHEN <trigger>,
THE SOFTWARE SHALL <system response>.
```
````

### Acceptance (*.acceptance.md)

An acceptance document defines observable scenarios derived from requirements.
Scenarios MUST use embedded Gherkin and MUST link back to the requirement they
verify.

Each acceptance scenario MUST declare its own doc status and verification
status. Implementation status belongs to the requirement, not to each scenario.

A requirement MAY have multiple acceptance scenarios. An acceptance scenario
MUST belong to one primary requirement.

**Status:**

Each acceptance MUST declare two independent statuses:

- **Doc status:** `Draft`, `Accepted`, or `Superseded`.
- **Verification status:** `Not Meets`, `Partially Meets`, `Meets` or `Unknown`

**Language:**

Sources:

- [Gherkin](https://github.com/cucumber/website/blob/9bca7375458301f81aaa2f7857587dd141688fab/docs/gherkin/reference.md)
  (Cucumber, commit `9bca737` of 2025-01-26).

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

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_CONVENTIONS.md](<path-to-docs>/GENERAL_CONVENTIONS.md#normative-language).
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords.

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
decision, a new ADR MUST supersede it. Only the old ADR's status is updated, and
it MUST link to the superseding ADR.

For example:

```md
- Doc status: Superseded by
  [<FEATURE>-ADR-0002](./<feature>-adr-0002-<slug>.md).
```

**Layout:**

```md
---
title: <FEATURE>-ADR-0001 - <Decision title>
summary: <One-line summary of the decision.>
tags: [<feature>, adr, decision]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_CONVENTIONS.md](<path-to-docs>/GENERAL_CONVENTIONS.md#normative-language).
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords.

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

## Review

- A review MUST read the documents linked from a requirement before reporting
  the requirement as incorrect.
- When a change alters the meaning of a requirement, a review MUST follow the
  requirement's links and evaluate the new meaning against each linked document,
  including the linked requirements and the code that implements it.
- When a change removes a requirement or alters the behavior that it defines, a
  review MUST compare the old and new scope, including conditions and triggers,
  and MUST identify the cases covered by one wording but not the other. It MUST
  evaluate those cases as the previous rule states.
- A review MUST report, as a correctness defect, a change that leaves existing
  code behavior without a requirement that defines it, and a contradiction
  between a requirement and a document linked from it.
- When a change adds or removes a requirement, a review MUST verify that its
  identifier is unique and that every link to and from it resolves.
- When a change touches a document that a requirement links to, a review MUST
  follow the link back to the requirement and verify that both still agree.
- When a change alters a rule of the EARS notation, a review MUST compare it
  with the cited source revision and MUST report what goes beyond it.
- A review MUST NOT report as a defect a difference that the *Differences from
  the source* list of the [Specification](#specification-specmd) section states.

## Related

- [README.md](../README.md): what Kraf is.
- Examples:
  - [INTENT](./example/example-intent.md)
  - [SPECIFICATION](./example/example.spec.md)
  - [ACCEPTANCE](./example/example.acceptance.md)
