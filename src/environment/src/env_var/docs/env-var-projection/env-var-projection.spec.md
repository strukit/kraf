---
title: Environment variables projection - SPEC
summary: Requirements for projecting resolved environment variables to managed processes.
tags: [environment, env-var, specification, spec, projection]
---

> [!IMPORTANT]
> Requirements in this specification use EARS notation. **SHALL** and **SHALL
> NOT** define mandatory behavior; **SHOULD** and **SHOULD NOT** define expected
> behavior that requires an explicit justification to deviate from; **MAY**
> defines optional behavior.
>
> These terms follow RFC 2119 and RFC 8174.

## Requirements

### ENV-VAR_PROJECTION-REQ-0001

Name: Explicit export authorization

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_PROJECTION-ACC-0001](./env-var-projection.acceptance.md#env-var_projection-acc-0001)
- [ENV-VAR_PROJECTION-ACC-0002](./env-var-projection.acceptance.md#env-var_projection-acc-0002)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
**SHALL:** The software projects only keys declared as export aliases by the
active mappings.
```

---

### ENV-VAR_PROJECTION-REQ-0002

Name: Export value symmetry

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_PROJECTION-ACC-0003](./env-var-projection.acceptance.md#env-var_projection-acc-0003)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
**SHALL:** Every export alias produced by a declaration has the declaration's
resolved value.
```
