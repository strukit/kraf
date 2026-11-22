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
- **Reviewer block**: the YAML block that opens the
  [review report template](#review-report---template), with the model, the
  reasoning effort, and the agent that ran the review.

## Rules

### Report Templates

#### Review Report - Template

````md
```yaml
model: <full identifier of the model: name, version, and date or revision>
effort: <low | medium | high | extra-high | max | unknown>
agent: <name of the harness or command-line client that ran the review>
```

# Review report

## Findings

<finding, or, in a review posted to a pull request, its title only>

...

<statement that the classification, the normative-language check, and the drop
of failing candidates were applied to every change in the scope>
````

#### Finding - Template

````md
### Finding <number>: <one-sentence statement of the issue>

- **Location**: [<file>:<line>:<column>](<link to the file at that line>)
- **Source**: <defect | rule violation | repository tool failure |
  <name of an external skill or agent>>
- **Original code**: <OPTIONAL, the code or category that the repository tool,
  skill, or agent reports for the issue, e.g. `correctness`>
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
- A review report MUST state, in its reviewer block, the model, the reasoning
  effort, and the agent that ran the review, and MUST write `unknown` for the
  effort when the agent cannot determine it.
- A developer who uses an agent in this repository MUST accept that the review
  reports it produces share the model, the reasoning effort, and the agent,
  because sharing them encourages good practices and a fair environment.
- A review report or review comment, produced by an agent, that lacks the
  reviewer block MUST be discarded.
- The rule that requires the reviewer block MUST override the rule of
  [Agent attribution](../../AGENTS.md#agent-attribution) that forbids a line
  that credits an agent, a model, or the tool that runs it, and MUST override it
  only for the reviewer block of a review report.
- An agent that produces a review report for a pull request MUST inform the user
  that the report shares the model, the reasoning effort, and the agent, and
  that a review report or review comment that lacks the reviewer block is
  discarded.
- An agent MUST NOT omit that notice, unless the user has expressly instructed
  it to stop giving it, and that user instruction MUST then prevail.
- Each finding MUST be numbered in the order of the review report.
- A finding whose source is a repository tool, or a skill or agent, MUST state,
  in every place of the review where it appears, the code or category that the
  tool, skill, or agent reports for it, and MAY omit the original code field
  when the source reports none.
- A review report MUST follow the templates of this document when it is posted,
  and MUST NOT follow the format of the tool, skill, or agent that produced a
  finding.
- A finding MUST state its location as a Markdown link to the file at that line,
  relative to the root of the repository.
- A finding posted to a pull request MUST instead link its location to the file
  at the reviewed commit (e.g.
  `https://github.com/<owner>/<repo>/blob/<commit>/<file>#L<line>`).
- A review report posted to a pull request MUST post each finding as an inline
  review comment attached to the file and the line of its location, or to its
  range of lines, and MUST list in the report only the title of each finding, as
  a link to its inline comment.
- A review posted to a pull request MUST read, before it posts a finding, every
  review thread on the location of that finding, including resolved ones.
- A finding whose issue an open thread already discusses MUST be posted as a
  reply in that thread, with its justification against what the thread states.
- A finding whose issue a resolved thread already discusses MUST be posted as a
  new inline comment that links to that thread and justifies why its outcome
  does not hold, and the review MUST NOT reopen the thread.
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
### Finding 02: A loop skips the last element of the list

- **Location**: [src/example.rs:10:8](src/example.rs#L10)
- **Source**: code-review skill
- **Original code**: `correctness`
- **Rule or defect**: the loop bound excludes the last index
- **Force**: non-blocking recommendation
- **Failure scenario**: a list of three items yields only the first two.
```

## Links

- [README.md](./README.md): Index of the standards that govern this repository,
  to help people and agents navigate it on their own through the document graph.
- [REVIEW_STANDARDS.md](./REVIEW_STANDARDS.md): Standards for what a valid
  review contains, its scope, its findings, and its report.
