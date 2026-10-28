---
title: Environment variables access - SPEC
summary: Requirements for how modules and plugins access host and resolved environment values.
tags: [environment, env-var, specification, spec, access, security]
---

> [!IMPORTANT]
> Requirements in this specification use EARS notation. **SHALL** and **SHALL
> NOT** define mandatory behavior; **SHOULD** and **SHOULD NOT** define expected
> behavior that requires an explicit justification to deviate from; **MAY**
> defines optional behavior.
>
> These terms follow RFC 2119 and RFC 8174.

## Requirements

### ENV-VAR_ACCESS-REQ-0001

Name: Host read isolation

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_ACCESS-ACC-0001](./env-var-access.acceptance.md#env-var_access-acc-0001)

**Status**:

- Doc status: Draft
- Implementation status: Unknown

```text
The software SHALL read raw host environment values only within the environment
module.
```

---

### ENV-VAR_ACCESS-REQ-0002

Name: Declared value access

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_ACCESS-ACC-0002](./env-var-access.acceptance.md#env-var_access-acc-0002)
- [ENV-VAR_ACCESS-ACC-0003](./env-var-access.acceptance.md#env-var_access-acc-0003)

**Status**:

- Doc status: Draft
- Implementation status: Unknown

```text
WHEN a module or plugin requests a resolved value for a declaration it owns or
declares as a dependency,
THE SOFTWARE SHALL provide the resolved value.

WHEN a module or plugin requests a resolved value for a declaration it neither
owns nor declares as a dependency,
THE SOFTWARE SHALL reject the request.
```

---

### ENV-VAR_ACCESS-REQ-0003

Name: No host enumeration or arbitrary lookup

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_ACCESS-ACC-0004](./env-var-access.acceptance.md#env-var_access-acc-0004)
- [ENV-VAR_ACCESS-ACC-0005](./env-var-access.acceptance.md#env-var_access-acc-0005)

**Status**:

- Doc status: Draft
- Implementation status: Unknown

```text
The software SHALL NOT expose an operation that enumerates all host environment
variables.

The software SHALL NOT expose a string-based host environment lookup outside
the environment module.
```
