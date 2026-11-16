---
description: Standards for what constitutes a valid review, its scope, classification, findings, and report format.
tags: [review, code-review, findings, checklist, standards]
type: ["standard"]
scope: ["**/*"]
language: en
---

# Review Standards

> [!NOTE]
> The keywords follow RFC 2119 and RFC 8174 across three levels of force:
>
> - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
> - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
> - **Optional:** MAY, OPTIONAL
>
> See
> [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#normative-language).

</br>

> [!IMPORTANT]
> This is a standard document:
>
> - Works like a constitution
> - Rules and Instructions MUST be mandatory and normative
> - Rules and Instructions MUST NOT be ignored
>
> See [DOC_STANDARDS.md](./DOC_STANDARDS.md#standard-documents).
## Overview

This document defines what constitutes a valid review: its scope, what it
reports as a finding, the criteria findings must meet, and how it structures its
report.

## Terms

- **Behavior-preserving change**: a change that does not alter the result or
  effect of what it touches, such as replacing a method with another that does
  the same thing, renaming, fixing a typo, or refactoring code.
- **Behavior-changing change**: a change that alters the result or effect, such
  as changing a value, modifying logic, or adding or removing functionality.
- **Defect**: content that produces a wrong result, contradicts its spec, or
  violates what was specified or promised.
- **Rule violation**: a defect where a rule contradicts another rule or an
  implementation contradicts its specification.

## Rules

### Review

- A review MUST NOT report a concern that is not defined by a rule or a
  standard of the repository.
- A rule violation MUST be considered a valid finding.

### Scope

The scope of a review is the change under review, the references to anything it
renames, and, for a behavior-changing change, the content that depends on it.

- A review MUST NOT review, validate, or report content outside its scope,
  including a defect that predates the change.
- A review MUST treat as outside its scope content that the change relies on but
  does not modify, such as a configuration file, a tool, or the review
  environment, even when the change makes it more relevant.
- A review MUST treat as outside its scope content that the pull request
  description leaves for a later pull request.
- The rules of this section MUST take precedence over every other rule of this
  document.

### Findings

- A finding MUST state its source: a defect, a standard document,
  or a repository tool.
- A finding MUST be blocking when its source is a defect, a standard
  document, or a failure of a repository tool.
- A review MUST NOT report or list missing content unless a standard document
  requires it by defining its absence as a defect.
- A review MUST be thorough from its first pass and apply the same criteria to
  every change in its scope, so that reviewing the same change again yields the
  same findings.
- A review MUST report every finding it can support and only those. It MUST NOT
  add, drop, or weaken a finding.
- A review that finds nothing it can support MUST report no findings.

### Report

- A review report MUST state, for each finding, its source and the rule or
  defect it cites.

## Links

- [AGENTS.md](../AGENTS.md): Entry point for agent instructions and the rules
  they MUST follow.
- [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md): General standards
  for this repository, document layout and metadata, normative language, the
  document graph, links, navigation, and language neutrality.
- [DOC_STANDARDS.md](./DOC_STANDARDS.md): Standards for standard documents,
  precedence between rules, and rule language.
- [SPEC_STANDARDS.md](./SPEC_STANDARDS.md): Standards for documenting feature
  intent, requirements, and acceptance scenarios.
