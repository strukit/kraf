---
title: Environment variables projection - Acceptance
summary: Acceptance scenarios for projecting environment variables.
tags: [environment, env-var, acceptance, projection, gherkin]
status: draft
---

## ENV_VAR_PROJECTION-ACC-0001

Name: Declared export alias is projected

**Links:**

- [ENV_VAR_PROJECTION-REQ-0001](./env-var-projection.spec.md#env_var_projection-req-0001)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_PROJECTION-FEAT-0001 — Environment variable projection

  Rule: ENV_VAR_PROJECTION-REQ-0001 — Explicit export authorization

    Scenario: ENV_VAR_PROJECTION-ACC-0001 — Declared export alias is projected
      Given "EXAMPLE_TOKEN" is declared as an export alias
      And "EXAMPLE_TOKEN" has a resolved value
      When the software applies the projection to a managed process
      Then the managed process receives "EXAMPLE_TOKEN"
```

---

## ENV_VAR_PROJECTION-ACC-0002

Name: Non-exportable resolved value is not projected

**Links:**

- [ENV_VAR_PROJECTION-REQ-0001](./env-var-projection.spec.md#env_var_projection-req-0001)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_PROJECTION-FEAT-0001 — Environment variable projection

  Rule: ENV_VAR_PROJECTION-REQ-0001 — Explicit export authorization

    Scenario: ENV_VAR_PROJECTION-ACC-0002 — Non-exportable resolved value is not projected
      Given "INTERNAL_TOKEN" has a resolved value
      And "INTERNAL_TOKEN" is not declared as an export alias
      When the software applies the projection to a managed process
      Then the managed process does not receive "INTERNAL_TOKEN"
```

---

## ENV_VAR_PROJECTION-ACC-0003

Name: Export aliases receive the canonical resolved value

**Links:**

- [ENV_VAR_PROJECTION-REQ-0002](./env-var-projection.spec.md#env_var_projection-req-0002)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_PROJECTION-FEAT-0001 — Environment variable projection

  Rule: ENV_VAR_PROJECTION-REQ-0002 — Export value symmetry

    Scenario: ENV_VAR_PROJECTION-ACC-0003 — Export aliases receive the canonical resolved value
      Given "HOME_TEMP" resolves to "/workspace/.tmp"
      And "TMPDIR" is an export alias of "HOME_TEMP"
      When the software resolves the declaration
      Then "TMPDIR" resolves to "/workspace/.tmp"
```
