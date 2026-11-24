---
description: Standards for documenting module intent, requirements, and acceptance scenarios.
tags: [specification, requirements, acceptance, ears, documentation]
type: ["standard"]
scope: ["module:*", "type:intent", "type:spec", "type:acceptance"]
language: en
---

# Specification Standards

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

This document defines how this repo records module intent, behavioral
requirements, and acceptance scenarios. It defines the documentation protocol,
not the behavior of a specific module. It defines the product-level
documentation of a module, not its architecture.

### Rules

- The rules of this document MUST be applied in addition to the repo-wide
  documentation standards in [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md), and
  MUST NOT waive those standards unless a rule of this document states the
  exception explicitly.

## Docs layout

This section defines where module documents live. The tree of
[Docs layout](#docs-layout) shows the layout of a module folder.

```text
<module>/
├── README.md
└── docs/
    ├── <module>-intent.md
    └── specs/
        └── <module>-<feature>/
            ├── <module>-<feature>.spec.md
            └── <module>-<feature>.acceptance.md
```

### Rules

- Documentation MUST live with the module it describes, and MUST NOT be in
  `<repo>/docs`.
- The name of a module MUST use `_` and MUST NOT use `-` (e.g. `env_var`, not
  `env-var`).
- A module folder MUST be named exactly as its module (e.g. `env_var/` for the
  module `env_var`).
- Every module folder MUST have a `README.md` that orients a reader before they
  open its `<module>/docs/`, and that declares the module `<module>` by listing
  `module:<module>` in its `type`.
- The intent of a module MUST be `<module>/docs/<module>-intent.md`, directly in
  `<module>/docs/`, and MUST NOT be inside a feature folder.
- The specification and the acceptance document of a feature MUST be in the
  `<module>/docs/specs/<module>-<feature>/` folder of that feature, and MUST NOT
  be directly in `<module>/docs/`.
- Documentation MAY map one-to-one to source files, platform files, or tests.
- A document SHOULD be created when a domain behavior needs a durable contract.

## Docs identifiers

This section defines the identifiers of requirements and acceptance scenarios.

```text
<MODULE>-<FEATURE>-REQ-0001
<MODULE>-<FEATURE>-ACC-0001
```

### Terms

- **Identifier part**: the name of a module or of a feature, in uppercase, with
  each `-` of the name of a feature replaced by `_`, written `<MODULE>` or
  `<FEATURE>`.

### Rules

- Identifiers MUST be formed of the identifier part of the module, the
  identifier part of the feature, a type, and a four-digit sequence number,
  separated by `-`, as the format of [Docs identifiers](#docs-identifiers)
  shows.
- An identifier MUST use `-` only to separate its parts, and MUST use `_` in
  place of each `-` of the name of a feature (e.g. `MY_FEATURE` for the feature
  `my-feature`).
- The type of an identifier MUST be `REQ` for a requirement, `ACC` for an
  acceptance scenario, or `FEAT` for a Gherkin `Feature`.
- Every part of an identifier MUST be written in uppercase wherever the
  identifier appears, including headings, plain text, link text, link anchors,
  Gherkin lines, code, and citations (e.g. `ENV_VAR-HOST-REQ-0004` and
  `#ENV_VAR-HOST-REQ-0004`).
- The identifier of a Gherkin `Feature` MUST use the same module and feature as
  its requirements, with the type `FEAT` and a four-digit sequence number, as
  `<MODULE>-<FEATURE>-FEAT-0001` shows. A `Feature` identifies the feature it
  describes, and it is not a name for the module or the feature.
- A citation of a requirement or an acceptance scenario, in a document, code, a
  commit message, a pull request, a comment, or a message to the user, MUST use
  its full identifier, with the module and feature (e.g.
  `ENV_VAR-HOST-REQ-0004`, not `REQ-0004`).

### Examples

- Module: env_var
- Feature: declarations

```text
ENV_VAR-DECLARATIONS-REQ-0001
ENV_VAR-DECLARATIONS-ACC-0001
```

## Docs graph

This section defines the links between module documents, so the full set forms a
navigable graph instead of files nobody points into. This lets graph-based
retrieval (graph RAG) walk from any document to related context without a
full-text search.

### Rules

- Every document MUST link to the documents adjacent to it.
- A document MUST NOT be reachable only by knowing its file path.
- An **Intent** MUST link back to the document that links to it and forward to
  every **Specification** it introduces.
- A **Specification** MUST link back to its **Intent** and forward to its
  matching **Acceptance** scenarios.
- An **Acceptance** document MUST link back to the **Specification** requirement
  it verifies.
- Module documents MUST use the relationship sections defined by their type.
- Repo-level documents MUST follow the
  [Link Navigation](./GENERAL_STANDARDS.md#link-navigation) rules.

## Docs types

This section defines each type of module document, its rules, and its layout.

### Rules

- The layout of each type in this section MUST override, for the documents of
  that type, the rule of
  [Document Layout](./GENERAL_STANDARDS.md#document-layout) that every document
  MUST follow its template.

### Intent (*-intent.md)

An intent explains why a module exists, the problem it addresses, its
boundaries, and its non-goals. An intent is an idea that still needs refinement.

#### Rules

- An intent MUST NOT be normative: implementing, specifying, or accepting what
  it describes is OPTIONAL, now or later.
- A principle of an intent MUST become normative only through a requirement that
  covers it.
- An intent MUST NOT carry the normative-language note and MUST NOT use
  normative keywords, and this rule MUST override, only for an intent, the rule
  of [Normative-Language Note](./GENERAL_STANDARDS.md#normative-language-note)
  that every document MUST carry the note.
- An intent, or any part of one, MAY lack a requirement, code, or any other
  document that specifies or details it, and that absence MUST NOT be treated as
  an error, a defect, or a problem.

#### Intent layout

```md
---
description: <One-line summary of why the module exists.>
tags: [<tag1>, <tag2>, ...]
type: ["intent"]
scope: ["module:<module>"]
language: <language-code>
---

## Problem

<What is difficult, inconsistent, or unsafe without this module.>

## Intent

<What the module is intended to make possible or guarantee.>

## Boundaries

<What this module does not own; where responsibility moves elsewhere.>

## Links

- [README.md](path-to-nearest-index): <description of the nearest index>.
- [<Feature>](./specs/<module>-<feature>/<module>-<feature>.spec.md): <feature>
  Specification.
```

### Specification (*.spec.md)

A specification defines normative, traceable behavioral requirements.

#### Rules

- Each requirement MUST have a stable identifier, a name, statuses, and links to
  its acceptance scenarios.
- Each requirement MUST declare two independent statuses:
  - **Doc status:** `Draft`, `Accepted`, or `Superseded`.
  - **Implementation status:** `Not Implemented`, `Partially Implemented`,
    `Implemented`, or `Unknown`.

#### Specification language

The rules of this section summarize the EARS sources. An EARS requirement is
made of, in this order: zero or more preconditions, zero or one trigger, the
system name, and one or more system responses.

Sources:

- [EARS](https://en.wikipedia.org/w/index.php?title=Easy_Approach_to_Requirements_Syntax&oldid=1342972975)
  (Wikipedia, revision of 2026-03-11).

##### Rules

- An agent MAY consult the sources lazily: only when the rules of
  [Specification language](#specification-language) leave a doubt about EARS
  unresolved.
- Specifications MUST use EARS notation and the normative terms defined by RFC
  2119 and RFC 8174.
- A requirement MUST use **SHALL** or **SHALL NOT** for mandatory behavior,
  **SHOULD** or **SHOULD NOT** for expected behavior that requires an explicit
  justification to deviate from, and **MAY** for optional behavior.
- A requirement MUST use `THE SOFTWARE` as the system name.
- A requirement MUST use the EARS pattern of [Patterns](#patterns) that matches
  the behavior.

##### Patterns

| Pattern           | Keyword         | Use when                                                                                  |
| ----------------- | --------------- | ----------------------------------------------------------------------------------------- |
| Ubiquitous        | (none)          | The behavior is always active.                                                            |
| Event-driven      | `WHEN`          | The behavior responds to a triggering event.                                              |
| State-driven      | `WHILE`         | The behavior is active while a precondition holds.                                        |
| Optional feature  | `WHERE`         | The behavior applies only where a feature is included.                                    |
| Unwanted behavior | `IF`/`THEN`     | The behavior responds to an undesired situation, such as a fault, a failure, or an error. |
| Complex           | Any combination | The behavior uses more than one EARS keyword.                                             |

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

#### Specification layout

````md
---
description: <One-line summary of the requirements.>
tags: [<module>, <feature>, specification, spec]
type: ["spec"]
scope: ["module:<module>"]
language: <language-code>
---

> [!NOTE]
> The keywords follow RFC 2119 and RFC 8174 across three levels of force:
>
> - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
> - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
> - **Optional:** MAY, OPTIONAL
>
> See
> [GENERAL_STANDARDS.md](<path-to-docs>/GENERAL_STANDARDS.md#normative-language).

</br>

> [!IMPORTANT]
> Requirements in this specification use EARS notation. **SHALL** and **SHALL
> NOT** define mandatory behavior; **SHOULD** and **SHOULD NOT** define expected
> behavior that requires an explicit justification to deviate from; **MAY**
> defines optional behavior.
>
> These terms follow RFC 2119 and RFC 8174.

## Requirements

### <MODULE>-<FEATURE>-REQ-0001

Name: <Requirement name>

**Links:**

- [<MODULE>-<FEATURE>-ACC-0001](./<module>-<feature>.acceptance.md#<MODULE>-<FEATURE>-ACC-0001)
- [Intent](../../<module>-intent.md)

**Status**:

- Doc status: Draft
- Implementation status: Not Implemented

```text
WHEN <trigger>,
THE SOFTWARE SHALL <system response>.
```

## Links

- [Intent](../../<module>-intent.md): <description of the intent>.
- [README.md](path-to-nearest-index): <description of the nearest index>.
````

### Acceptance (*.acceptance.md)

An acceptance document defines observable scenarios derived from requirements.

#### Rules

- Scenarios MUST use embedded Gherkin and MUST link back to the requirement they
  verify.
- Each acceptance scenario MUST declare two independent statuses of its own:
  - **Doc status:** `Draft`, `Accepted`, or `Superseded`.
  - **Verification status:** `Not Meets`, `Partially Meets`, `Meets`, or
    `Unknown`.
- An acceptance scenario MUST have the verification status `Meets` only when an
  automated check runs its `When` steps and asserts each `Then` step on the
  value that the software produces.
- A check that only validates the format of the data MUST give the verification
  status `Partially Meets`.
- The implementation status MUST be declared by the requirement, and MUST NOT be
  declared by each scenario.
- A requirement MAY have multiple acceptance scenarios.
- An acceptance scenario MUST belong to one primary requirement.

#### Acceptance language

The rules of this section summarize the Gherkin sources.

Sources:

- [Gherkin](https://github.com/cucumber/website/blob/9bca7375458301f81aaa2f7857587dd141688fab/docs/gherkin/reference.md)
  (Cucumber, commit `9bca737` of 2025-01-26).

```gherkin
Feature: <name>

  Rule: <requirement name>

    Scenario: <scenario name>
      Given <precondition>
      When <action>
      Then <expected outcome>
```

##### Rules

- An agent MAY consult the sources lazily: only when the rules of
  [Acceptance language](#acceptance-language) leave a doubt about Gherkin
  unresolved.
- Acceptance scenarios MUST use Gherkin.
- A `Feature` MUST group the scenarios of one feature, a `Rule` MUST scope
  scenarios to the requirement they verify, and a `Scenario` MUST state one
  observable behavior as `Given`/`When`/`Then` steps.
- The word `Feature` in a Gherkin document MUST be read as the Gherkin keyword,
  and MUST NOT be read as the term `feature` of this document.

#### Acceptance layout

````md
---
description: <One-line summary of the acceptance scenarios.>
tags: [<module>, <feature>, acceptance, gherkin]
type: ["acceptance"]
scope: ["module:<module>"]
language: <language-code>
---

> [!NOTE]
> The keywords follow RFC 2119 and RFC 8174 across three levels of force:
>
> - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
> - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
> - **Optional:** MAY, OPTIONAL
>
> See
> [GENERAL_STANDARDS.md](<path-to-docs>/GENERAL_STANDARDS.md#normative-language).

## <MODULE>-<FEATURE>-ACC-0001

Name: <Scenario name>

**Links:**

- [<MODULE>-<FEATURE>-REQ-0001](./<module>-<feature>.spec.md#<MODULE>-<FEATURE>-REQ-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: <MODULE>-<FEATURE>-FEAT-0001 — <Feature description>

  Rule: <MODULE>-<FEATURE>-REQ-0001 — <Requirement name>

    Scenario: <MODULE>-<FEATURE>-ACC-0001 — <Scenario name>
      Given <precondition>
      When <action>
      Then <expected outcome>
```

## Links

- [Specification](./<module>-<feature>.spec.md):
  <description of the specification>.
- [README.md](path-to-nearest-index): <description of the nearest index>.
````

## Review

This section defines how a review treats specifications and their acceptance
scenarios.

### Rules

- A review MUST read the acceptance files linked from a specification before
  reporting one of its requirements as incorrect.
- When a change removes a requirement or alters the behavior that a requirement
  defines, the review MUST compare the old and new requirement's scope,
  including its conditions and triggers, and MUST identify the cases covered by
  one wording but not the other.
- The review MUST evaluate those cases against the linked acceptance scenarios
  and the code that implements the requirement.
- A review MUST report, as a correctness defect, a change that leaves existing
  code behavior without a requirement that defines it.

## Links

- [README.md](./README.md): Index of the standards that govern this repository,
  to help people and agents navigate it on their own through the document graph.
