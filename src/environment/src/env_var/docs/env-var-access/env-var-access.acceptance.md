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

## ENV-VAR_ACCESS-ACC-0001

Name: Raw host values are read only by the environment module

**Links:**

- [ENV-VAR_ACCESS-REQ-0001](./env-var-access.spec.md#env-var_access-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_ACCESS-FEAT-0001 — Environment variable access

  Rule: ENV-VAR_ACCESS-REQ-0001 — Host read isolation

    Scenario: ENV-VAR_ACCESS-ACC-0001 — Raw host values are read only by the environment module
      Given a module other than the environment module
      When it needs an environment value
      Then it does not read the value from the host directly
      And it obtains the value from the environment module
```

---

## ENV-VAR_ACCESS-ACC-0002

Name: Owned or dependent declaration provides a resolved value

**Links:**

- [ENV-VAR_ACCESS-REQ-0002](./env-var-access.spec.md#env-var_access-req-0002)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_ACCESS-FEAT-0001 — Environment variable access

  Rule: ENV-VAR_ACCESS-REQ-0002 — Declared value access

    Scenario: ENV-VAR_ACCESS-ACC-0002 — Owned or dependent declaration provides a resolved value
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

## ENV-VAR_ACCESS-ACC-0004

Name: No operation enumerates host variables

**Links:**

- [ENV-VAR_ACCESS-REQ-0003](./env-var-access.spec.md#env-var_access-req-0003)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_ACCESS-FEAT-0001 — Environment variable access

  Rule: ENV-VAR_ACCESS-REQ-0003 — No host enumeration or arbitrary lookup

    Scenario: ENV-VAR_ACCESS-ACC-0004 — No operation enumerates host variables
      Given the operations the environment module exposes
      When they are inspected
      Then none of them returns all host environment variables
```

---

## ENV-VAR_ACCESS-ACC-0005

Name: No arbitrary string lookup outside the module

**Links:**

- [ENV-VAR_ACCESS-REQ-0003](./env-var-access.spec.md#env-var_access-req-0003)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_ACCESS-FEAT-0001 — Environment variable access

  Rule: ENV-VAR_ACCESS-REQ-0003 — No host enumeration or arbitrary lookup

    Scenario: ENV-VAR_ACCESS-ACC-0005 — No arbitrary string lookup outside the module
      Given the operations the environment module exposes to other modules
      When they are inspected
      Then none of them looks up a host variable by an arbitrary string key
```
