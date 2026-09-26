---
title: Environment variables declaration - SPEC
summary: Requirements for declaring environment variables.
tags: [environment, env-var, specification, spec, declaration]
---

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

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_DECLARATIONS-ACC-0001](./env-var-declarations.acceptance.md#env-var_declarations-acc-0001)
- [ENV-VAR_DECLARATIONS-ACC-0002](./env-var-declarations.acceptance.md#env-var_declarations-acc-0002)

**Status**:

- Doc status: Draft
- Implementation status: Not implemented

```text
The software SHALL resolve and project only environment-variable declarations
that are built into the software or packaged as immutable metadata with a
plugin.

The software MAY accept values for known declarations from runtime
configuration.

The software SHALL NOT accept declarations, import aliases, export aliases, or
source eligibility from runtime configuration.

**WHEN:** Runtime configuration provides a value for an unknown declaration,

**THE SOFTWARE SHALL:** reject the configuration.
```

---

### ENV-VAR_DECLARATIONS-REQ-0002

Name: Canonical key uniqueness

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_DECLARATIONS-ACC-0003](./env-var-declarations.acceptance.md#env-var_declarations-acc-0003)

**Status**:

- Doc status: Draft
- Implementation status: Partially implemented

```text
The software SHALL accept at most one declaration for each canonical key.

**WHEN:** A declaration set contains duplicate canonical keys,

**THE SOFTWARE SHALL:** reject the declaration set.
```

---

### ENV-VAR_DECLARATIONS-REQ-0003

Name: Mapping mode validity

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_DECLARATIONS-ACC-0004](./env-var-declarations.acceptance.md#env-var_declarations-acc-0004)
- [ENV-VAR_DECLARATIONS-ACC-0005](./env-var-declarations.acceptance.md#env-var_declarations-acc-0005)
- [ENV-VAR_DECLARATIONS-ACC-0006](./env-var-declarations.acceptance.md#env-var_declarations-acc-0006)

**Status**:

- Doc status: Draft
- Implementation status: Partially implemented

```text
The software SHALL accept a declaration using merge import mode only when it
declares at least two import aliases.

The software SHALL accept a declaration using pass-through import mode only when
its canonical key is its only import alias.

The software SHALL NOT accept a declaration using pass-through import mode that
declares a fallback.
```
