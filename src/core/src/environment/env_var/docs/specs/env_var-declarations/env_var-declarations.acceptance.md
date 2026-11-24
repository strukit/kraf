---
description: Acceptance scenarios for declaring environment variables.
tags: [environment, env_var, acceptance, declaration, gherkin]
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

## ENV_VAR-DECLARATIONS-ACC-0001

Name: Value for an unknown declaration is rejected

**Links:**

- [ENV_VAR-DECLARATIONS-REQ-0001](./env_var-declarations.spec.md#ENV_VAR-DECLARATIONS-REQ-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR-DECLARATIONS-REQ-0001 — Static declaration

    Scenario: ENV_VAR-DECLARATIONS-ACC-0001 — Value for an unknown declaration is rejected
      Given no declaration exists for "EXAMPLE_KEY"
      When a configuration file provides a value for "EXAMPLE_KEY"
      Then the software rejects the configuration
      And "EXAMPLE_KEY" is not projected to a managed process
```

---

## ENV_VAR-DECLARATIONS-ACC-0002

Name: Known declaration accepts a value from a configuration file

**Links:**

- [ENV_VAR-DECLARATIONS-REQ-0001](./env_var-declarations.spec.md#ENV_VAR-DECLARATIONS-REQ-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR-DECLARATIONS-REQ-0001 — Static declaration

    Scenario: ENV_VAR-DECLARATIONS-ACC-0002 — Known declaration accepts a value from a configuration file
      Given a declaration exists for "EXAMPLE_KEY"
      When a configuration file provides a value for "EXAMPLE_KEY"
      Then the software accepts the configuration
```

---

## ENV_VAR-DECLARATIONS-ACC-0003

Name: Duplicate canonical key is rejected

**Links:**

- [ENV_VAR-DECLARATIONS-REQ-0002](./env_var-declarations.spec.md#ENV_VAR-DECLARATIONS-REQ-0002)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR-DECLARATIONS-REQ-0002 — Canonical key uniqueness

    Scenario: ENV_VAR-DECLARATIONS-ACC-0003 — Duplicate canonical key is rejected
      Given two declarations use the canonical key "EXAMPLE_KEY"
      When the software loads the declaration set
      Then the software rejects the declaration set
```

---

## ENV_VAR-DECLARATIONS-ACC-0004

Name: Merge mapping declares multiple import aliases

**Links:**

- [ENV_VAR-DECLARATIONS-REQ-0003](./env_var-declarations.spec.md#ENV_VAR-DECLARATIONS-REQ-0003)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR-DECLARATIONS-REQ-0003 — Mapping mode validity

    Scenario: ENV_VAR-DECLARATIONS-ACC-0004 — Merge mapping declares multiple import aliases
      Given a declaration uses merge import mode
      When the declaration defines fewer than two import aliases
      Then the software rejects the declaration
```

---

## ENV_VAR-DECLARATIONS-ACC-0005

Name: Pass-through mapping imports and exports only its canonical key

**Links:**

- [ENV_VAR-DECLARATIONS-REQ-0003](./env_var-declarations.spec.md#ENV_VAR-DECLARATIONS-REQ-0003)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR-DECLARATIONS-REQ-0003 — Mapping mode validity

    Scenario: ENV_VAR-DECLARATIONS-ACC-0005 — Pass-through mapping imports and exports only its canonical key
      Given a declaration uses pass-through import mode
      When the declaration imports or exports an alias other than its canonical key
      Then the software rejects the declaration
```

---

## ENV_VAR-DECLARATIONS-ACC-0006

Name: Pass-through mapping has no fallback

**Links:**

- [ENV_VAR-DECLARATIONS-REQ-0003](./env_var-declarations.spec.md#ENV_VAR-DECLARATIONS-REQ-0003)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR-DECLARATIONS-REQ-0003 — Mapping mode validity

    Scenario: ENV_VAR-DECLARATIONS-ACC-0006 — Pass-through mapping has no fallback
      Given a declaration uses pass-through import mode
      When the declaration defines a fallback
      Then the software rejects the declaration
```

---

## ENV_VAR-DECLARATIONS-ACC-0007

Name: Configuration file declaration is accepted at startup

**Links:**

- [ENV_VAR-DECLARATIONS-REQ-0001](./env_var-declarations.spec.md#ENV_VAR-DECLARATIONS-REQ-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR-DECLARATIONS-REQ-0001 — Static declaration

    Scenario: ENV_VAR-DECLARATIONS-ACC-0007 — Configuration file declaration is accepted at startup
      Given a configuration file declares "RUSTUP_HOME" with the export alias "RUSTUP_HOME"
      When the software loads the configuration file before resolution starts
      Then "RUSTUP_HOME" is part of the declaration set
```

---

## ENV_VAR-DECLARATIONS-ACC-0008

Name: Plugin declaration during a session is rejected

**Links:**

- [ENV_VAR-DECLARATIONS-REQ-0001](./env_var-declarations.spec.md#ENV_VAR-DECLARATIONS-REQ-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV_VAR-DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR-DECLARATIONS-REQ-0001 — Static declaration

    Scenario: ENV_VAR-DECLARATIONS-ACC-0008 — Plugin declaration during a session is rejected
      Given a session is running
      When a plugin requests a new declaration for "EXAMPLE_KEY"
      Then the software rejects the request
      And "EXAMPLE_KEY" is not projected to a managed process
```

## Links

- [Specification](./env_var-declarations.spec.md): Requirements for declaring
  environment variables.
- [README.md](../../../README.md): Declarative model, resolution, and projection
  of environment variables in Kraf.
