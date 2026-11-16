---
description: Standards for the format of a review report, with its templates and examples.
tags: [review, report, findings, templates, standards]
type: ["standard"]
scope: ["**/*"]
task: ["review"]
language: en
---

# Review Report Standards

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

This document defines the format of a review report: the templates that it
follows, and how it presents its findings and candidates. What a report contains
is defined in [REVIEW_STANDARDS.md](./REVIEW_STANDARDS.md#report).

## Terms

- **[Finding](./REVIEW_STANDARDS.md#terms)**,
  **[Unverified candidate](./REVIEW_STANDARDS.md#terms)**,
  **[Dropped candidate](./REVIEW_STANDARDS.md#terms)**, and
  **[Force of a finding](./REVIEW_STANDARDS.md#terms)**: defined in
  [REVIEW_STANDARDS.md](./REVIEW_STANDARDS.md#terms).

## Rules

### Report Templates

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
- A review report with no findings MUST replace `<finding>` with the line
  `None.`
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
- [REVIEW_STANDARDS.md](./REVIEW_STANDARDS.md): Standards for what a valid
  review contains, its scope, its findings, and its report.
