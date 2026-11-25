---
description: Standards for the content and format of a review report, with its templates and examples.
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

This document defines the content and format of a review report: the findings
that it contains, the templates that it follows, and how it presents its
findings.

## Terms

- **[Finding](./REVIEW_STANDARDS.md#terms)**,
  **[Unverified candidate](./REVIEW_STANDARDS.md#terms)**,
  **[Dropped candidate](./REVIEW_STANDARDS.md#terms)**, and
  **[Force of a finding](./REVIEW_STANDARDS.md#terms)**: defined in
  [REVIEW_STANDARDS.md](./REVIEW_STANDARDS.md#terms).
- **Reviewer block**: the YAML block that follows the title of the
  [review report template](#review-report---template), with the model and the
  agent that ran the review.

## Rules

### Report Content

This section defines which findings a review report contains, and how agents
report the candidates that it leaves out.

#### Rules

- A review report MUST contain every finding that meets the rules of
  [REVIEW_STANDARDS.md](./REVIEW_STANDARDS.md), and only those.
- The findings of a review report MUST NOT depend on a target number of
  findings.
- A review of the same change under the same rules MUST yield the same findings.
- A review report MUST state, for each finding, its source and the rule or
  defect it cites.
- A review report MUST NOT list an unverified or a dropped candidate among its
  findings.
- Agents MUST keep each unverified and dropped candidate, with its reason and
  the rule it was checked against, in their session context.
- Agents MUST list each unverified and dropped candidate to the user in the
  reply that reports the review, after the review report and outside it, without
  waiting for the user to ask. Each entry MUST state the candidate, its status
  (`unverified` or `dropped`), the rule or defect it was checked against, why it
  is not a finding, what would settle it, and how to reproduce the check: the
  command, file, or steps that the user can run to repeat it.
- When a review has no reply to a user, its unverified and dropped candidates
  MUST be reported in the context where the review runs, such as in the output
  of the command that runs it, and MUST NOT be added to the review report.

### Report Templates

#### Review Report - Template

````md
# Review

```yaml
agent: <name of the agent harness>
model: <name of the model> (<full identifier of model>)
```

## Findings

<one - `Finding NN: title` section per finding, in order of severity, or, in a
review posted to a pull request, only its title as a link to its comment>
````

#### Finding - Template

````md
### Finding <number>: <one-sentence statement of the issue>

- **Location**: [<file>:<line or range of lines>](<link to the file at that line
  or range of lines, e.g. `#L10` or `#L10-L12`>)
- **Source**: <defect | rule violation | repository tool failure |
  <name of an external skill or agent>>
- **Original code**: <OPTIONAL, the code or category that the repository tool,
  skill, or agent reports for the issue, e.g. `correctness`>
- **Rule or defect**: <link to the rule that the content violates, or statement
  of the defect>
- **Force**: <blocking finding | non-blocking recommendation | strong
  recommendation>
- **Failure scenario**: <the content or input that leads to the wrong result>

<OPTIONAL, the block below, with the change that fixes the finding: a
`suggestion` block in an inline review comment, or a `diff` block elsewhere>

```suggestion
<content that replaces the lines that the inline review comment is attached to>
```
````

#### Rules

- A review report MUST follow the
  [review report template](#review-report---template), and each finding MUST
  follow the [finding template](#finding---template).
- A review report MUST state, in its reviewer block, the model and the agent
  that ran the review.
- A developer who uses an agent in this repository MUST accept that the review
  reports it produces share the model and the agent, because sharing them
  encourages good practices and a fair environment.
- A review report or review comment, produced by an agent, that lacks the
  reviewer block MUST be discarded.
- An inline review comment that belongs to a review whose body holds the review
  report, with its reviewer block, MUST NOT repeat the reviewer block, and MUST
  NOT be discarded for lacking it.
- The rule that requires the reviewer block MUST override the rule of
  [Agent attribution](../../AGENTS.md#agent-attribution) that forbids a line
  that credits an agent, a model, or the tool that runs it, and MUST override it
  only for the reviewer block of a review report.
- An agent that produces a review report for a pull request MUST inform the user
  that the report shares the model and the agent, and that a review report or
  review comment that lacks the reviewer block is discarded.
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
- A finding MUST state its location as a Markdown link to the file at that line
  or range of lines, relative to the root of the repository.
- A finding posted to a pull request MUST instead link its location to the file
  at the reviewed commit (e.g.
  `https://github.com/<owner>/<repo>/blob/<commit>/<file>#L<line>` or
  `#L<start>-L<end>`).
- A review report posted to a pull request MUST post each finding as an inline
  review comment attached to the file and the line of its location, or to its
  range of lines, and MUST list in the report only the title of each finding, as
  a link to its comment, which is either the inline comment or its comment in
  the discussion of the pull request.
- A finding posted as an inline review comment MUST state its suggestion, when
  it has one, as a `suggestion` block whose content replaces exactly the lines
  that the comment is attached to, and MUST use a `diff` block only when the
  change cannot be applied to those lines.
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
  `## Findings`, with the line `None.`
- The unverified and dropped candidates MUST NOT be posted to a pull request.
- Agents MUST post to a pull request only the review report and its findings,
  and MUST NOT post notes, limitations, or other content outside that report.
- Agents MUST check each comment that they are about to post to a pull request
  against the two rules above before posting it.

#### Examples

```md
### Finding 01: A loop skips the last element of the list

- **Location**: [src/example.rs:10-12](src/example.rs#L10-L12)
- **Source**: skill: code-review
- **Original code**: `correctness`
- **Rule or defect**: the loop bound excludes the last index
- **Force**: non-blocking recommendation
- **Failure scenario**: a list of three items yields only the first two.
```

## Links

- [README.md](./README.md): Index of the standards that govern this repository,
  to help people and agents navigate it on their own through the document graph.
- [REVIEW_STANDARDS.md](./REVIEW_STANDARDS.md): Standards for what a valid
  review contains, its scope, and its findings.
