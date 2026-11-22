---
description: Requirements for platform-specific environment-variable mappings.
tags: [environment, env-var, specification, spec, host, platform]
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

### ENV-VAR_HOST-REQ-0001

Name: Supported-platform mapping completeness

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_HOST-ACC-0001](./env-var-host.acceptance.md#env-var_host-acc-0001)
- [ENV-VAR_HOST-ACC-0002](./env-var-host.acceptance.md#env-var_host-acc-0002)
- [ENV-VAR_HOST-ACC-0003](./env-var-host.acceptance.md#env-var_host-acc-0003)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL declare every shared canonical environment-variable key in
every supported platform mapping.
```

---

### ENV-VAR_HOST-REQ-0002

Name: Session-owned directory sources

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_HOST-ACC-0004](./env-var-host.acceptance.md#env-var_host-acc-0004)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL resolve the WORK_DIR and HOME_DIR declaration families from
provided values rather than from values imported from the host.
```

---

### ENV-VAR_HOST-REQ-0003

Name: Host home import

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_HOST-ACC-0005](./env-var-host.acceptance.md#env-var_host-acc-0005)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL declare, in every supported platform mapping, a `HOST_HOME`
declaration with source `Host` that imports the home directory of the host
user.

THE SOFTWARE SHALL NOT export the `HOST_HOME` declaration to a managed process.
```

---

### ENV-VAR_HOST-REQ-0004

Name: Native export aliases only

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_HOST-ACC-0006](./env-var-host.acceptance.md#env-var_host-acc-0006)
- [ENV-VAR_HOST-ACC-0010](./env-var-host.acceptance.md#env-var_host-acc-0010)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL NOT declare, in a supported platform mapping, a canonical key
of the PLATFORM, TERMINAL, USER, or HOME families as an export alias.

THE SOFTWARE SHALL NOT declare any canonical key of a supported platform
mapping as an export alias, unless the declaration uses the pass-through import
mode.
```

---

### ENV-VAR_HOST-REQ-0005

Name: Host path lists and isolated session path

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_HOST-ACC-0007](./env-var-host.acceptance.md#env-var_host-acc-0007)
- [ENV-VAR_HOST-ACC-0008](./env-var-host.acceptance.md#env-var_host-acc-0008)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL declare, in every supported platform mapping, the
`HOST_PATH`, `HOST_DATA`, and `HOST_CONFIG` declarations with source `Host`.

THE SOFTWARE SHALL NOT export the `HOST_PATH` declaration to a managed process.

THE SOFTWARE SHALL resolve the `HOME_PATH` declaration, in every supported
platform mapping, from its fallback, without importing the `PATH` of the host.

THE SOFTWARE SHALL NOT give the `HOST_DATA` and `HOST_CONFIG` declarations a
fallback, so that a value the host does not define is not projected.
```

---

### ENV-VAR_HOST-REQ-0006

Name: System declarations

**Links:**

- [Intent](../env-var-intent.md): module intent.
- [ENV-VAR_HOST-ACC-0009](./env-var-host.acceptance.md#env-var_host-acc-0009)

**Status**:

- Doc status: Draft
- Implementation status: Implemented

```text
THE SOFTWARE SHALL declare, in every supported platform mapping, the
`SYSTEM_ROOT`, `SYSTEM_DRIVE`, and `SYSTEM_PROGRAMS` declarations with source
`Host`.

WHERE the Windows mappings are included,
THE SOFTWARE SHALL import the `SYSTEM_ROOT`, `SYSTEM_DRIVE`, and
`SYSTEM_PROGRAMS` declarations from the native Windows variables of the host.

WHERE the Linux or Mac mappings are included,
THE SOFTWARE SHALL resolve the `SYSTEM_ROOT`, `SYSTEM_DRIVE`, and
`SYSTEM_PROGRAMS` declarations from fallbacks, without import aliases.
```

## Links

- [Intent](../env-var-intent.md): Why Kraf resolves and projects environment
  variables through a declarative model.
- [Acceptance](./env-var-host.acceptance.md): Acceptance scenarios for
  platform-specific environment-variable mappings.
- [README.md](../../README.md): Declarative model, resolution, and projection of
  environment variables in Kraf.
