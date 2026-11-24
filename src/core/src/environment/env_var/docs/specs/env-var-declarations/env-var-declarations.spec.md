---
description: Requirements for declaring environment variables.
tags: [environment, env-var, specification, spec, declaration]
type: ["spec"]
scope: ["module:env-var"]
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

### ENV-VAR_DECLARATIONS-REQ-0001

Name: Static declaration

**Links:**

- [Intent](../../env-var-intent.md): module intent.
- [ENV-VAR_DECLARATIONS-ACC-0001](./env-var-declarations.acceptance.md#env-var_declarations-acc-0001)
- [ENV-VAR_DECLARATIONS-ACC-0002](./env-var-declarations.acceptance.md#env-var_declarations-acc-0002)
- [ENV-VAR_DECLARATIONS-ACC-0007](./env-var-declarations.acceptance.md#env-var_declarations-acc-0007)
- [ENV-VAR_DECLARATIONS-ACC-0008](./env-var-declarations.acceptance.md#env-var_declarations-acc-0008)

**Status**:

- Doc status: Draft
- Implementation status: Not Implemented

```text
THE SOFTWARE SHALL determine the complete set of declarations before resolution
starts, from source code, immutable plugin metadata, or configuration files
loaded at startup.

THE SOFTWARE MAY accept values for known declarations from configuration files
loaded at startup and from dynamic sources while a session is running.

WHILE a session is running,
THE SOFTWARE SHALL NOT accept new declarations, import aliases, export aliases,
or source eligibility.

IF a plugin requests a new declaration while a session is running,
THEN THE SOFTWARE SHALL reject the request.

IF a configuration file provides a value for an unknown declaration,
THEN THE SOFTWARE SHALL reject the configuration.
```

---

### ENV-VAR_DECLARATIONS-REQ-0002

Name: Canonical key uniqueness

**Links:**

- [Intent](../../env-var-intent.md): module intent.
- [ENV-VAR_DECLARATIONS-ACC-0003](./env-var-declarations.acceptance.md#env-var_declarations-acc-0003)

**Status**:

- Doc status: Draft
- Implementation status: Partially Implemented

```text
THE SOFTWARE SHALL accept at most one declaration for each canonical key.

IF a declaration set contains duplicate canonical keys,
THEN THE SOFTWARE SHALL reject the declaration set.
```

---

### ENV-VAR_DECLARATIONS-REQ-0003

Name: Mapping mode validity

**Links:**

- [Intent](../../env-var-intent.md): module intent.
- [ENV-VAR_DECLARATIONS-ACC-0004](./env-var-declarations.acceptance.md#env-var_declarations-acc-0004)
- [ENV-VAR_DECLARATIONS-ACC-0005](./env-var-declarations.acceptance.md#env-var_declarations-acc-0005)
- [ENV-VAR_DECLARATIONS-ACC-0006](./env-var-declarations.acceptance.md#env-var_declarations-acc-0006)

**Status**:

- Doc status: Draft
- Implementation status: Partially Implemented

```text
THE SOFTWARE SHALL accept a declaration using merge import mode only when it
declares at least two import aliases.

THE SOFTWARE SHALL accept a declaration using pass-through import mode only when
its canonical key is its only import alias and its only export alias.

THE SOFTWARE SHALL NOT accept a declaration using pass-through import mode that
declares a fallback.
```

## Links

- [Intent](../../env-var-intent.md): Why Kraf resolves and projects environment
  variables through a declarative model.
- [Acceptance](./env-var-declarations.acceptance.md): Acceptance scenarios for
  declaring environment variables.
- [README.md](../../../README.md): Declarative model, resolution, and projection
  of environment variables in Kraf.
