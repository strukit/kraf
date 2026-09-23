---
title: Environment variables projection - Acceptance
summary: Acceptance scenarios for projecting environment variables.
tags: [environment, env-var, acceptance, projection, gherkin]
---

## ENV-VAR_PROJECTION-ACC-0001

Name: Declared export alias is projected

**Links:**

- [ENV-VAR_PROJECTION-REQ-0001](./env-var-projection.spec.md#env-var_projection-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_PROJECTION-FEAT-0001 — Environment variable projection

  Rule: ENV-VAR_PROJECTION-REQ-0001 — Explicit export authorization

    Scenario: ENV-VAR_PROJECTION-ACC-0001 — Declared export alias is projected
      Given "ENV-VAR_X" is declared as an export alias
      And "ENV-VAR_X" has a resolved value
      When the software applies the projection to a managed process
      Then the managed process receives "ENV-VAR_X"
```

---

## ENV-VAR_PROJECTION-ACC-0002

Name: Non-exportable resolved value is not projected

**Links:**

- [ENV-VAR_PROJECTION-REQ-0001](./env-var-projection.spec.md#env-var_projection-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_PROJECTION-FEAT-0001 — Environment variable projection

  Rule: ENV-VAR_PROJECTION-REQ-0001 — Explicit export authorization

    Scenario: ENV-VAR_PROJECTION-ACC-0002 — Non-exportable resolved value is not projected
      Given "ENV-VAR_X" has a resolved value
      And "ENV-VAR_X" is not declared as an export alias
      When the software applies the projection to a managed process
      Then the managed process does not receive "ENV-VAR_X"
```

---

## ENV-VAR_PROJECTION-ACC-0003

Name: Export aliases receive the canonical resolved value

**Links:**

- [ENV-VAR_PROJECTION-REQ-0002](./env-var-projection.spec.md#env-var_projection-req-0002)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_PROJECTION-FEAT-0001 — Environment variable projection

  Rule: ENV-VAR_PROJECTION-REQ-0002 — Export value symmetry

    Scenario: ENV-VAR_PROJECTION-ACC-0003 — Export aliases receive the canonical resolved value
      Given "HOME_TEMP" resolves to "/workspace/.tmp"
      And "TMPDIR" is an export alias of "HOME_TEMP"
      When the software resolves the declaration
      Then "TMPDIR" resolves to "/workspace/.tmp"
```
