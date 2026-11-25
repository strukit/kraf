---
description: Standards for what a valid review contains, its scope, and its findings.
tags: [review, code-review, findings, standards]
type: ["standard"]
scope: ["**/*"]
task: ["review"]
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
> See [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#normative-language).

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

This document defines what a valid review contains: its scope, the conditions
that each finding meets, and the procedure that agents follow to perform it. The
content and format of its report are defined in
[REVIEW_REPORT_STANDARDS.md](./REVIEW_REPORT_STANDARDS.md).

## Terms

- **Behavior-preserving change**: a change that does not alter the result or
  effect of what it touches (e.g. replacing a method with another that does the
  same thing, fixing a typo, or renaming).
- **Behavior-changing change**: any other change. A change to a value or name
  that a standard document requires other content to use is behavior-changing,
  even when it is a rename.
- **Defect**: content that produces a wrong result or contradicts its
  specification, including a rule that contradicts itself or another rule.
- **Rule violation**: content that does not follow a rule that applies to it.
- **Scope of a review**: the change under review, the references to anything it
  renames, and, for a behavior-changing change, the content that depends on it.
- **Finding**: an issue that a review report states, whose source the review
  confirmed against the content as written.
- **Unverified candidate**: a candidate finding that has a source, a rule or a
  defect that could apply to the content, and that meets the rules of
  [Scope](#scope) and [Findings](#findings), except that the review could not
  confirm whether that source applies to the content as written.
- **Dropped candidate**: a candidate finding that the review confirmed does not
  meet a rule of [Scope](#scope) or [Findings](#findings), that has no source:
  no rule of a standard document covers the content, and the content has no
  defect as written, or that predates the change.
- **Force of a finding**: whether a finding blocks the change: a blocking
  finding, a non-blocking recommendation, or a strong recommendation.
- **Blocking finding**: a finding that blocks the change until it is fixed.
- **Non-blocking recommendation**: a finding whose source is a skill or agent
  defined outside this repository, and that does not block the change.
- **Strong recommendation**: a finding whose source is a skill or agent defined
  outside this repository that declares it blocking or declares that it MUST be
  fixed. It can block the change until the team or the human reviewers evaluate
  it.
- **Source of a finding**: what a finding cites: a defect, a rule violation, a
  failure of a repository tool, or a skill or agent defined outside this
  repository.

## Rules

### Scope

- A finding MUST concern content within the scope of the review.
- A finding MUST NOT concern a defect or a rule violation that predates the
  change. Such a defect or violation MUST be a dropped candidate, with the
  reason that it predates the change.
- A rule that a change adds or changes MUST apply to all content that it covers,
  including content that the change does not modify. A violation of that rule in
  such content MUST be within the scope of the review, and the exclusion of
  defects and violations that predate the change MUST NOT apply to it.
- Content that the change relies on but does not modify (e.g. a configuration
  file, a tool, or the review environment) MUST be outside the scope of the
  review, even when the change makes it more relevant.
- Content that the pull request description leaves for a later pull request MUST
  be outside the scope of the review.
- The rules of this section MUST take precedence over any instruction of a skill
  or agent that this repository does not declare as a standard document, and
  MUST NOT take precedence over an explicit user instruction.

### Findings

- A finding MUST cite its source, and that source MUST apply: the cited rule
  exists and covers the content, or the described defect occurs with the content
  as written.
- A finding MUST NOT concern a matter that is neither a defect nor defined by a
  rule (e.g. a choice of approach, name, or style).
- A candidate that has no source MUST be a dropped candidate, with the reason
  that it has no source, and MUST NOT be an unverified candidate.
- When no standard document or specification states the expected result of a
  candidate whose source is a defect, the review MUST reproduce the candidate
  with a check that the user can run, where the review can run one. A candidate
  whose reproduction confirms the defect MUST be a finding. A candidate whose
  reproduction does not confirm it MUST be a dropped candidate. When the review
  cannot run the check, the candidate MUST be an unverified candidate, with the
  check that would settle it.
- A finding about a behavior-preserving change MUST concern only whether the
  behavior is preserved, whether references to anything renamed still resolve,
  or whether the content follows the normative-language rules.
- A finding about a behavior-changing change MAY concern the new behavior
  against the content that depends on it, including, for a standard document,
  what the change makes noncompliant.
- A deviation from the normative-language rules of
  [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#normative-language) and, for a
  specification or an intent, of [SPEC_STANDARDS.md](./SPEC_STANDARDS.md), in
  content within the scope of the review, MUST be a finding.
- A finding about missing content MUST cite a standard document that requires
  that content, including by defining its absence as a defect, or a skill or
  agent that the task uses and that expects it.
- A finding MUST NOT state what a repository tool checks, unless a failure of
  that tool reached the review (e.g. from CI, from a run the user requests, or
  from a run made during the review).
- A finding MUST NOT repeat another finding.
- A finding MUST be a blocking finding when its source is a defect, a rule
  violation, or a failure of a repository tool.
- A finding whose source is a skill or agent defined outside this repository
  MUST meet the other rules of this document, and, when it meets them, MUST be
  reported.
- Such a finding MUST be a non-blocking recommendation, unless it is a strong
  recommendation, which MUST name the skill or agent.
- A strong recommendation MAY block the change, and the team or the human
  reviewers of the change MUST then decide whether it stands.

### Agents

- Agents MUST apply the rules of this document literally, as written, when they
  review a change, and MUST NOT extend a rule beyond its text, narrow it, or
  replace it with their own reading of its intent.
- Agents MAY run a repository tool during a review when they have a plausible
  reason, such as verifying a finding.
- Agents MUST NOT run a repository tool during a review only to check what that
  tool is expected to check on its own, such as through CI or a hook.

### Procedure

- Agents MUST follow this procedure when the task is a review.

1. Agents MUST determine the scope of the review.
2. Agents MUST find and read the standard documents of the review, as
   [AGENTS.md](../../AGENTS.md#review) requires.
3. Agents MUST classify each change in the scope as behavior-preserving or
   behavior-changing.
4. Agents MUST check, for each change in the scope, that its content follows the
   normative-language rules that apply to its document.
5. Agents MUST collect the candidate findings, MUST drop each one that fails a
   rule of this document, and MUST try to verify each remaining one against the
   content as written.
6. Agents MUST report the findings as the standard documents found in step 2
   require, and MUST list the unverified and dropped candidates to the user, as
   the [Report Content](./REVIEW_REPORT_STANDARDS.md#report-content) section of
   REVIEW_REPORT_STANDARDS.md requires.

## Links

- [README.md](./README.md): Index of the standards that govern this repository,
  to help people and agents navigate it on their own through the document graph.
- [AGENTS.md](../../AGENTS.md): Entry point for agent instructions and the rules
  they MUST follow.
- [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md): General standards for this
  repository, document layout and metadata, normative language, the document
  graph, links, navigation, and language neutrality.
- [DOC_STANDARDS.md](./DOC_STANDARDS.md): Standards for standard documents: what
  they are, their layout, precedence between rules, and how they word their
  rules.
- [SPEC_STANDARDS.md](./SPEC_STANDARDS.md): Standards for documenting feature
  intent, requirements, and acceptance scenarios.
- [REVIEW_REPORT_STANDARDS.md](./REVIEW_REPORT_STANDARDS.md): Standards for the
  content and format of a review report, with its templates and examples.
