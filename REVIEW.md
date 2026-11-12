---
description: Review instructions for pull requests in this repository, with the forms and the template of each finding.
tags: [standards, review, pull-requests, findings, report-template]
type: ["standard"]
scope: ["**/*"]
---

# Review instructions

> [!NOTE]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> See
> [GENERAL_STANDARDS.md](./docs/standards/GENERAL_STANDARDS.md#normative-language).

</br>

> [!IMPORTANT]
> This is a standard document. It works like a constitution. Its rules MUST NOT
> be bypassed by ad-hoc exceptions. See
> [DOC_STANDARDS.md](./docs/standards/DOC_STANDARDS.md#standard-documents).

## Overview

This document defines how a reviewer reviews a pull request in this repository:
what it reads first, the forms that a finding takes, and the template of the
report. It applies the review rules of [AGENTS.md](./AGENTS.md) and does not
replace them.

## Reading order

This section defines what a reviewer reads before it reviews.

### Rules

- A reviewer of a pull request in this repository MUST read
  [AGENTS.md](./AGENTS.md) in full before any other document, and MUST then
  follow it, starting with the traversal of [INDEX.md](./INDEX.md) that its
  opening section defines.
- The reviewer MUST NOT skip any section of AGENTS.md, including its Reviews,
  Findings, and Report sections.

## Finding forms

This section defines the forms that a finding takes, one for each source that
the Findings section of AGENTS.md accepts.

### Rules

- Each finding MUST state its source and the content that identifies it,
  according to its source:
  - A rule of a standard document: the rule, by file and section name, with the
    starting line of the content that violates it.
  - A correctness defect: the wrong result or the contradiction with the
    specification, with the file and the starting line.
  - A failure of a declared tool: the tool, as listed in
    [TOOLS.md](./docs/TOOLS.md), and the failure that reached the review, with
    the file and the starting line.
  - A skill or agent defined outside this repository: the skill or agent and
    what it recommends, with the file and the starting line.
- A finding MUST NOT cite a direction of a document that is not a standard
  document as a rule of this repository.
- A comment that does not follow one of these forms MUST NOT be published.

## Report template

This section defines the template of each finding in a review report.

### Template

````md
#### Issue <number, from the review tool>: <title that identifies the issue> [<blocking | strong recommendation, must be fixed | recommendation>]

```md
- **Severity**: <High | Medium | Low>
- **Category**: <category>
- **Source**: <repository rule, declared tool, external skill or agent, or
  correctness defect>
- **Environment**:
  - Skill: <skill name>
  - Version: <version>
  - Harness: <harness>
  - Model: <model>
  - Effort: <effort>
- **File**: <path relative to the repository>
- **Lines**: <start-end>
```

### Problem

<what the issue is and why it matters>

**Broken rule:** <the rule text, quoted, with its file and section; for a
correctness defect, "no explicit rule"; for a declared tool or an external skill
or agent, what it reports>

### Correction

<the correction, in plain words or in pseudocode>

When the content is not code, **Current** and **Suggestion** in a diff block:

```diff
- <current line>
+ <suggested line>
```
````

### Rules

- Every report MUST use the template above for each finding, whether it is
  posted explicitly or implicitly, including any report that is sent under a
  request to send it.
- The level of each finding MUST be one of the values of the template, as the
  Findings section of AGENTS.md defines them.
- A field that the source does not inform, such as Severity, Category, or an
  Environment field, MUST be written as `not informed`.
- Environment MUST be filled for every reviewer, not only for GitHub Copilot.
- The fields MUST keep the order of the template and MUST NOT change their
  names.
- The Correction section MUST describe the correction in plain words or in
  pseudocode, and MUST NOT contain ready-to-run code, as the Report section of
  AGENTS.md defines. A diff block MAY show the current and the suggested text
  when the content is not code.
- A report MUST end with the `Files read` section that the Report section of
  AGENTS.md defines.

## Links

- [INDEX.md](./INDEX.md): Index of the repository's documents, modules and other
  files.
- [AGENTS.md](./AGENTS.md): Agent instructions and the repository standards they
  MUST follow.
- [TOOLS.md](./docs/TOOLS.md): Tools declared in this repository, with their
  purpose, configuration, capabilities, and version.
