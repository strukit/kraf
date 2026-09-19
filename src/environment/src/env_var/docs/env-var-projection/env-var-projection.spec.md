---
title: Environment variables projection - SPEC
summary: Requirements for projecting resolved environment variables to managed processes.
tags: [environment, env-var, specification, spec, projection]
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

### ENV_VAR_PROJECTION-REQ-0001

Name: Explicit export authorization

**Links:**

- [ENV_VAR_PROJECTION-ACC-0001](./env-var-projection.acceptance.md#env_var_projection-acc-0001)
- [ENV_VAR_PROJECTION-ACC-0002](./env-var-projection.acceptance.md#env_var_projection-acc-0002)

**Status**:

- Requirement status: Draft
- Implementation status: Implemented
- Verification status: Not verified

**SHALL:** The software projects only keys declared as export aliases by the
active mappings.

---

### ENV_VAR_PROJECTION-REQ-0002

Name: Export value symmetry

**Links:**

- [ENV_VAR_PROJECTION-ACC-0003](./env-var-projection.acceptance.md#env_var_projection-acc-0003)

**Status**:

- Requirement status: Draft
- Implementation status: Implemented
- Verification status: Not verified

**SHALL:** Every export alias produced by a declaration has the declaration's
resolved value.
