---
title: Environment variables access - SPEC
summary: Requirements for how modules and plugins access resolved environment values.
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

Name: Declared value access

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_ACCESS-ACC-0001](./env-var-access.acceptance.md#env-var_access-acc-0001)
- [ENV-VAR_ACCESS-ACC-0002](./env-var-access.acceptance.md#env-var_access-acc-0002)
- [ENV-VAR_ACCESS-ACC-0003](./env-var-access.acceptance.md#env-var_access-acc-0003)

**Status**:

- Doc status: Draft
- Implementation status: Unknown

```text
WHEN a module or plugin requests a resolved value for a declaration it owns or
declares as a dependency,
THE SOFTWARE SHALL provide the resolved value.

IF a module or plugin requests a resolved value for a declaration it neither
owns nor declares as a dependency,
THEN THE SOFTWARE SHALL reject the request.
```

### ENV-VAR_ACCESS-REQ-0002

Name: Host environment access isolation

**Links:**

- [Intent](../env-var-intent.md#security-access): module intent.

**Status**:

- Doc status: Undefined

```text
Isolation of host environment access, as stated in the intent's "Security
access" principle, is not yet defined:

- only the environment module reads raw host environment values;
- the environment module exposes no operation that enumerates all host
  environment variables;
- the environment module exposes no arbitrary string-based host lookup outside
  the module.
```
