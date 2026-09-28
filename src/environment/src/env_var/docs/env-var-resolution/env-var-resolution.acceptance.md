---
title: Environment variables resolution - Acceptance
summary: Acceptance scenarios for environment-variable resolution.
tags: [environment, env-var, acceptance, resolution, gherkin]
---

## ENV-VAR_RESOLUTION-ACC-0001

Name: Import aliases are collected and deduplicated

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0001](./env-var-resolution.spec.md#env-var_resolution-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0001 — Declared import dependency collection

    Scenario: ENV-VAR_RESOLUTION-ACC-0001 — Import aliases are collected and deduplicated
      Given multiple declarations import the alias "SHELL"
      When the software collects import dependencies
      Then "SHELL" is collected once
```

---

## ENV-VAR_RESOLUTION-ACC-0002

Name: Fallback variable reference is collected

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0001](./env-var-resolution.spec.md#env-var_resolution-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0001 — Declared import dependency collection

    Scenario: ENV-VAR_RESOLUTION-ACC-0002 — Fallback variable reference is collected
      Given a declaration fallback references "$HOME"
      When the software collects import dependencies
      Then "HOME" is collected
```

---

## ENV-VAR_RESOLUTION-ACC-0003

Name: First available import alias is selected

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0002](./env-var-resolution.spec.md#env-var_resolution-req-0002)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0002 — First-found resolution

    Scenario: ENV-VAR_RESOLUTION-ACC-0003 — First available import alias is selected
      Given a declaration imports "USER_LANGUAGE" before "LANG"
      And both aliases provide a value
      When the software resolves the declaration
      Then the value of "USER_LANGUAGE" is selected
```

---

## ENV-VAR_RESOLUTION-ACC-0004

Name: Fallback is used when no import alias has a value

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0002](./env-var-resolution.spec.md#env-var_resolution-req-0002)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0002 — First-found resolution

    Scenario: ENV-VAR_RESOLUTION-ACC-0004 — Fallback is used when no import alias has a value
      Given no import alias provides a value
      And the declaration fallback is "C.UTF-8"
      When the software resolves the declaration
      Then the declaration resolves to "C.UTF-8"
```

---

## ENV-VAR_RESOLUTION-ACC-0005

Name: Non-empty path aliases are merged in declaration order

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0003](./env-var-resolution.spec.md#env-var_resolution-req-0003)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0003 — Path merge resolution

    Scenario: ENV-VAR_RESOLUTION-ACC-0005 — Non-empty path aliases are merged in declaration order
      Given a declaration imports "BIN_A" before "BIN_B"
      And "BIN_A" provides "/custom/bin"
      And "BIN_B" provides "/usr/bin"
      When the software resolves the declaration
      Then the resolved value contains "/custom/bin" before "/usr/bin"
```

---

## ENV-VAR_RESOLUTION-ACC-0006

Name: Merge fallback is used when no path is available

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0003](./env-var-resolution.spec.md#env-var_resolution-req-0003)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0003 — Path merge resolution

    Scenario: ENV-VAR_RESOLUTION-ACC-0006 — Merge fallback is used when no path is available
      Given no import alias provides a non-empty path value
      And the declaration fallback is "/usr/share/man"
      When the software resolves the declaration
      Then the declaration resolves to "/usr/share/man"
```

---

## ENV-VAR_RESOLUTION-ACC-0007

Name: Tilde fallback expands from resolved home

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0004](./env-var-resolution.spec.md#env-var_resolution-req-0004)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0004 — Placeholder expansion

    Scenario: ENV-VAR_RESOLUTION-ACC-0007 — Tilde fallback expands from resolved home
      Given "HOME" resolves to "/workspace"
      And a declaration fallback is "~/.cache"
      When the software resolves the declaration
      Then the declaration resolves to "/workspace/.cache"
```

---

## ENV-VAR_RESOLUTION-ACC-0008

Name: Dollar fallback expands from a collected value

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0004](./env-var-resolution.spec.md#env-var_resolution-req-0004)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0004 — Placeholder expansion

    Scenario: ENV-VAR_RESOLUTION-ACC-0008 — Dollar fallback expands from a collected value
      Given "HOME" has the collected value "/workspace"
      And a declaration fallback is "$HOME/.cache"
      When the software resolves the declaration
      Then the declaration resolves to "/workspace/.cache"
```

---

## ENV-VAR_RESOLUTION-ACC-0009

Name: Unresolved placeholder remains literal

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0004](./env-var-resolution.spec.md#env-var_resolution-req-0004)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0004 — Placeholder expansion

    Scenario: ENV-VAR_RESOLUTION-ACC-0009 — Unresolved placeholder remains literal
      Given a declaration fallback is "$UNKNOWN/.cache"
      And "UNKNOWN" has no resolved value
      When the software resolves the declaration
      Then the declaration resolves to "$UNKNOWN/.cache"
```

---

## ENV-VAR_RESOLUTION-ACC-0010

Name: Empty first-found value is selected

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0002](./env-var-resolution.spec.md#env-var_resolution-req-0002)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0002 — First-found resolution

    Scenario: ENV-VAR_RESOLUTION-ACC-0010 — Empty first-found value is selected
      Given a declaration imports "PRIMARY" before "SECONDARY"
      And "PRIMARY" has the collected value ""
      And "SECONDARY" has the collected value "fallback-value"
      When the software resolves the declaration
      Then the declaration resolves to ""
```

---

## ENV-VAR_RESOLUTION-ACC-0011

Name: Provided canonical value precedes fallback

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0005](./env-var-resolution.spec.md#env-var_resolution-req-0005)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0005 — Source precedence and absence

    Scenario: ENV-VAR_RESOLUTION-ACC-0011 — Provided canonical value precedes fallback
      Given a provided declaration has no import aliases
      And its canonical key has the collected value "/custom/tmp"
      And its fallback is "/default/tmp"
      When the software resolves the declaration
      Then the declaration resolves to "/custom/tmp"
```

---

## ENV-VAR_RESOLUTION-ACC-0012

Name: Provided fallback is used when canonical value is absent

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0005](./env-var-resolution.spec.md#env-var_resolution-req-0005)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0005 — Source precedence and absence

    Scenario: ENV-VAR_RESOLUTION-ACC-0012 — Provided fallback is used when canonical value is absent
      Given a provided declaration has no import aliases
      And its canonical key has no collected value
      And its fallback is "/default/tmp"
      When the software resolves the declaration
      Then the declaration resolves to "/default/tmp"
```

---

## ENV-VAR_RESOLUTION-ACC-0013

Name: Host declaration without aliases ignores its canonical collected value

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0005](./env-var-resolution.spec.md#env-var_resolution-req-0005)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0005 — Source precedence and absence

    Scenario: ENV-VAR_RESOLUTION-ACC-0013 — Host declaration without aliases ignores its canonical collected value
      Given a host declaration has no import aliases
      And its canonical key has the collected value "Linux"
      And its fallback is "Darwin"
      When the software resolves the declaration
      Then the declaration resolves to "Darwin"
```

---

## ENV-VAR_RESOLUTION-ACC-0014

Name: Empty path values are ignored during merge

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0003](./env-var-resolution.spec.md#env-var_resolution-req-0003)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0003 — Path merge resolution

    Scenario: ENV-VAR_RESOLUTION-ACC-0014 — Empty path values are ignored during merge
      Given a merge declaration imports "BIN_A" before "BIN_B"
      And "BIN_A" has the collected value ""
      And "BIN_B" has the collected value "/usr/bin"
      When the software resolves the declaration
      Then the declaration resolves to "/usr/bin"
```

---

## ENV-VAR_RESOLUTION-ACC-0015

Name: Merged paths use the current platform separator

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0003](./env-var-resolution.spec.md#env-var_resolution-req-0003)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0003 — Path merge resolution

    Scenario: ENV-VAR_RESOLUTION-ACC-0015 — Merged paths use the current platform separator
      Given a merge declaration has two non-empty path values
      When the software resolves the declaration
      Then its paths are joined with the current platform path separator
      And duplicate path entries are preserved
```

---

## ENV-VAR_RESOLUTION-ACC-0016

Name: Tilde expansion takes precedence and replaces only the first tilde

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0004](./env-var-resolution.spec.md#env-var_resolution-req-0004)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0004 — Placeholder expansion

    Scenario: ENV-VAR_RESOLUTION-ACC-0016 — Tilde expansion takes precedence and replaces only the first tilde
      Given "HOME" has the collected value "/workspace"
      And a declaration fallback is "~/$HOME/~"
      When the software resolves the declaration
      Then the declaration resolves to "/workspace/$HOME/~"
```

---

## ENV-VAR_RESOLUTION-ACC-0017

Name: Invalid dollar syntax remains literal

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0004](./env-var-resolution.spec.md#env-var_resolution-req-0004)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0004 — Placeholder expansion

    Scenario Outline: ENV-VAR_RESOLUTION-ACC-0017 — Invalid dollar syntax remains literal
      Given a declaration fallback is "<fallback>"
      When the software resolves the declaration
      Then the declaration resolves to "<fallback>"

      Examples:
        | fallback       |
        | $              |
        | $/cache        |
        | ${HOME}/cache  |
```

---

## ENV-VAR_RESOLUTION-ACC-0018

Name: Dollar expansion replaces only the first placeholder

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0004](./env-var-resolution.spec.md#env-var_resolution-req-0004)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0004 — Placeholder expansion

    Scenario: ENV-VAR_RESOLUTION-ACC-0018 — Dollar expansion replaces only the first placeholder
      Given "HOME" has the collected value "/workspace"
      And a declaration fallback is "$HOME/$HOME"
      When the software resolves the declaration
      Then the declaration resolves to "/workspace/$HOME"
```

---

## ENV-VAR_RESOLUTION-ACC-0019

Name: Pass-through forwards the first available value

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0006](./env-var-resolution.spec.md#env-var_resolution-req-0006)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0006 — Pass-through resolution

    Scenario: ENV-VAR_RESOLUTION-ACC-0019 — Pass-through forwards the first available value
      Given a pass-through declaration imports "SystemRoot"
      And "SystemRoot" has the collected value "C:\\Windows"
      When the software resolves the declaration
      Then the declaration resolves to "C:\\Windows"
```

---

## ENV-VAR_RESOLUTION-ACC-0020

Name: Pass-through absence resolves to an empty value

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0006](./env-var-resolution.spec.md#env-var_resolution-req-0006)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0006 — Pass-through resolution

    Scenario: ENV-VAR_RESOLUTION-ACC-0020 — Pass-through absence resolves to an empty value
      Given a pass-through declaration imports "SystemRoot"
      And "SystemRoot" has no collected value
      When the software resolves the declaration
      Then the declaration resolves to ""
```

---

## ENV-VAR_RESOLUTION-ACC-0021

Name: Unjoinable merged paths resolve to an empty value

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0003](./env-var-resolution.spec.md#env-var_resolution-req-0003)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0003 — Path merge resolution

    Scenario: ENV-VAR_RESOLUTION-ACC-0021 — Unjoinable merged paths resolve to an empty value
      Given a merge declaration has paths that cannot be joined for the current platform
      When the software resolves the declaration
      Then the declaration resolves to ""
```

---

## ENV-VAR_RESOLUTION-ACC-0022

Name: Missing source and fallback resolve to an empty value

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0005](./env-var-resolution.spec.md#env-var_resolution-req-0005)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0005 — Source precedence and absence

    Scenario: ENV-VAR_RESOLUTION-ACC-0022 — Missing source and fallback resolve to an empty value
      Given a host declaration has no import aliases
      And the declaration has no fallback
      When the software resolves the declaration
      Then the declaration resolves to ""
```

---

## ENV-VAR_RESOLUTION-ACC-0023

Name: Empty provided canonical value precedes fallback

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0005](./env-var-resolution.spec.md#env-var_resolution-req-0005)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0005 — Source precedence and absence

    Scenario: ENV-VAR_RESOLUTION-ACC-0023 — Empty provided canonical value precedes fallback
      Given a provided declaration has no import aliases
      And its canonical key has the collected value ""
      And its fallback is "/default/tmp"
      When the software resolves the declaration
      Then the declaration resolves to ""
```

---

## ENV-VAR_RESOLUTION-ACC-0024

Name: Resolved output preserves declaration and export order

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0007](./env-var-resolution.spec.md#env-var_resolution-req-0007)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0007 — Resolved output order

    Scenario: ENV-VAR_RESOLUTION-ACC-0024 — Resolved output preserves declaration and export order
      Given "HOME_TEMP" resolves to "/workspace/.tmp"
      And its export aliases are "TEMP", "TMPDIR", and "TMP" in that order
      When the software resolves the declaration
      Then the resolved keys are "HOME_TEMP", "TEMP", "TMPDIR", and "TMP" in that order
      And each resolved key has the value "/workspace/.tmp"
```

---

## ENV-VAR_RESOLUTION-ACC-0025

Name: Collected keys are deduplicated and sorted

**Links:**

- [ENV-VAR_RESOLUTION-REQ-0001](./env-var-resolution.spec.md#env-var_resolution-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: ENV-VAR_RESOLUTION-FEAT-0001 — Environment variable resolution

  Rule: ENV-VAR_RESOLUTION-REQ-0001 — Declared import dependency collection

    Scenario: ENV-VAR_RESOLUTION-ACC-0025 — Collected keys are deduplicated and sorted
      Given active mappings declare the import aliases "SHELL", "EDITOR", and "SHELL"
      When the software collects import dependencies
      Then the collected import keys are "EDITOR" and "SHELL" in that order
```
