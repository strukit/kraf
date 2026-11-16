---
description: Entry point for reviewers, with the sections and documents a review MUST read.
tags: [review, code-review, entry-point]
type: ["review"]
scope: []
language: en
---

# Review

> [!NOTE]
> The keywords follow RFC 2119 and RFC 8174 across three levels of force:
>
> - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
> - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
> - **Optional:** MAY, OPTIONAL
>
> See
> [GENERAL_STANDARDS.md](./docs/standards/GENERAL_STANDARDS.md#normative-language).

</br>

> [!WARNING]
> An agent that has not read every document and section below in full, in its
> current version, MUST abandon the review and MUST tell the user which one it
> did not read.

## Overview

This document is the entry point for a review of a change in this repository. It
lists what an agent reads before it reviews, and the template its report
follows.

## Required Reading

- Agents MUST read in full [AGENTS.md](./AGENTS.md) before the first operation
  of a review, and MUST follow its rules.
- Agents MUST read in full the [Review rules](./AGENTS.md#review) and the
  [Review procedure](./AGENTS.md#review-1) of AGENTS.md, and MUST follow the
  procedure step by step.
- Agents MUST read in full
  [REVIEW_STANDARDS.md](./docs/standards/REVIEW_STANDARDS.md) before the first
  operation of a review.
- Agents MUST write the review report in the
  [Review Report template](./docs/standards/REVIEW_STANDARDS.md#review-report---template),
  each finding in the
  [Finding template](./docs/standards/REVIEW_STANDARDS.md#finding---template),
  and the unverified and dropped candidates in the
  [Candidates template](./docs/standards/REVIEW_STANDARDS.md#candidates---template).

## Links

- [README.md](./README.md): What Kraf is and its vision, and the index of the
  repository's documents, modules and other files.
- [AGENTS.md](./AGENTS.md): Entry point for agent instructions and the rules
  they MUST follow.
- [REVIEW_STANDARDS.md](./docs/standards/REVIEW_STANDARDS.md): Standards for
  what a valid review contains, its scope, its findings, and its report.
