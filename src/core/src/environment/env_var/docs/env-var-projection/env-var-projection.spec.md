---
description: Requirements for projecting resolved environment variables to managed processes.
tags: [environment, env-var, specification, spec, projection]
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
> [GENERAL_STANDARDS.md](../../../../../../../docs/standards/GENERAL_STANDARDS.md#normative-language).

</br>

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
THE SOFTWARE SHALL project only keys declared as export aliases by the active
mappings.
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
THE SOFTWARE SHALL assign the declaration's resolved value to every export alias
the declaration produces.
```

---

### ENV-VAR_PROJECTION-REQ-0003

Name: Projected output order

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_PROJECTION-ACC-0004](./env-var-projection.acceptance.md#env-var_projection-acc-0004)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL emit projected declarations in active mapping order.

THE SOFTWARE SHALL emit, for each declaration, its export aliases in declaration
order.
```

## Links

- [Intent](../env-var-intent.md): Why Kraf resolves and projects environment
  variables through a declarative model.
- [Acceptance](./env-var-projection.acceptance.md): Acceptance scenarios for
  projecting environment variables.
- [README.md](../../README.md): Declarative model, resolution, and projection of
  environment variables in Kraf.
