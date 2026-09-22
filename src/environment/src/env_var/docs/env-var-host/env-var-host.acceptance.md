---
title: Environment variables host mappings - Acceptance
summary: Acceptance scenarios for platform-specific environment-variable mappings.
tags: [environment, env-var, acceptance, host, platform, gherkin]
---

## ENV-VAR_HOST-ACC-0001

Name: Every shared canonical key has a Linux mapping

**Links:**

- [ENV-VAR_HOST-REQ-0001](./env-var-host.spec.md#env-var_host-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

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

Name: Every shared canonical key has a macOS mapping

**Links:**

- [ENV-VAR_HOST-REQ-0001](./env-var-host.spec.md#env-var_host-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0001 — Supported-platform mapping completeness

    Scenario: ENV-VAR_HOST-ACC-0002 — Every shared canonical key has a macOS mapping
      Given the shared canonical environment-variable keys
      When the macOS mappings are loaded
      Then every shared canonical key has a macOS mapping
```

---

## ENV-VAR_HOST-ACC-0003

Name: Every shared canonical key has a Windows mapping

**Links:**

- [ENV-VAR_HOST-REQ-0001](./env-var-host.spec.md#env-var_host-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

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
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV-VAR_HOST-REQ-0002 — Session-owned directory sources

    Scenario: ENV-VAR_HOST-ACC-0004 — Home and work-directory declarations use provided values
      Given a supported platform mapping
      When the mapping declares WORKDIR or a HOME family key
      Then the declaration source is provided
```
