---
description: Standards for what a valid review contains, its scope, its findings, and its report.
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
that each finding meets, and the content of its report.

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
- **Unverified candidate**: a candidate finding that meets the rules of
  [Scope](#scope) and [Findings](#findings), except that the review did not
  confirm whether its source applies, because it could not or did not verify it.
- **Dropped candidate**: a candidate finding that the review confirmed does not
  meet a rule of [Scope](#scope) or [Findings](#findings).
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
  change.
- Content that the change relies on but does not modify (e.g. a configuration
  file, a tool, or the review environment) MUST be outside the scope of the
  review, even when the change makes it more relevant.
- Content that the pull request description leaves for a later pull request MUST
  be outside the scope of the review.
- The rules of this section MUST take precedence over every other rule of this
  document.

### Findings

- A finding MUST cite its source, and that source MUST apply: the cited rule
  exists and covers the content, or the described defect occurs with the content
  as written.
- A finding MUST NOT concern a matter that is neither a defect nor defined by a
  rule (e.g. a choice of approach, name, or style).
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

### Report

#### Review Report - Template

```md
# Review report

<the change under review>

## Findings

<finding>

...

<statement that the classification, the normative-language check, and the drop
of failing candidates were applied to every change in the scope>

<unverified and dropped candidates>
```

#### Finding - Template

````md
### <number>. <one-sentence statement of the issue>

- **Location**: <file, line, and column>
- **Source**: <defect | rule violation | repository tool failure |
  <name of an external skill or agent>>
- **Rule or defect**: <link to the rule that the content violates, or statement
  of the defect>
- **Force**: <blocking finding | non-blocking recommendation | strong
  recommendation>
- **Failure scenario**: <the content or input that leads to the wrong result>
- **Suggestion**: <OPTIONAL, the change that fixes the finding, as a diff>

```diff
- <current content>
+ <suggested content>
```
````

#### Candidates - Template

```text
<file>:<line>:<column> <unverified | dropped>: <short reason>
...
```

#### Rules

- A review report MUST follow the
  [review report template](#review-report---template), and each finding MUST
  follow the [finding template](#finding---template).
- A review report MUST contain every finding that meets the rules of this
  document, and only those.
- The findings of a review report MUST NOT depend on a target number of
  findings.
- A review of the same change under the same rules MUST yield the same findings.
- A review report with no findings MUST replace `<finding>` with the line
  `None.`
- A review report MUST NOT list an unverified or a dropped candidate among its
  findings.
- A review report MUST list its unverified and dropped candidates at its end, in
  the [candidates template](#candidates---template), in at most five lines, for
  the person who runs the review.
- The unverified and dropped candidates MUST NOT be posted to a pull request.

#### Examples

````md
### 1. A rule item has a sentence without a normative keyword

- **Location**: docs/standards/EXAMPLE.md:42:3
- **Source**: rule violation
- **Rule or defect**: [Rule language](./DOC_STANDARDS.md#rule-language)
- **Force**: blocking finding
- **Failure scenario**: the sentence "The agent reads the file first." cannot be
  classified as an instruction or a description.
- **Suggestion**:

```diff
- - The agent reads the file first.
+ - Agents MUST read the file first.
```
````

```md
### 2. A function name does not follow the external style guide

- **Location**: src/example.rs:10:8
- **Source**: example-style-skill
- **Rule or defect**: naming recommendation of example-style-skill
- **Force**: non-blocking recommendation
- **Failure scenario**: a reader expects the name to follow the guide.
```

```text
docs/EXAMPLE.md:12:1 unverified: could not run the tool that the finding cites
src/example.rs:30:5 dropped: the defect predates the change
```

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
