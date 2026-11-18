---
description: Acceptance scenarios for how modules and plugins access environment values.
tags: [environment, env-var, acceptance, access, security, gherkin]
type: ["acceptance"]
scope: ["module:env-var"]
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
> [GENERAL_STANDARDS.md](../../../../../../docs/standards/GENERAL_STANDARDS.md#normative-language).

## ENV-VAR_ACCESS-ACC-0001

Name: Dependent declaration provides a resolved value

**Links:**

- [ENV-VAR_ACCESS-REQ-0001](./env-var-access.spec.md#env-var_access-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_ACCESS-FEAT-0001 — Environment variable access

  Rule: ENV-VAR_ACCESS-REQ-0001 — Declared value access

    Scenario: ENV-VAR_ACCESS-ACC-0001 — Dependent declaration provides a resolved value
      Given a plugin declares "EXAMPLE_KEY" as a dependency
      And "EXAMPLE_KEY" has a resolved value
      When the plugin requests the value of "EXAMPLE_KEY"
      Then the software provides the resolved value
```

---

## ENV-VAR_ACCESS-ACC-0002

Name: Undeclared value request is rejected

**Links:**

- [ENV-VAR_ACCESS-REQ-0001](./env-var-access.spec.md#env-var_access-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_ACCESS-FEAT-0001 — Environment variable access

  Rule: ENV-VAR_ACCESS-REQ-0001 — Declared value access

    Scenario: ENV-VAR_ACCESS-ACC-0002 — Undeclared value request is rejected
      Given a plugin neither owns "EXAMPLE_KEY" nor declares it as a dependency
      When the plugin requests the value of "EXAMPLE_KEY"
      Then the software rejects the request
```

---

## ENV-VAR_ACCESS-ACC-0003

Name: Owned declaration provides a resolved value

**Links:**

- [ENV-VAR_ACCESS-REQ-0001](./env-var-access.spec.md#env-var_access-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_ACCESS-FEAT-0001 — Environment variable access

  Rule: ENV-VAR_ACCESS-REQ-0001 — Declared value access

    Scenario: ENV-VAR_ACCESS-ACC-0003 — Owned declaration provides a resolved value
      Given a plugin owns "EXAMPLE_KEY"
      And "EXAMPLE_KEY" has a resolved value
      When the plugin requests the value of "EXAMPLE_KEY"
      Then the software provides the resolved value
```

## Links

- [Specification](./env-var-access.spec.md): Requirements for how modules and
  plugins access resolved environment values.
- [README.md](../../README.md): Declarative model, resolution, and projection of
  environment variables in Kraf.
