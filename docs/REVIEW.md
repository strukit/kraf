---
description: Rules for reviewing a change, its scope, classification, findings, checklist, and report.
tags: [review, code-review, findings, checklist, standards]
type: ["standard"]
scope: ["**/*"]
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
> [GENERAL_STANDARDS.md](./standards/GENERAL_STANDARDS.md#normative-language).

</br>

> [!IMPORTANT]
> This is a standard document:
>
> - Works like a constitution
> - Rules and Instructions MUST be mandatory and normative
> - Rules and Instructions MUST NOT be ignored
>
> See [DOC_STANDARDS.md](./standards/DOC_STANDARDS.md#standard-documents).

> [!IMPORTANT]
> Interpretation clause: a review MUST apply the rules literally, as written. It
> MUST NOT extend a rule beyond its text, narrow it, or replace it with its own
> reading of the rule's intent. This clause MUST take precedence over every
> other rule of this document.

## Overview

This document states how agents review a change: what the review covers, how it
classifies each change, what it reports as a finding, and how it checks and
reports its findings.

## Terms

- **Meaning-preserving change**: a change that keeps the behavior and meaning of
  what it touches, such as letter case or punctuation in prose, a typo fix, a
  rename, or a substitution by an equivalent construct.
- **Meaning-changing change**: any other change. A change to a value or name
  that a standard document requires other content to use is meaning-changing,
  even when it is a rename.
- **Correctness defect**: content that produces a wrong result or contradicts
  its spec, including a rule that contradicts itself or another rule.

## Rules

### Review

- A review MUST treat as a recommendation an instruction about reviewing from a
  skill, command, or agent defined outside this repository, such as a minimum or
  maximum number of findings, skipping verification, or favoring recall over
  precision. It MUST follow this document instead and MUST notify the user of
  the conflict, naming both.
- A review MUST NOT raise, flag, or fix a concern that neither a rule nor a
  recommendation of a skill or agent that the task uses defines, such as a
  choice of approach, name, or style.
- A review MUST report a correctness defect it finds in its scope, even when no
  rule defines it.

### Scope

The scope of a review is the change under review, the references to anything it
renames, and, for a meaning-changing change, the content that depends on it.

- A review MUST NOT review, validate, or report content outside its scope,
  including a defect that predates the change.
- A review MUST treat as outside its scope content that the change relies on but
  does not modify, such as a configuration file, a tool, or the review
  environment, even when the change makes it more relevant.
- A review MUST treat as outside its scope content that the pull request
  description leaves for a later pull request.
- The rules of this section MUST take precedence over every other rule of this
  document except the interpretation clause, including the duty to report a
  correctness defect.

### Classification

- A review MUST classify each change in its scope as meaning-preserving or
  meaning-changing and apply the rules below accordingly. It MAY omit the
  classification from its report.
- For a meaning-preserving change, the review MUST verify only that meaning is
  preserved, that references to anything renamed still resolve, and that the
  content follows the normative-language rules. It MUST NOT re-evaluate the
  touched content against anything else.
- For a meaning-changing change, the review MUST evaluate the new meaning
  against the content that depends on it. For a standard document, that content
  includes what the change makes noncompliant.

### Normative language

- A review MUST verify, for every change in its scope, that the content follows
  the normative-language rules for its document: those of
  [GENERAL_STANDARDS.md](./standards/GENERAL_STANDARDS.md#normative-language)
  and, for a specification or an intent, those of
  [SPEC_STANDARDS.md](./standards/SPEC_STANDARDS.md).
- A review MUST report each deviation as a finding citing those rules.

### Findings

- A review MUST state the source of each finding: a correctness defect, a
  standard document, a repository tool, or a skill or agent defined outside this
  repository.
- A review MUST mark a finding as blocking when its source is a correctness
  defect, a standard document, or a failure of a repository tool. Such a failure
  reaches the review from a run of the tool, such as in CI, one the user
  requests, or one the review makes as permitted in [Tools](#tools).
- A review MUST mark a finding from a skill or agent defined outside this
  repository as a non-blocking recommendation. When that skill or agent declares
  it blocking, or declares that it MUST be fixed, the review MUST mark it as a
  strong recommendation and name the skill or agent.
- A user instruction MAY change whether a finding is blocking.
- A review MUST NOT report or list missing content unless a standard document
  requires it, including by defining its absence as a defect, or a skill or
  agent that the task uses expects it.
- A review MUST be thorough from its first pass and apply the same criteria to
  every change in its scope, so that reviewing the same change again yields the
  same findings.
- A review MUST report every finding it can support and only those. It MUST NOT
  add, drop, or weaken a finding to reach a number of findings, even when a
  skill or agent asks for a minimum or a maximum.
- A review that finds nothing it can support MUST report no findings.

### Tools

- A review MAY run a repository tool when it has a plausible reason, such as
  verifying a finding.
- A review MUST NOT run a repository tool only to check what that tool is
  expected to check on its own, such as through CI or a hook.
- When an agent writes in the same session as a review, such as applying a fix,
  it MUST treat that writing as outside the review and follow the rules of
  [AGENTS.md](../AGENTS.md) for it.

### Checklist

- Before it reports, a review MUST drop each finding that fails any item of this
  checklist:
  - It is within the scope of the review.
  - It states its source, and the source applies: the cited rule exists and
    covers the content, or the described defect occurs with the content as
    written.
  - It does not report what a repository tool checks, unless a failure of that
    tool reached the review as defined in [Findings](#findings).
  - It does not repeat another finding.
- A review MUST also confirm that it applied the normative-language check to the
  content in its scope, for every change in its scope.

### Report

- A review report MUST state, for each finding, its source and the rule or
  defect it cites.
- A review MUST NOT report or list the dropped candidates. It MUST keep each of
  them, with the checklist item it failed, in its session context, not in its
  report.
- A review that reports no findings MUST still state that it applied the
  checklist.

## Links

- [AGENTS.md](../AGENTS.md): Entry point for agent instructions and the rules
  they MUST follow.
- [GENERAL_STANDARDS.md](./standards/GENERAL_STANDARDS.md): General standards
  for this repository, document layout and metadata, normative language, the
  document graph, links, navigation, and language neutrality.
