---
title: Environment variables resolution - SPEC
summary: Requirements for resolving declared environment variables.
tags: [environment, env-var, specification, spec, resolution]
---

> [!IMPORTANT]
> Requirements in this specification use EARS notation. **SHALL** and **SHALL
> NOT** define mandatory behavior; **SHOULD** and **SHOULD NOT** define expected
> behavior that requires an explicit justification to deviate from; **MAY**
> defines optional behavior.
>
> These terms follow RFC 2119 and RFC 8174.

## Requirements

### ENV-VAR_RESOLUTION-REQ-0001

Name: Declared import dependency collection

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_RESOLUTION-ACC-0001](./env-var-resolution.acceptance.md#env-var_resolution-acc-0001)
- [ENV-VAR_RESOLUTION-ACC-0002](./env-var-resolution.acceptance.md#env-var_resolution-acc-0002)
- [ENV-VAR_RESOLUTION-ACC-0025](./env-var-resolution.acceptance.md#env-var_resolution-acc-0025)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL collect only import aliases and the first valid dollar
variable reference in a declaration fallback.

THE SOFTWARE SHALL deduplicate and lexicographically sort collected import and
export keys.
```

---

### ENV-VAR_RESOLUTION-REQ-0002

Name: First-found resolution

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_RESOLUTION-ACC-0003](./env-var-resolution.acceptance.md#env-var_resolution-acc-0003)
- [ENV-VAR_RESOLUTION-ACC-0004](./env-var-resolution.acceptance.md#env-var_resolution-acc-0004)
- [ENV-VAR_RESOLUTION-ACC-0010](./env-var-resolution.acceptance.md#env-var_resolution-acc-0010)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL resolve a declaration using first-found import mode to the
first available import alias in declaration order. A collected value is
available when it is present, including when its value is empty.

WHEN no import alias provides a value,
THE SOFTWARE SHALL resolve the declaration fallback.
```

---

### ENV-VAR_RESOLUTION-REQ-0003

Name: Path merge resolution

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_RESOLUTION-ACC-0005](./env-var-resolution.acceptance.md#env-var_resolution-acc-0005)
- [ENV-VAR_RESOLUTION-ACC-0006](./env-var-resolution.acceptance.md#env-var_resolution-acc-0006)
- [ENV-VAR_RESOLUTION-ACC-0014](./env-var-resolution.acceptance.md#env-var_resolution-acc-0014)
- [ENV-VAR_RESOLUTION-ACC-0015](./env-var-resolution.acceptance.md#env-var_resolution-acc-0015)
- [ENV-VAR_RESOLUTION-ACC-0021](./env-var-resolution.acceptance.md#env-var_resolution-acc-0021)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL resolve a declaration using merge import mode by ignoring
empty imported values, splitting each remaining value into paths using the
current platform semantics, and joining the resulting paths in declaration order
using the current platform path separator.

THE SOFTWARE SHALL NOT deduplicate path entries during merge resolution.

WHEN no import alias provides a non-empty path value,
THE SOFTWARE SHALL resolve the declaration fallback.

IF joining the collected paths cannot produce a valid string for the current
platform,
THEN THE SOFTWARE SHALL resolve the declaration to an empty string.
```

---

### ENV-VAR_RESOLUTION-REQ-0004

Name: Placeholder expansion

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_RESOLUTION-ACC-0007](./env-var-resolution.acceptance.md#env-var_resolution-acc-0007)
- [ENV-VAR_RESOLUTION-ACC-0008](./env-var-resolution.acceptance.md#env-var_resolution-acc-0008)
- [ENV-VAR_RESOLUTION-ACC-0009](./env-var-resolution.acceptance.md#env-var_resolution-acc-0009)
- [ENV-VAR_RESOLUTION-ACC-0016](./env-var-resolution.acceptance.md#env-var_resolution-acc-0016)
- [ENV-VAR_RESOLUTION-ACC-0017](./env-var-resolution.acceptance.md#env-var_resolution-acc-0017)
- [ENV-VAR_RESOLUTION-ACC-0018](./env-var-resolution.acceptance.md#env-var_resolution-acc-0018)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
WHEN a fallback contains a tilde and a non-empty collected `HOME` value is
available,
THE SOFTWARE SHALL replace the first tilde with that value.

WHEN a non-empty collected `HOME` value is available,
THE SOFTWARE SHALL give tilde expansion precedence over dollar expansion.

WHEN a fallback contains a tilde but no non-empty collected `HOME` value is
available,
THE SOFTWARE SHALL leave the tilde unchanged and evaluate a dollar
placeholder when one is present.

THE SOFTWARE SHALL evaluate only the first dollar sign and, when it is followed
by one or more ASCII letters, digits, or underscores, replace that placeholder
with the collected value whose key matches its name, including an empty value.

IF a dollar sign has no valid placeholder name or a dollar placeholder has no
collected value,
THEN THE SOFTWARE SHALL preserve it literally.
```

---

### ENV-VAR_RESOLUTION-REQ-0005

Name: Source precedence and absence

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_RESOLUTION-ACC-0011](./env-var-resolution.acceptance.md#env-var_resolution-acc-0011)
- [ENV-VAR_RESOLUTION-ACC-0012](./env-var-resolution.acceptance.md#env-var_resolution-acc-0012)
- [ENV-VAR_RESOLUTION-ACC-0013](./env-var-resolution.acceptance.md#env-var_resolution-acc-0013)
- [ENV-VAR_RESOLUTION-ACC-0022](./env-var-resolution.acceptance.md#env-var_resolution-acc-0022)
- [ENV-VAR_RESOLUTION-ACC-0023](./env-var-resolution.acceptance.md#env-var_resolution-acc-0023)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
WHEN a declaration without import aliases and with source `Provided` has a
collected canonical value, including an empty value,
THE SOFTWARE SHALL resolve the declaration to that value.

WHEN that provided canonical value is absent,
THE SOFTWARE SHALL resolve the declaration fallback.

THE SOFTWARE SHALL resolve a declaration without import aliases and with source
`Host` to its fallback without reading a collected value for its canonical key.

WHEN the selected source and fallback provide no value,
THE SOFTWARE SHALL resolve the declaration to an empty string.
```

---

### ENV-VAR_RESOLUTION-REQ-0006

Name: Pass-through resolution

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_RESOLUTION-ACC-0019](./env-var-resolution.acceptance.md#env-var_resolution-acc-0019)
- [ENV-VAR_RESOLUTION-ACC-0020](./env-var-resolution.acceptance.md#env-var_resolution-acc-0020)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL resolve a declaration using pass-through import mode to the
first available import alias in declaration order without applying a fallback.

WHEN no import alias provides a value,
THE SOFTWARE SHALL resolve the declaration to an empty string.
```

---

### ENV-VAR_RESOLUTION-REQ-0007

Name: Resolved output order

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_RESOLUTION-ACC-0024](./env-var-resolution.acceptance.md#env-var_resolution-acc-0024)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL emit resolved declarations in active mapping order.

THE SOFTWARE SHALL emit, for each declaration, the canonical key first, followed
by its export aliases in declaration order, all with the same resolved value.
```
