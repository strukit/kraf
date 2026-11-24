---
description: Acceptance scenarios for platform-specific environment-variable mappings.
tags: [environment, env-var, acceptance, host, platform, gherkin]
type: ["acceptance"]
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

## ENV-VAR_HOST-ACC-0001

Name: Every shared canonical key has a Linux mapping

**Links:**

- [ENV-VAR_HOST-REQ-0001](./env-var-host.spec.md#env-var_host-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Meets

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0001 — Supported-platform mapping completeness

    Scenario: ENV-VAR_HOST-ACC-0001 — Every shared canonical key has a Linux mapping
      Given the shared canonical environment-variable keys
      When the Linux mappings are loaded
      Then every shared canonical key has a Linux mapping
```

---

## ENV-VAR_HOST-ACC-0002

Name: Every shared canonical key has a Mac mapping

**Links:**

- [ENV-VAR_HOST-REQ-0001](./env-var-host.spec.md#env-var_host-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Meets

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0001 — Supported-platform mapping completeness

    Scenario: ENV-VAR_HOST-ACC-0002 — Every shared canonical key has a Mac mapping
      Given the shared canonical environment-variable keys
      When the Mac mappings are loaded
      Then every shared canonical key has a Mac mapping
```

---

## ENV-VAR_HOST-ACC-0003

Name: Every shared canonical key has a Windows mapping

**Links:**

- [ENV-VAR_HOST-REQ-0001](./env-var-host.spec.md#env-var_host-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Meets

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0001 — Supported-platform mapping completeness

    Scenario: ENV-VAR_HOST-ACC-0003 — Every shared canonical key has a Windows mapping
      Given the shared canonical environment-variable keys
      When the Windows mappings are loaded
      Then every shared canonical key has a Windows mapping
```

---

## ENV-VAR_HOST-ACC-0004

Name: Home and work-directory declarations use provided values

**Links:**

- [ENV-VAR_HOST-REQ-0002](./env-var-host.spec.md#env-var_host-req-0002)

**Status**:

- Doc status: Draft
- Verification status: Meets

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0002 — Session-owned directory sources

    Scenario: ENV-VAR_HOST-ACC-0004 — Home and work-directory declarations use provided values
      Given a supported platform mapping
      When the mapping declares HOST_WORKDIR or a HOME_DIR family key
      Then the declaration source is provided
```

---

## ENV-VAR_HOST-ACC-0005

Name: Host home is imported and not exported

**Links:**

- [ENV-VAR_HOST-REQ-0003](./env-var-host.spec.md#env-var_host-req-0003)

**Status**:

- Doc status: Draft
- Verification status: Meets

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0003 — Host home import

    Scenario: ENV-VAR_HOST-ACC-0005 — Host home is imported and not exported
      Given a supported platform mapping
      When the mapping declares "HOST_HOME"
      Then the declaration source is host
      And the declaration imports the home directory of the host user
      And the declaration has no export aliases
```

---

## ENV-VAR_HOST-ACC-0006

Name: Kraf canonical keys stay out of the native projection

**Links:**

- [ENV-VAR_HOST-REQ-0004](./env-var-host.spec.md#env-var_host-req-0004)

**Status**:

- Doc status: Draft
- Verification status: Meets

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0004 — Native export aliases only

    Scenario: ENV-VAR_HOST-ACC-0006 — Kraf canonical keys stay out of the native projection
      Given a supported platform mapping
      When the mappings are projected
      Then no projected key is a canonical key of the PLATFORM, TERMINAL, USER, or HOME families
```

---

## ENV-VAR_HOST-ACC-0007

Name: Session path does not import the host path

**Links:**

- [ENV-VAR_HOST-REQ-0005](./env-var-host.spec.md#env-var_host-req-0005)

**Status**:

- Doc status: Draft
- Verification status: Meets

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0005 — Host path lists and isolated session path

    Scenario: ENV-VAR_HOST-ACC-0007 — Session path does not import the host path
      Given a supported platform mapping
      And the host "PATH" is "/opt/tools/bin:/usr/bin"
      When the software resolves "HOME_PATH"
      Then "HOME_PATH" does not contain "/opt/tools/bin"
      And "HOST_PATH" resolves to "/opt/tools/bin:/usr/bin"
```

---

## ENV-VAR_HOST-ACC-0008

Name: Undefined host data and config lists are not projected

**Links:**

- [ENV-VAR_HOST-REQ-0005](./env-var-host.spec.md#env-var_host-req-0005)

**Status**:

- Doc status: Draft
- Verification status: Meets

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0005 — Host path lists and isolated session path

    Scenario: ENV-VAR_HOST-ACC-0008 — Undefined host data and config lists are not projected
      Given a supported platform mapping
      And the host defines no import alias of "HOST_DATA" or "HOST_CONFIG"
      When the software resolves and projects the mappings
      Then the projection contains no export alias of "HOST_DATA" or "HOST_CONFIG"
```

---

## ENV-VAR_HOST-ACC-0009

Name: Windows system variables pass through only on Windows

**Links:**

- [ENV-VAR_HOST-REQ-0006](./env-var-host.spec.md#env-var_host-req-0006)

**Status**:

- Doc status: Draft
- Verification status: Meets

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0006 — Windows system variables

    Scenario: ENV-VAR_HOST-ACC-0009 — Windows system variables pass through only on Windows
      Given the supported platform mappings
      When the mappings are loaded
      Then the Windows mapping declares "SystemRoot", "windir", and "SystemDrive"
      And each of them uses the pass-through import mode
      And no other platform mapping declares them
```

---

## ENV-VAR_HOST-ACC-0010

Name: Only pass-through declarations export their canonical key

**Links:**

- [ENV-VAR_HOST-REQ-0004](./env-var-host.spec.md#env-var_host-req-0004)

**Status**:

- Doc status: Draft
- Verification status: Meets

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0004 — Native export aliases only

    Scenario: ENV-VAR_HOST-ACC-0010 — Only pass-through declarations export their canonical key
      Given a supported platform mapping with a pass-through declaration
      When the mappings are projected
      Then the pass-through declaration is projected under its canonical key
      And no canonical key of another declaration is projected
```

## Links

- [Specification](./env-var-host.spec.md): Requirements for platform-specific
  environment-variable mappings.
- [README.md](../../README.md): Declarative model, resolution, and projection of
  environment variables in Kraf.
