---
title: Environment variables access - Acceptance
summary: Acceptance scenarios for how modules and plugins access environment values.
tags: [environment, env-var, acceptance, access, security, gherkin]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_CONVENTIONS.md](../../../../../../docs/GENERAL_CONVENTIONS.md#normative-language).
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords.

## ENV-VAR_ACCESS-ACC-0002

Name: Dependent declaration provides a resolved value

**Links:**

- [ENV-VAR_ACCESS-REQ-0002](./env-var-access.spec.md#env-var_access-req-0002)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_ACCESS-FEAT-0001 — Environment variable access

  Rule: ENV-VAR_ACCESS-REQ-0002 — Declared value access

    Scenario: ENV-VAR_ACCESS-ACC-0002 — Dependent declaration provides a resolved value
      Given a plugin declares "EXAMPLE_KEY" as a dependency
      And "EXAMPLE_KEY" has a resolved value
      When the plugin requests the value of "EXAMPLE_KEY"
      Then the software provides the resolved value
```

---

## ENV-VAR_ACCESS-ACC-0003

Name: Undeclared value request is rejected

**Links:**

- [ENV-VAR_ACCESS-REQ-0002](./env-var-access.spec.md#env-var_access-req-0002)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_ACCESS-FEAT-0001 — Environment variable access

  Rule: ENV-VAR_ACCESS-REQ-0002 — Declared value access

    Scenario: ENV-VAR_ACCESS-ACC-0003 — Undeclared value request is rejected
      Given a plugin neither owns "EXAMPLE_KEY" nor declares it as a dependency
      When the plugin requests the value of "EXAMPLE_KEY"
      Then the software rejects the request
```

---

## ENV-VAR_ACCESS-ACC-0006

Name: Owned declaration provides a resolved value

**Links:**

- [ENV-VAR_ACCESS-REQ-0002](./env-var-access.spec.md#env-var_access-req-0002)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_ACCESS-FEAT-0001 — Environment variable access

  Rule: ENV-VAR_ACCESS-REQ-0002 — Declared value access

    Scenario: ENV-VAR_ACCESS-ACC-0006 — Owned declaration provides a resolved value
      Given a plugin owns "EXAMPLE_KEY"
      And "EXAMPLE_KEY" has a resolved value
      When the plugin requests the value of "EXAMPLE_KEY"
      Then the software provides the resolved value
```
