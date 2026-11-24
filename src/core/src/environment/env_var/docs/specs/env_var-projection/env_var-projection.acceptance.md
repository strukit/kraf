---
description: Acceptance scenarios for projecting environment variables.
tags: [environment, env_var, acceptance, projection, gherkin]
type: ["acceptance"]
scope: ["module:env_var"]
language: en
---

> [!NOTE]
> The keywords follow RFC 2119 and RFC 8174 across three levels of force:
>
> - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
> - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
> - **Optional:** MAY, OPTIONAL
>
> See
> [GENERAL_STANDARDS.md](../../../../../../../../docs/standards/GENERAL_STANDARDS.md#normative-language).

## ENV_VAR-PROJECTION-ACC-0001

Name: Declared export alias is projected

**Links:**

- [ENV_VAR-PROJECTION-REQ-0001](./env_var-projection.spec.md#ENV_VAR-PROJECTION-REQ-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-PROJECTION-FEAT-0001 — Environment variable projection

  Rule: ENV_VAR-PROJECTION-REQ-0001 — Explicit export authorization

    Scenario: ENV_VAR-PROJECTION-ACC-0001 — Declared export alias is projected
      Given "EXAMPLE_KEY" is declared as an export alias
      And "EXAMPLE_KEY" has a resolved value
      When the software applies the projection to a managed process
      Then the managed process receives "EXAMPLE_KEY"
```

---

## ENV_VAR-PROJECTION-ACC-0002

Name: Non-exportable resolved value is not projected

**Links:**

- [ENV_VAR-PROJECTION-REQ-0001](./env_var-projection.spec.md#ENV_VAR-PROJECTION-REQ-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-PROJECTION-FEAT-0001 — Environment variable projection

  Rule: ENV_VAR-PROJECTION-REQ-0001 — Explicit export authorization

    Scenario: ENV_VAR-PROJECTION-ACC-0002 — Non-exportable resolved value is not projected
      Given "EXAMPLE_KEY" has a resolved value
      And "EXAMPLE_KEY" is not declared as an export alias
      When the software applies the projection to a managed process
      Then the managed process does not receive "EXAMPLE_KEY"
```

---

## ENV_VAR-PROJECTION-ACC-0003

Name: Export aliases receive the canonical resolved value

**Links:**

- [ENV_VAR-PROJECTION-REQ-0002](./env_var-projection.spec.md#ENV_VAR-PROJECTION-REQ-0002)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-PROJECTION-FEAT-0001 — Environment variable projection

  Rule: ENV_VAR-PROJECTION-REQ-0002 — Export value symmetry

    Scenario: ENV_VAR-PROJECTION-ACC-0003 — Export aliases receive the canonical resolved value
      Given "HOME_TEMP" resolves to "/workspace/.tmp"
      And "TMPDIR" is an export alias of "HOME_TEMP"
      When the software projects the declaration
      Then "TMPDIR" is projected with the value "/workspace/.tmp"
```

---

## ENV_VAR-PROJECTION-ACC-0004

Name: Projected output preserves export order

**Links:**

- [ENV_VAR-PROJECTION-REQ-0003](./env_var-projection.spec.md#ENV_VAR-PROJECTION-REQ-0003)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-PROJECTION-FEAT-0001 — Environment variable projection

  Rule: ENV_VAR-PROJECTION-REQ-0003 — Projected output order

    Scenario: ENV_VAR-PROJECTION-ACC-0004 — Projected output preserves export order
      Given "HOME_TEMP" resolves to "/workspace/.tmp"
      And its export aliases are "TEMP", "TMPDIR", and "TMP" in that order
      When the software projects the declaration
      Then the projected keys are "TEMP", "TMPDIR", and "TMP" in that order
      And each projected key has the value "/workspace/.tmp"
```

---

## ENV_VAR-PROJECTION-ACC-0005

Name: Empty resolved value leaves its export aliases unset

**Links:**

- [ENV_VAR-PROJECTION-REQ-0004](./env_var-projection.spec.md#ENV_VAR-PROJECTION-REQ-0004)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-PROJECTION-FEAT-0001 — Environment variable projection

  Rule: ENV_VAR-PROJECTION-REQ-0004 — Empty values are not projected

    Scenario: ENV_VAR-PROJECTION-ACC-0005 — Empty resolved value leaves its export aliases unset
      Given "USER_TIMEZONE" resolves to ""
      And "TZ" is an export alias of "USER_TIMEZONE"
      When the software projects the declaration
      Then the projection does not contain "TZ"
```

## Links

- [Specification](./env_var-projection.spec.md): Requirements for projecting
  resolved environment variables to managed processes.
- [README.md](../../../README.md): Declarative model, resolution, and projection
  of environment variables in Kraf.
