---
title: Specifications
summary: Conventions for documenting feature intent, requirements, and acceptance scenarios.
tags: [specification, requirements, acceptance, ears, documentation]
---

## Purpose

This document defines how Kraf records feature intent, behavioral requirements,
and acceptance scenarios.

It defines the documentation protocol, not the behavior of a specific feature.

## Document types

### Intent

An intent explains why a feature exists, the problem it addresses, its
boundaries, and its non-goals. It is not normative.

### Specification

A specification defines normative, traceable behavioral requirements. Each
requirement has a stable identifier, a name, statuses, and links to its
acceptance scenarios.

### Acceptance

An acceptance document defines observable scenarios derived from requirements.
Scenarios use embedded Gherkin and link back to the requirement they verify.

## Traceability

```text
Intent → Requirement ↔ Acceptance
```

A requirement may have multiple acceptance scenarios. An acceptance scenario
belongs to one primary requirement.

## Feature-local layout

Documentation lives with the feature it describes.

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

## Identifiers

Identifiers use an uppercase feature area, followed by a type and a four-digit
sequence number.

```text
<FEATURE>_<AREA>-FEAT-0001
<FEATURE>_<AREA>-REQ-0001
<FEATURE>_<AREA>-ACC-0001
```

For example:

```text
ENV_VAR_DECLARATIONS-REQ-0001
ENV_VAR_DECLARATIONS-ACC-0001
```

## Normative language

Specifications use EARS notation and the normative terms defined by RFC 2119
and RFC 8174.

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

## Statuses

Each requirement declares three independent statuses:

- **Requirement status:** `Draft`, `Accepted`, or `Superseded`.
- **Implementation status:** `Not implemented`, `Partially implemented`, or
  `Implemented`.
- **Verification status:** `Not verified` or `Verified`.

Each acceptance scenario declares its own acceptance and verification status.
Implementation status belongs to the requirement, not to each scenario.

## Minimal example

An intent establishes the boundary:

```md
## Intent

The feature must make process configuration explicit rather than inheriting
undeclared host state.
```

A specification defines a requirement and links to acceptance evidence:

```md
### EXAMPLE_AREA-REQ-0001

Name: Explicit configuration

**Links:**

- [EXAMPLE_AREA-ACC-0001](./example.acceptance.md#example_area-acc-0001)

**Status**:

- Requirement status: Draft
- Implementation status: Not implemented
- Verification status: Not verified

**WHEN:** Configuration provides an unknown key,

**THE SOFTWARE SHALL:** reject the configuration.
```

Acceptance links back and states the observable behavior:

````md
## EXAMPLE_AREA-ACC-0001

Name: Unknown configuration key is rejected

**Links:**

- [EXAMPLE_AREA-REQ-0001](./example.spec.md#example_area-req-0001)

```gherkin
Feature: EXAMPLE_AREA-FEAT-0001 — Explicit configuration

  Rule: EXAMPLE_AREA-REQ-0001 — Explicit configuration

    Scenario: EXAMPLE_AREA-ACC-0001 — Unknown configuration key is rejected
      Given no declaration exists for "EXAMPLE_KEY"
      When configuration provides a value for "EXAMPLE_KEY"
      Then the software rejects the configuration
```
````

## Current reference

The environment-variable feature is the current reference implementation of
this convention:

- [Environment variable intent](./src/environment/src/env_var/docs/env-var-intent.md)
- [Environment variable declarations specification](./src/environment/src/env_var/docs/env-var-declarations/env-var-declarations.spec.md)
- [Environment variable declarations acceptance](./src/environment/src/env_var/docs/env-var-declarations/env-var-declarations.acceptance.md)
