---
description: Requirements for how modules and plugins access resolved environment values.
tags: [environment, env_var, specification, spec, access, security]
type: ["spec"]
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

</br>

> [!IMPORTANT]
> Requirements in this specification use EARS notation. **SHALL** and **SHALL
> NOT** define mandatory behavior; **SHOULD** and **SHOULD NOT** define expected
> behavior that requires an explicit justification to deviate from; **MAY**
> defines optional behavior.
>
> These terms follow RFC 2119 and RFC 8174.

## Requirements

### ENV_VAR-ACCESS-REQ-0001

Name: Declared value access

**Links:**

- [Intent](../../env_var-intent.md): module intent.
- [ENV_VAR-ACCESS-ACC-0001](./env_var-access.acceptance.md#ENV_VAR-ACCESS-ACC-0001)
- [ENV_VAR-ACCESS-ACC-0002](./env_var-access.acceptance.md#ENV_VAR-ACCESS-ACC-0002)
- [ENV_VAR-ACCESS-ACC-0003](./env_var-access.acceptance.md#ENV_VAR-ACCESS-ACC-0003)

**Status**:

- Doc status: Draft
- Implementation status: Not Implemented

```text
WHEN a module or plugin requests a resolved value for a declaration it owns or
declares as a dependency,
THE SOFTWARE SHALL provide the resolved value.

IF a module or plugin requests a resolved value for a declaration it neither
owns nor declares as a dependency,
THEN THE SOFTWARE SHALL reject the request.
```

## Links

- [Intent](../../env_var-intent.md): Why Kraf resolves and projects environment
  variables through a declarative model.
- [Acceptance](./env_var-access.acceptance.md): Acceptance scenarios for how
  modules and plugins access environment values.
- [README.md](../../../README.md): Declarative model, resolution, and projection
  of environment variables in Kraf.
