# Environment variables declaration — Acceptance

## ENV_VAR_DECLARATIONS-ACC-0001

Name: Unknown runtime declaration is rejected

**Links:**

- [ENV_VAR_DECLARATIONS-REQ-0001](./env-var-declarations.spec.md#env_var_declarations-req-0001)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR_DECLARATIONS-REQ-0001 — Static declaration

    Scenario: ENV_VAR_DECLARATIONS-ACC-0001 — Unknown runtime declaration is rejected
      Given no declaration exists for "ENV_VAR_X"
      When runtime configuration provides a value for "ENV_VAR_X"
      Then the software rejects the configuration
      And "ENV_VAR_X" is not projected to a managed process
```

---

## ENV_VAR_DECLARATIONS-ACC-0002

Name: Known runtime declaration accepts a value

**Links:**

- [ENV_VAR_DECLARATIONS-REQ-0001](./env-var-declarations.spec.md#env_var_declarations-req-0001)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR_DECLARATIONS-REQ-0001 — Static declaration

    Scenario: ENV_VAR_DECLARATIONS-ACC-0002 — Known runtime declaration accepts a value
      Given a declaration exists for "ENV_VAR_X"
      When runtime configuration provides a value for "ENV_VAR_X"
      Then the software accepts the configuration
```

---

## ENV_VAR_DECLARATIONS-ACC-0003

Name: Duplicate canonical key is rejected

**Links:**

- [ENV_VAR_DECLARATIONS-REQ-0002](./env-var-declarations.spec.md#env_var_declarations-req-0002)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR_DECLARATIONS-REQ-0002 — Canonical key uniqueness

    Scenario: ENV_VAR_DECLARATIONS-ACC-0003 — Duplicate canonical key is rejected
      Given two declarations use the canonical key "ENV_VAR_X"
      When the software loads the declaration set
      Then the software rejects the declaration set
```

---

## ENV_VAR_DECLARATIONS-ACC-0004

Name: Merge mapping declares multiple import aliases

**Links:**

- [ENV_VAR_DECLARATIONS-REQ-0003](./env-var-declarations.spec.md#env_var_declarations-req-0003)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR_DECLARATIONS-REQ-0003 — Mapping mode validity

    Scenario: ENV_VAR_DECLARATIONS-ACC-0004 — Merge mapping declares multiple import aliases
      Given a declaration uses merge import mode
      When the declaration defines fewer than two import aliases
      Then the software rejects the declaration
```

---

## ENV_VAR_DECLARATIONS-ACC-0005

Name: Pass-through mapping imports only its canonical key

**Links:**

- [ENV_VAR_DECLARATIONS-REQ-0003](./env-var-declarations.spec.md#env_var_declarations-req-0003)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR_DECLARATIONS-REQ-0003 — Mapping mode validity

    Scenario: ENV_VAR_DECLARATIONS-ACC-0005 — Pass-through mapping imports only its canonical key
      Given a declaration uses pass-through import mode
      When the declaration imports an alias other than its canonical key
      Then the software rejects the declaration
```

---

## ENV_VAR_DECLARATIONS-ACC-0006

Name: Pass-through mapping has no fallback

**Links:**

- [ENV_VAR_DECLARATIONS-REQ-0003](./env-var-declarations.spec.md#env_var_declarations-req-0003)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_DECLARATIONS-FEAT-0001 — Environment variable declarations

  Rule: ENV_VAR_DECLARATIONS-REQ-0003 — Mapping mode validity

    Scenario: ENV_VAR_DECLARATIONS-ACC-0006 — Pass-through mapping has no fallback
      Given a declaration uses pass-through import mode
      When the declaration defines a fallback
      Then the software rejects the declaration
```
