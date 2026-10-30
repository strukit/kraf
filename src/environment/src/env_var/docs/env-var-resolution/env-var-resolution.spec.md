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
THE SOFTWARE SHALL collect the import aliases of every declaration of the
current platform.

WHEN the software collects import dependencies for a declaration whose fallback
has a first dollar sign followed by a placeholder name of one or more ASCII
letters, digits, or underscores,
THE SOFTWARE SHALL collect that placeholder name.

THE SOFTWARE SHALL NOT collect import dependencies other than these import
aliases and placeholder names.

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
WHILE a declaration uses first-found import mode,
WHILE an import alias of the declaration has a collected value, including an
empty value,
WHEN the software resolves the declaration,
THE SOFTWARE SHALL resolve the declaration to the collected value of the first
such import alias in declaration order.

WHILE a declaration uses first-found import mode,
WHILE the declaration has one or more import aliases,
WHILE no import alias of the declaration has a collected value,
WHEN the software resolves the declaration,
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
- Implementation status: Partially Implemented

```text
WHILE a declaration uses merge import mode,
IF the software resolves the declaration and an import alias of the declaration
is not valid Unicode,
THEN THE SOFTWARE SHALL resolve the declaration to an empty string, without
splitting or joining any path, and log a warning that names the declaration.

WHILE a declaration uses merge import mode,
IF the software resolves the declaration and an import alias of the declaration
has a non-empty collected value that is not valid Unicode,
THEN THE SOFTWARE SHALL resolve the declaration to an empty string, without
splitting or joining any path, and log a warning that names the declaration.

WHILE a declaration uses merge import mode,
WHILE an import alias of the declaration has a non-empty collected value,
WHILE every import alias of the declaration and every non-empty collected value
of its import aliases is valid Unicode,
WHEN the software resolves the declaration,
THE SOFTWARE SHALL ignore empty imported values, split each remaining value into
paths using the current platform semantics, and join the resulting paths in
declaration order using the current platform path separator.

WHILE a declaration uses merge import mode,
WHEN the software resolves the declaration,
THE SOFTWARE SHALL NOT deduplicate path entries.

WHILE a declaration uses merge import mode,
WHILE every import alias of the declaration is valid Unicode,
WHILE no import alias of the declaration has a non-empty collected value,
WHEN the software resolves the declaration,
THE SOFTWARE SHALL resolve the declaration fallback.
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
WHEN the software expands a fallback that contains a tilde,
THE SOFTWARE SHALL replace the first tilde with the provided `HOME` value
without evaluating any dollar placeholder.

THE SOFTWARE SHALL evaluate only the first dollar sign of a fallback.

WHEN the software expands a fallback that contains no tilde and whose first
dollar sign is followed by the placeholder name `HOME`,
THE SOFTWARE SHALL replace that placeholder with the provided `HOME` value.

WHEN the software expands a fallback that contains no tilde and whose first
dollar sign is followed by a placeholder name of one or more ASCII letters,
digits, or underscores, other than `HOME`, that matches the key of a collected
value, including an empty value,
THE SOFTWARE SHALL replace that placeholder with the collected value.

WHEN the software expands a fallback whose first dollar sign is followed by a
placeholder name of one or more ASCII letters, digits, or underscores, other
than `HOME`, that matches no collected value,
THE SOFTWARE SHALL leave the dollar sign and the placeholder name unchanged.

IF the software expands a fallback whose first dollar sign is not followed by
one or more ASCII letters, digits, or underscores,
THEN THE SOFTWARE SHALL leave the dollar sign unchanged.
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
WHILE a declaration has no import aliases,
WHILE the declaration has source `Provided`,
WHILE the canonical key of the declaration has a collected value, including an
empty value,
WHEN the software resolves the declaration,
THE SOFTWARE SHALL resolve the declaration to that value.

WHILE a declaration has no import aliases,
WHILE the declaration has source `Provided`,
WHILE the canonical key of the declaration has no collected value,
WHEN the software resolves the declaration,
THE SOFTWARE SHALL resolve the declaration fallback.

WHILE a declaration has no import aliases,
WHILE the declaration has source `Host`,
WHEN the software resolves the declaration,
THE SOFTWARE SHALL resolve the declaration to its fallback without reading a
collected value for its canonical key.

WHILE the selected source and the fallback of a declaration provide no value,
WHEN the software resolves the declaration,
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
WHILE a declaration uses pass-through import mode,
WHILE an import alias of the declaration has a collected value, including an
empty value,
WHEN the software resolves the declaration,
THE SOFTWARE SHALL resolve the declaration to the collected value of the first
such import alias in declaration order without applying a fallback.

WHILE a declaration uses pass-through import mode,
WHILE no import alias of the declaration has a collected value,
WHEN the software resolves the declaration,
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
THE SOFTWARE SHALL emit resolved declarations in the order they appear in the
current platform mapping.

THE SOFTWARE SHALL emit, for each declaration, the canonical key first, followed
by its export aliases in declaration order, all with the same resolved value.
```
