---
title: Environment variables declaration - SPEC
summary: Requirements for declaring environment variables.
tags: [environment, env-var, specification, spec, declaration]
status: draft
---

> [!IMPORTANT]
> Requirements in this specification use EARS notation. **SHALL** and **SHALL
> NOT** define mandatory behavior; **SHOULD** and **SHOULD NOT** define expected
> behavior that requires an explicit justification to deviate from; **MAY**
> defines optional behavior.
>
> These terms follow RFC 2119 and RFC 8174.

## Requirements

### ENV_VAR_DECLARATIONS-REQ-0001

Name: Static declaration

**Links:**

- [ENV_VAR_DECLARATIONS-ACC-0001](./env-var-declarations.acceptance.md#env_var_declarations-acc-0001)
- [ENV_VAR_DECLARATIONS-ACC-0002](./env-var-declarations.acceptance.md#env_var_declarations-acc-0002)

**Status**:

- Requirement status: Draft
- Implementation status: Not implemented
- Verification status: Not verified

**SHALL:** The software resolves and projects only environment-variable
declarations that are built into the software or packaged as immutable metadata
with a plugin.

**MAY:** Runtime configuration provides values for known declarations.

**SHALL NOT:** Runtime configuration creates declarations or adds import
aliases, export aliases, or source eligibility.

**WHEN:** Runtime configuration provides a value for an unknown declaration,

**THE SOFTWARE SHALL:** reject the configuration.

---

### ENV_VAR_DECLARATIONS-REQ-0002

Name: Canonical key uniqueness

**Links:**

- [ENV_VAR_DECLARATIONS-ACC-0003](./env-var-declarations.acceptance.md#env_var_declarations-acc-0003)

**Status**:

- Requirement status: Draft
- Implementation status: Partially implemented
- Verification status: Not verified

**SHALL:** The software accepts at most one declaration for each canonical key.

**WHEN:** A declaration set contains duplicate canonical keys,

**THE SOFTWARE SHALL:** reject the declaration set.

---

### ENV_VAR_DECLARATIONS-REQ-0003

Name: Mapping mode validity

**Links:**

- [ENV_VAR_DECLARATIONS-ACC-0004](./env-var-declarations.acceptance.md#env_var_declarations-acc-0004)
- [ENV_VAR_DECLARATIONS-ACC-0005](./env-var-declarations.acceptance.md#env_var_declarations-acc-0005)
- [ENV_VAR_DECLARATIONS-ACC-0006](./env-var-declarations.acceptance.md#env_var_declarations-acc-0006)

**Status**:

- Requirement status: Draft
- Implementation status: Partially implemented
- Verification status: Not verified

**SHALL:** A declaration using merge import mode declares at least two import
aliases.

**SHALL:** A declaration using pass-through import mode declares only its
canonical key as an import alias.

**SHALL NOT:** A declaration using pass-through import mode declares a fallback.
