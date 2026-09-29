---
title: Example area - Acceptance
summary: Illustrative acceptance scenario used to demonstrate the acceptance format.
tags: [example, acceptance, gherkin]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
>
> This document MUST use these terms, in uppercase, when expressing normative
> requirements, permissions, recommendations, or prohibitions. It MUST NOT use
> alternative words or lowercase variants as normative keywords.

## EXAMPLE_AREA-ACC-0001

Name: Unknown configuration key is rejected

**Links:**

- [EXAMPLE_AREA-REQ-0001](./example.spec.md#example_area-req-0001)

**Status**:

- Doc status: Draft
- Verification status: Unknown

```gherkin
Feature: EXAMPLE_AREA-FEAT-0001 — Explicit configuration

  Rule: EXAMPLE_AREA-REQ-0001 — Explicit configuration

    Scenario: EXAMPLE_AREA-ACC-0001 — Unknown configuration key is rejected
      Given no declaration exists for "EXAMPLE_KEY"
      When configuration provides a value for "EXAMPLE_KEY"
      Then the software rejects the configuration
```
