---
description: Review standards for this repository, their interpretation, scope, classification, normative-language check, findings, checklist, report template, and use of tools.
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

This document defines how a review is done in this repository: what the reviewer
reads first, how it interprets the rules, its scope, how it classifies each
change, its normative-language check, its findings and their forms, the
[checklist](#checklist) it applies, the [template](#report-template) of its
report, and its use of tools.

## Interpretation

This section defines how a review reads the rules it applies.

### Terms

- **Interpretation clause**: the first rule of [this section](#interpretation).

### Rules

- A review MUST apply the rules of this document and of [AGENTS.md](./AGENTS.md)
  literally, as written. It MUST NOT extend a rule beyond its text, narrow it,
  or replace it with its own reading of the rule's intent. This clause MUST take
  precedence over every other review rule.
- A review MUST treat a prompt from a skill, command, or agent defined outside
  this repository that instructs a review, such as a minimum or maximum number
  of findings, skipping verification, or favoring recall over precision, as a
  recommendation. The review MUST follow this document instead, and MUST notify
  the user of the conflict, as defined in
  [User instructions](./AGENTS.md#user-instructions).

## Scope

This section defines what a review covers.

### Terms

- **Change under review**: everything that the diff adds, removes, or modifies,
  including files that it adds, removes, or renames, and content that it adds,
  removes, or edits.
- **Scope of a review**: the change under review, the references to anything it
  renames or removes, and, for a meaning-changing change, as defined in
  [Classification](#classification), the content that depends on it.

### Rules

- A review MUST NOT review, validate, or report content outside its scope,
  including a defect that predates the change.
- A review MUST treat content that the change relies on but does not modify,
  such as a configuration file, a tool, or the review environment, as outside
  its scope, even when the change makes it more relevant.
- A review MAY make a [content search](./AGENTS.md#local-links) for the name of
  anything the change renames or removes, to find the references that its scope
  requires. This is the only content search that a review MAY make, and it MUST
  NOT be used to find other content.
- A review MUST find each reference to anything the change renames or removes,
  by that search, and MUST evaluate each reference found.
- A change that removes or renames a file, a section, or a heading MUST update
  every reference to it, in any file. A review MUST report each reference that
  the change does not update.
- A review of a change to a standard document MUST read that document in full,
  not only the parts that changed.
- A review MUST treat content that the pull request description leaves for a
  later pull request as outside its scope.
- A review MUST use the pull request description only after it has reviewed the
  change, to check that it did not miss anything, and MUST NOT use it to decide
  which documents to read or how to navigate.
- The rules of this section MUST take precedence over every other review rule
  except the [interpretation clause](#interpretation), including the duty to
  report a correctness defect that
  [Rules and recommendations](./AGENTS.md#rules-and-recommendations) defines.

## Classification

This section defines how a review classifies each change in its scope.

### Terms

- **Meaning-preserving change**: a change that keeps the behavior and meaning of
  what it touches, such as letter case or punctuation in prose, a typo fix, a
  rename, or a substitution by an equivalent construct.
- **Meaning-changing change**: any other change. A change to a value or name
  that a standard document requires other content to use is meaning-changing,
  even when it is a rename.

### Rules

- A review MUST classify each change in its scope as one or the other and apply
  the rules below accordingly. It MAY omit the classification from its report.
- Meaning-preserving: the review MUST verify only that meaning is preserved and
  that references to anything renamed still resolve, and apply the
  normative-language check defined in [Normative language](#normative-language).
  It MUST NOT re-evaluate the touched content against anything else.
- Meaning-changing: the review MUST evaluate the new meaning against the content
  that depends on it. For a standard document, that content is each file that
  its `scope` field matches, as defined in its front matter, and it includes
  what the change makes noncompliant.

## Normative language

This section defines the normative-language check of a review.

### Rules

- A review MUST verify that the content in its scope follows the
  normative-language rules for that content, as defined in
  [Normative language](./AGENTS.md#normative-language), for every change in its
  scope, and MUST report each deviation as a finding citing them.

## Findings

This section defines the sources of a finding and how strongly each one binds.

### Terms

- **Correctness defect**: as defined in
  [Rules and recommendations](./AGENTS.md#rules-and-recommendations), including
  a rule that contradicts itself or another rule.
- **Failure of a declared tool**: a failure that reaches the review from a run
  of the tool, such as in CI, one the user requests, or one the review makes as
  permitted in [Tools in reviews](#tools-in-reviews).

### Rules

- A review MUST state the source of each finding: a correctness defect, a
  standard document, a tool declared in this repository, or a skill or agent
  defined outside this repository.
- A finding MUST be blocking when its source is a correctness defect, a standard
  document, or a failure of a declared tool.
- A finding from a skill or agent defined outside this repository MUST be a
  non-blocking recommendation. When that skill or agent declares it blocking, or
  declares that it MUST be fixed, the review MUST mark it as a strong
  recommendation and name the skill or agent.
- A review MAY report a deviation from a preference that a skill or agent
  defined outside this repository states, as defined in
  [Skills](./AGENTS.md#skills).
- An explicit user instruction MAY change whether a finding is blocking, for the
  task at hand only.
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

## Finding forms

This section defines the forms that a finding takes, one for each source that
[Findings](#findings) accepts.

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

## Checklist

This section defines the checklist that a review applies before it reports.

### Rules

- Before it reports, a review MUST drop each finding that fails any item of this
  checklist:
  - It is within the scope of the review.
  - It states its source, and the source applies: the cited rule exists and
    covers the content, or the described defect occurs with the content as
    written.
  - It does not report what a declared tool checks, unless a failure of that
    tool reached the review as defined in [Findings](#findings).
  - It does not repeat another finding.
- The review MUST also confirm that it applied the normative-language check,
  defined in [Normative language](#normative-language), to the content in its
  scope, for every change in its scope.

## Report

This section defines the template of a finding, the template of a published
report, and the content of both.

### Terms

- **Published report**: a review report that is posted outside the conversation
  with the user (e.g. a comment on a pull request, or a file).

### Finding template

````md
---
id: <finding-id>
ruleId: <qualified-rule-identifier>
level: error | warning | note
fingerprint: sha256:<hash>

location:
  file: <path relative to the repository root, starting with ./>
  line: <line number>
  columns:
    start: <column>
    end: <column>

source:
  type: repository-rule | declared-tool | correctness-defect | skill | agent
  document: <document-name>
  section: <section-name>
---

#### <title that identifies the issue> [<blocking | strong recommendation, must be fixed | recommendation>]

**Problem:** <what the issue is and why it matters>

**Broken rule:** <the rule text, quoted, with its file and section; for a
correctness defect, "no explicit rule"; for a declared tool or an external skill
or agent, what it reports>

**Correction:** <the correction, in plain words or in pseudocode>

```diff
- <current line>
+ <suggested line>
```
````

### Report template

```md
---
repository: <repository-url>

branch:
  head: <branch-name>
  base: <target-branch>

commit:
  head: <sha-full>
  base: <sha-full>

reviewer:
  human:
    email: <email>

  agent:
    skills: [<skill-name>, ...]
    model: <model>
    harness: <harness>
    effort: <effort>
    timestamp: <ISO-8601-datetime>
    session-id: <session-id>
---

# Review Report

## Findings

<each finding, in the finding template, with front matter and content>
```

### Rules

- A review MUST write each finding in the [finding template](#finding-template),
  whether it reports in the conversation with the user or in a published report.
- A published report MUST use the [report template](#report-template), including
  any report that is sent under a request to send it. In the conversation with
  the user, the review MAY report the findings alone.
- A review report MUST NOT contain personal data about the user, under any
  authorization.
- A path in a review report MUST be relative to the repository working
  directory. Before writing a path, the agent MUST resolve it against that
  directory, and MUST NOT write an absolute path from the user's machine.
- A finding that shows a defect in code MUST describe its proof in pseudocode or
  in plain words. A report MUST NOT contain ready-to-run code, exploit code, or
  a ready-to-use payload.
- A review report MUST state, for each finding, its source and the rule or
  defect it cites.
- The level of each finding MUST be one of the values of the
  [finding template](#finding-template), as [Findings](#findings) defines them.
- `review.severity` MUST be the severity that the source of the finding gives,
  as the source writes it (e.g. a skill, a tool, or the user). The reviewer MUST
  NOT assign a severity of its own.
- A field that the source does not inform MUST be written as `not informed`.
- `review` MUST be filled for every reviewer, not only for GitHub Copilot.
- The fields MUST keep the order of the [finding template](#finding-template)
  and MUST NOT change their names.
- The correction MUST be described in plain words or in pseudocode. When the
  content is not code, the finding MUST also show the current and the suggested
  text in a diff block, as the [finding template](#finding-template) shows.
- A review report MUST NOT report or list the dropped candidates. The review
  MUST keep each of them, with the [checklist](#checklist) item it failed, in
  its session context, not in its report.
- A review that reports no findings MUST still state that it applied the
  [checklist](#checklist).

## Tools in reviews

This section defines which tools a review uses and when it runs a declared tool.

### Rules

- In a review, an agent MUST use native tools instead of shell commands, custom
  scripts, or other custom means that do the same work, as defined in
  [Choosing a tool](./AGENTS.md#choosing-a-tool), unless a declared tool is
  declared for that work, in which case the declared tool MUST be used.
- In a review, an agent MUST use a provided tool instead of reaching the same
  service through the shell or a custom means.
- A review MAY run a declared tool when it has a plausible reason, such as
  verifying a finding.
- A review MUST NOT run a declared tool only to check what that tool is expected
  to check on its own, such as through CI or a hook.
- Writing that an agent does in the same session as a review, such as applying a
  fix, MUST NOT be treated as part of the review, and
  [Writing content](./AGENTS.md#writing-content) MUST apply to it.

## Links

- [README.md](./README.md): What Kraf is and its vision, and the index of the
  repository's documents, modules and other files.
- [AGENTS.md](./AGENTS.md): Agent instructions and the repository standards they
  MUST follow.
- [TOOLS.md](./docs/TOOLS.md): Tools declared in this repository, with their
  purpose, configuration, capabilities, and version.
