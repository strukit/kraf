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
follows, and how it presents its findings. What a report contains is defined in
[REVIEW_STANDARDS.md](./REVIEW_STANDARDS.md#report).

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

<finding, or, in a review posted to a pull request, its title only>

...

<statement that the classification, the normative-language check, and the drop
of failing candidates were applied to every change in the scope>
```

#### Finding - Template

````md
### Finding <number>: <one-sentence statement of the issue>

- **Location**: [<file>:<line>:<column>](<link to the file at that line>)
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

#### Rules

- A review report MUST follow the
  [review report template](#review-report---template), and each finding MUST
  follow the [finding template](#finding---template).
- Each finding MUST be numbered in the order of the review report.
- A finding MUST state its location as a Markdown link to the file at that line,
  relative to the root of the repository.
- A finding posted to a pull request MUST instead link its location to the file
  at the reviewed commit (e.g.
  `https://github.com/<owner>/<repo>/blob/<commit>/<file>#L<line>`).
- A review report posted to a pull request MUST post each finding as an inline
  review comment attached to the file and the line of its location, or to its
  range of lines, and MUST list in the report only the title of each finding, as
  a link to its inline comment.
- A review report posted to a pull request MUST use an available tool that posts
  inline review comments, when there is one, such as the review tools of the
  hosting service or its command-line client.
- A review report with no findings MUST replace the finding placeholder under
  `## Findings`, and the `...` that follows it, with the line `None.`
- The unverified and dropped candidates MUST NOT be posted to a pull request.

#### Examples

````md
### Finding 01: A rule item has a sentence without a normative keyword

- **Location**: [docs/standards/EXAMPLE.md:42:3](docs/standards/EXAMPLE.md#L42)
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
### Finding 02: A function name does not follow the external style guide

- **Location**: [src/example.rs:10:8](src/example.rs#L10)
- **Source**: example-style-skill
- **Rule or defect**: naming recommendation of example-style-skill
- **Force**: non-blocking recommendation
- **Failure scenario**: a reader expects the name to follow the guide.
```

## Links

- [README.md](./README.md): Index of the standards that govern this repository,
  to help people and agents navigate it on their own through the document graph.
- [REVIEW_STANDARDS.md](./REVIEW_STANDARDS.md): Standards for what a valid
  review contains, its scope, its findings, and its report.
