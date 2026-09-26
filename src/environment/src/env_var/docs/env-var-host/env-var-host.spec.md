---
title: Environment variables host mappings - SPEC
summary: Requirements for platform-specific environment-variable mappings.
tags: [environment, env-var, specification, spec, host, platform]
---

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
The software SHALL declare every shared canonical environment-variable key in
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
The software SHALL resolve the WORKDIR and HOME declaration families from
provided values rather than from values imported from the host.
```
