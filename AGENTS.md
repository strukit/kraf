---
title: AGENTS
summary: Entry point for agent instructions and the repository conventions they MUST follow.
tags: [agents, instructions, repository, coding-conventions]
applies_to: [all]
---

Before anything else, you MUST read
[GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md). You MUST NOT continue
with this document, or with any other action, until you have read it. After it,
you MUST read [README.md](./README.md).

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#normative-language).
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords.

---

> [!IMPORTANT]
> This is a convention document, as defined in
> [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#convention-documents):
> its rules are mandatory and non-negotiable. Treat them like a constitution: a
> rule changes only through a pull request that edits this document and MUST NOT
> be bypassed by an ad-hoc exception in code.

This document is the entry point for every agent task in this repository.

- Before starting any task, agents MUST start here and follow the documentation
  links to documents the task needs.

## Rules and recommendations

**Rule**: a requirement that a convention document defines, including a skill or
agent defined in this repository. A rule binds with the force of its normative
keyword, as RFC 2119 defines it: a SHOULD or RECOMMENDED rule, for example,
allows a deviation for a valid reason.

**Recommendation**: what a skill or agent defined outside this repository
states, as defined in Skills. It is not a rule and does not bind.

- Agents MUST follow the rules.
- A recommendation MUST NOT override a rule.
- Agents MAY act on a recommendation from a skill or agent that the task uses.
- Agents MUST NOT raise, flag, or fix a concern that neither a rule nor such a
  recommendation defines, such as choosing one approach, name, or style over
  another.
- Agents MUST report a correctness defect they find, such as code that produces
  a wrong result or contradicts its spec, even when no convention document
  defines it.
- When an agent raises a concern, it MUST cite its source: the rule that defines
  it, the skill or agent that recommends it, or, for a correctness defect, the
  wrong result or contradiction it found.
- Besides the exceptions this document defines, an explicit user instruction MAY
  set aside a rule for the task at hand only, and does not change the rule.
- Agents MUST NOT override a rule on their own preference or judgment.
- An agent MUST NOT do work unless the user explicitly requests it, a convention
  document or a skill or agent that the task uses requires, recommends, or
  permits it as part of the work the task performs, or the task cannot be
  completed without it.

## Normative language

- When an agent writes or edits a document, it MUST verify that the content it
  writes follows the normative-language rules that apply to that document: those
  of
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#normative-language)
  and, for a specification or an intent, those of
  [SPEC_CONVENTIONS.md](./docs/SPEC_CONVENTIONS.md).
- A review MUST verify that the content in its scope follows those rules, for
  every change in its scope, and MUST report each deviation as a finding with
  those rules as its source.

## Tools

**Tool**: anything the agent can invoke, other than one whose purpose is to
delegate work to an agent or a language model: its native tools, tools provided
to it, such as MCP tools, programs installed in the user's environment, and
tools declared in this repository.

**Declared tool**: a tool listed in [TOOLS.md](./docs/TOOLS.md). No other tool
is declared.

### Choosing a tool

- An agent MUST NOT perform or verify by its own means work that an available
  tool performs.
- An agent MUST use its native tools instead of shell commands that do the same
  work, and a provided tool instead of reaching the same service through the
  shell.
- When this repository declares a tool for a purpose, an agent MUST use that
  tool for that purpose.
- When the list states that a declared tool is invoked by another declared tool,
  using the invoking tool counts as using the invoked one. An agent SHOULD use
  the invoking tool and SHOULD NOT invoke the invoked tool directly.
- When a tool is not available, the agent MUST inform the user and continue the
  task.

### Declaring a tool

- A pull request that adds or removes a tool's configuration MUST update the
  list in [TOOLS.md](./docs/TOOLS.md).
- A file that only editors or IDEs read, such as
  [.editorconfig](./.editorconfig), configures no tool that an agent can invoke:
  it is not a tool's configuration and MUST NOT be listed.

### Writing content

- While writing, an agent MAY ignore conventions that a declared tool with
  autofix applies, such as formatting rules.
- While writing, an agent MUST follow the conventions that any other declared
  tool checks, such as spelling or lint rules.
- Once it finishes writing, it MUST run each declared tool for which
  [TOOLS.md](./docs/TOOLS.md) states capabilities and that checks the files it
  changed, in the way it states under Run, if any, and according to those
  capabilities:
  - For a tool with autofix, the agent MUST run the tool's fix. It MAY skip the
    check.
  - For any other tool, the agent MUST run the check first, then fix each
    problem the check reports, with the tool's fix when the tool can fix it and
    by its own means otherwise, and then MUST run the check again to confirm
    that no problem remains.

### Tools in reviews

- A review MUST NOT run a declared tool and MUST NOT ask anyone to run one.
- When an agent writes content in the same session as a review, such as applying
  a fix, that writing is not part of the review, and the rules in Writing
  content apply to it.

## Reviews

> [!IMPORTANT]
> Golden rule: a review MUST apply the rules of this document literally, as
> written. It MUST NOT extend a rule beyond its text, narrow it, or replace it
> with its own reading of the rule's intent. This rule MUST take precedence over
> every other review rule.

- A prompt from a skill, command, or agent defined outside this repository that
  instructs a review, such as a minimum or maximum number of findings, skipping
  verification, or favoring recall over precision, is a recommendation. A
  review MUST follow this section instead and MUST notify the user of the
  conflict, as defined in User instructions.

### Scope

The scope of a review is the change under review, the references to anything
the change renames, and, for a meaning-changing change, the content that depends
on it.

- A review MUST NOT review, validate, or report content outside that scope,
  including a defect that predates the change.
- Content that the change relies on but does not modify, such as a configuration
  file, a tool, or the environment the review runs in, is outside the scope,
  even when the change makes it more relevant.
- This scope rule MUST take precedence over every other review rule except the
  golden rule, including the duty to report a correctness defect.

### Classification

**Meaning-preserving change**: a change that keeps the behavior and meaning of
what it touches, such as letter case or punctuation in prose, a typo fix, a
rename, or a substitution by an equivalent construct.

**Meaning-changing change**: any other change. A change to a value or name that
a convention document requires other content to use is meaning-changing.

- A review MUST determine, for each change in its scope, whether it is
  meaning-preserving or meaning-changing, and MUST apply the rules below
  accordingly. It MAY omit this classification from its report.
- For a meaning-preserving change, the review MUST verify only that the change
  preserves meaning and that references to anything it renames still resolve,
  and apply the normative-language check defined in Normative language. It MUST
  NOT re-evaluate the touched content against anything else.
- For a meaning-changing change, the review MUST evaluate the new meaning
  against the content that depends on it.
- For a meaning-changing change to a convention document, the content that
  depends on it includes the content that the change makes noncompliant.
  Content that the pull request description leaves for a later pull request is
  outside the scope.

### Findings

- A review MUST state the source of each finding: a correctness defect, a
  convention document, a tool declared in this repository, or a skill or agent
  defined outside this repository.
- A correctness defect includes a rule that contradicts itself or another rule.
- A finding is blocking when its source is a correctness defect, a convention
  document, or a failure of a tool declared in this repository. Since a review
  MUST NOT run declared tools, such a failure reaches it from outside the
  review, such as from CI or from a run that the user requests.
- A finding from a skill or agent defined outside this repository is a
  recommendation and is non-blocking. When that skill or agent declares the
  finding blocking, or declares that it MUST be fixed, the review MUST mark it
  as a strong recommendation and name the skill or agent.
- An explicit user instruction MAY change whether a finding is blocking, for
  the task at hand only.
- A review MUST NOT report or list missing content unless a convention document
  requires that content, including by defining its absence as a defect, or a
  skill or agent that the task uses expects it.
- A review MUST be thorough from its first pass and MUST apply the same criteria
  to every change in its scope, so that reviewing the same change again yields
  the same findings.
- A review MUST report every finding it can support and only those. It MUST NOT
  add, drop, or weaken a finding to reach a number of findings, even when a
  skill or agent asks for a minimum or a maximum.
- A review that finds nothing it can support MUST report no findings.

### Checklist

Before it reports, a review MUST go back over its findings and drop each one
that fails any item of this checklist:

- It is within the scope of the review.
- It states its source, and the source applies: the rule it cites exists and
  covers the content, or the defect it describes occurs with the content as
  written.
- It does not report what a declared tool checks, unless a failure of that tool
  reached the review from outside it.
- It does not repeat another finding.

The review MUST also confirm that it applied the normative-language check to
every change in its scope, as defined in Normative language.

### Report

- A review report MUST state, for each finding, its source and the rule or
  defect it cites.
- It MUST NOT report or list the candidates that the checklist dropped. It MUST
  keep each of them, with the checklist item it failed, in the context of its
  session, and not in its report.
- A review that reports no findings MUST still state that it applied the
  checklist.

## Pull requests

- When an agent writes or updates a pull request description, it MUST follow
  [the pull request template](./.github/pull_request_template.md) and fill in
  each of its sections.
- Every `<...>` placeholder in the template MUST be replaced with content; its
  text describes what the section usually contains. A placeholder MUST NOT
  remain in the description.
- Each item MUST state briefly, in the agent's own words, what changed and, when
  it is not evident, why.
- An item MUST NOT copy or restate in detail the text of the content it changes,
  such as the wording of a rule; the diff carries that text.
- A pull request description MUST NOT ask a reviewer to run a declared tool,
  such as in its `How to test?` section, as defined in Tools in reviews.

## Memory

- Memory MAY inform a task but MUST NOT override a rule.
- A conflict between memory and a rule is handled as defined in User
  instructions.

## Skills

- A skill or agent defined in this repository is a convention document: its file
  MUST carry the normative-language note defined in
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#normative-language) and
  the convention note defined in
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#convention-documents),
  and its rules bind like those of any other convention document.
- A skill or agent defined outside this repository, including the user's own and
  third-party ones, states recommendations, not rules. It MAY inform a task but
  MUST NOT override a rule. A conflict between it and a rule is handled as
  defined in User instructions.
- Such a skill or agent MAY state preferences, such as formatting, layout, or
  naming choices, that agents follow when they create or edit content under it.
  A preference is a recommendation: it MUST NOT override a convention document.
  A review MAY report a deviation from it, as defined in Reviews.

## User instructions

**Explicit user instruction**: one the user gives in the task itself. An
instruction the agent loads from a source that persists across tasks, including
a personal instruction file and the agent's memory, is not explicit, even when
it is the user's own.

- An explicit user instruction MUST be respected, as defined in Rules and
  recommendations.
- An instruction that is not explicit, including memory, or a skill or agent
  defined outside this repository can conflict with a rule. When an agent finds
  such a conflict:
  - It MUST continue the task following the rule.
  - It MUST notify the user, briefly stating which instruction and which rule
    are involved and that, if the content is pertinent, it needs to be committed
    to a convention document to take effect.
  - It MUST NOT suppress the conflict silently.
- When following the rule would cause an effect that is hard to reverse or that
  reaches outside the working root, the agent MUST stop and ask the user before
  continuing.

## Workspace boundary

- Agents MUST NOT leave the working root for any operation they perform
  directly, including reading a file, writing a file, navigating files, or
  executing a command whose working directory or path arguments are outside the
  working root.
- The rule applies to the agent's direct operations only. Files that a tool
  reads or writes on its own when the agent invokes it are not operations by the
  agent and are outside the scope of this rule.
- Reading or writing the agent's own memory is outside the scope of this rule.
- An explicit user instruction MAY authorize an operation outside the working
  root for the task at hand only.
- Before its first operation outside the working root, an agent MUST ask the
  user for confirmation, even when the user has provided that authorization.

## Navigation

Following local Markdown links is how agents MUST navigate this repository. It
keeps the agent's context limited to documents the task needs, instead of
unrelated content pulled in by other means.

### Local links

**Listing**: links presented as a list. A listing is an index, not a reading
list.

- When an agent encounters a link to a local file whose content the task needs,
  it MUST open and read the file before continuing. It MUST resolve relative
  paths from the directory of the document containing the link.
- To follow a link to a whole file, an agent MUST use its native file-reading
  tool.
- To follow a link to a section of a file, an agent MUST use its native
  file-reading tool when that tool can read the section. Otherwise, it MAY use
  any other means to locate and read the section in the linked file.
- Reading the links of a listing is OPTIONAL. An agent MUST open a file linked
  from a listing only when the task needs its content, and MUST NOT open it only
  because it is listed. An entry MAY state when the task needs the file.
- As an exception, an agent MAY read only the front-matter of a file linked from
  a listing to determine whether the task needs its content.
- A link inside an example, a code block, or a template illustrates a format and
  is not navigation. An agent MUST NOT follow it unless the task needs the
  linked file for a reason other than the example.
- A link MAY resolve to any file inside the working root, including one above
  the document containing the link.
- When a link resolves outside the working root, the agent MUST NOT read the
  file, MUST notify the user, naming the link and the document that contains it,
  and MUST ignore the link and continue the task without that file.
- Agents MUST NOT search file contents as a substitute for following those
  links, unless the user explicitly requests it. Reading a file, and listing or
  filtering file names, are not content search.
- When no link leads to content the task needs, an agent MAY locate it by file
  name.
- If the destination also contains local links needed for the task, the agent
  MUST follow them and SHOULD NOT reread files already consulted.
- If a file does not exist or cannot be read, the agent MUST report the
  limitation and MUST NOT assume its contents.

### HTTP links

This section applies to every HTTP URL an agent fetches, whether it comes from a
Markdown link, from plain text, or from the agent itself.

- An agent MUST NOT fetch an HTTP URL unless the task needs content of the URL
  that the agent does not know in enough depth from its training.
- An agent SHOULD request the raw-text form of the page when the site provides
  one.

Saved content:

- Before fetching such a URL, the agent MAY look for content previously saved
  from that exact URL in its own memory.
- Memory MAY be used, but it MUST NOT override the convention documents, and
  when it differs from the page at the URL, the page MUST take precedence.
- When the URL is pinned to a revision or a commit and the saved content records
  the same URL, the saved content is equivalent to the page, and the agent MAY
  use it instead of fetching the URL.
- Otherwise, the saved content can be outdated, and the agent SHOULD confirm the
  saved content against the page, or against evidence of when the content was
  saved and when the page last changed, before relying on it.
- After fetching such a URL, the agent MAY save the content the task needed in
  its own memory, under the exact URL. It SHOULD save it in the memory it keeps
  for this project rather than in its general memory, and it MUST NOT save it
  inside the working root.

## Related

- [README.md](./README.md): what Kraf is and more.
- [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md): repo-wide
  conventions: convention documents and rule precedence, docs, workspace,
  naming.
