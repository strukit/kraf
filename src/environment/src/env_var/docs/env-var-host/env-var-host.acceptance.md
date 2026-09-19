---
title: Environment variables host mappings - Acceptance
summary: Acceptance scenarios for platform-specific environment-variable mappings.
tags: [environment, env-var, acceptance, host, platform, gherkin]
status: draft
---

## ENV_VAR_HOST-ACC-0001

Name: Every shared canonical key has a Linux mapping

**Links:**

- [ENV_VAR_HOST-REQ-0001](./env-var-host.spec.md#env_var_host-req-0001)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV_VAR_HOST-REQ-0001 — Supported-platform mapping completeness

    Scenario: ENV_VAR_HOST-ACC-0001 — Every shared canonical key has a Linux mapping
      Given the shared canonical environment-variable keys
      When the Linux mappings are loaded
      Then every shared canonical key has a Linux mapping
```

---

## ENV_VAR_HOST-ACC-0002

Name: Every shared canonical key has a macOS mapping

**Links:**

- [ENV_VAR_HOST-REQ-0001](./env-var-host.spec.md#env_var_host-req-0001)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV_VAR_HOST-REQ-0001 — Supported-platform mapping completeness

    Scenario: ENV_VAR_HOST-ACC-0002 — Every shared canonical key has a macOS mapping
      Given the shared canonical environment-variable keys
      When the macOS mappings are loaded
      Then every shared canonical key has a macOS mapping
```

---

## ENV_VAR_HOST-ACC-0003

Name: Every shared canonical key has a Windows mapping

**Links:**

- [ENV_VAR_HOST-REQ-0001](./env-var-host.spec.md#env_var_host-req-0001)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV_VAR_HOST-REQ-0001 — Supported-platform mapping completeness

    Scenario: ENV_VAR_HOST-ACC-0003 — Every shared canonical key has a Windows mapping
      Given the shared canonical environment-variable keys
      When the Windows mappings are loaded
      Then every shared canonical key has a Windows mapping
```

---

## ENV_VAR_HOST-ACC-0004

Name: Home and work-directory declarations use provided values

**Links:**

- [ENV_VAR_HOST-REQ-0002](./env-var-host.spec.md#env_var_host-req-0002)

**Status**:

- Acceptance status: Draft
- Verification status: Not verified

```gherkin
Feature: ENV_VAR_HOST-FEAT-0001 — Environment variable host mappings

  Rule: ENV_VAR_HOST-REQ-0002 — Session-owned directory sources

    Scenario: ENV_VAR_HOST-ACC-0004 — Home and work-directory declarations use provided values
      Given a supported platform mapping
      When the mapping declares WORKDIR or a HOME family key
      Then the declaration source is provided
```
