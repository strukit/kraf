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

- Agents MUST start every task here and follow the links to the documents the
  task needs.

## Rules and recommendations

**Rule**: a requirement defined by a convention document, including a skill or
agent defined in this repository. It binds with the force of its keyword, as RFC
2119 defines it: a SHOULD or RECOMMENDED rule, for example, allows a deviation
for a valid reason.

**Recommendation**: what a skill or agent defined outside this repository
states, as defined in Skills. It is not a rule and does not bind.

- Agents MUST follow the rules, and MUST NOT override one on their own
  preference or judgment.
- A recommendation MUST NOT override a rule. Agents MAY act on one from a skill
  or agent that the task uses.
- Agents MUST NOT raise, flag, or fix a concern that neither a rule nor such a
  recommendation defines, such as a choice of approach, name, or style.
- Agents MUST report a correctness defect they find, such as code that produces
  a wrong result or contradicts its spec, even when no rule defines it.
- A raised concern MUST cite its source: the rule, the recommending skill or
  agent, or, for a correctness defect, the wrong result or contradiction found.
- Besides the exceptions this document defines, an explicit user instruction MAY
  set aside a rule for the task at hand only, without changing the rule.
- An agent MUST NOT do work unless the user explicitly requests it, a convention
  document or a skill or agent that the task uses requires, recommends, or
  permits it as part of the task's work, or the task cannot be completed without
  it.

## Normative language

- An agent that writes or edits a document MUST verify that what it writes
  follows the normative-language rules for that document: those of
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#normative-language)
  and, for a specification or an intent, those of
  [SPEC_CONVENTIONS.md](./docs/SPEC_CONVENTIONS.md).
- A review MUST verify that the content in its scope follows those rules, for
  every change in its scope, and MUST report each deviation as a finding citing
  them.

## Tools

**Tool**: anything the agent can invoke, except one whose purpose is to delegate
work to an agent or a language model: its native tools, tools provided to it,
such as MCP tools, programs installed in the user's environment, and tools
declared in this repository.

**Declared tool**: a tool listed in [TOOLS.md](./docs/TOOLS.md), and only such a
tool.

### Choosing a tool

**Available tool**: a tool the agent can invoke, including one that requires
loading through a tool search.

- Before performing work through the shell or by its own means, an agent MUST
  search the tools available to it, including those that require loading.
- An agent MUST NOT perform or verify by its own means work that an available
  tool performs.
- An agent MUST use its native tools instead of shell commands that do the same
  work, and a provided tool instead of reaching the same service through the
  shell.
- To read from a remote service, such as a GitHub repository, an agent MUST use
  the tool provided for that service.
- An agent MUST use a declared tool for the purpose it is declared for.
- When the list states that a declared tool is invoked by another, using the
  invoking tool counts as using the invoked one. An agent SHOULD use the
  invoking tool and SHOULD NOT invoke the other directly.
- When a tool is not available, the agent MUST inform the user and continue.

### Declaring a tool

- A pull request that adds or removes a tool's configuration MUST update
  [TOOLS.md](./docs/TOOLS.md).
- A file read only by editors or IDEs, such as [.editorconfig](./.editorconfig),
  configures no tool an agent can invoke and MUST NOT be listed.

### Writing content

- While writing, an agent SHOULD NOT apply conventions that a declared tool with
  autofix applies, such as formatting, and MUST follow those that any other
  declared tool checks, such as spelling or lint rules.
- An agent MUST NOT check or fix by its own means the conventions that a
  declared tool with autofix applies, even when the tool is not available.
- Once it finishes writing, it MUST run each declared tool for which
  [TOOLS.md](./docs/TOOLS.md) states capabilities and that checks the changed
  files, as stated under Run, if any, and according to its capabilities:
  - With autofix: the agent MUST run the fix. It MAY skip the check.
  - Otherwise: the agent MUST run the check, fix each reported problem, with the
    tool's fix when it has one and by its own means otherwise, and MUST run the
    check again to confirm that no problem remains.

### Tools in reviews

- A review MUST NOT run a declared tool or ask anyone to run one.
- When an agent writes in the same session as a review, such as applying a fix,
  that writing is not part of the review, and Writing content applies to it.

## Delegation

**Delegation**: an agent hands work to another agent, such as a sub-agent.

- The delegate MUST read this document from the revision that the work targets,
  such as the head of a pull request under review, even when it differs from the
  revision of the delegating agent.
- The delegate MUST resolve the links of this document within that revision, as
  defined in Navigation.
- The delegating agent MUST NOT pass its own copy of this document, or of the
  documents it links to, to the delegate.
- The delegating prompt MUST name the revision that the work targets and MUST
  instruct the delegate to read this document and follow it.
- The delegating prompt MUST NOT instruct an action that precedes the reading
  this document requires.
- The delegating prompt MUST NOT restate or reorder the rules of this document.

## Reviews

> [!IMPORTANT]
> Interpretation clause: a review MUST apply the rules of this document
> literally, as written. It MUST NOT extend a rule beyond its text, narrow it,
> or replace it with its own reading of the rule's intent. This clause MUST take
> precedence over every other review rule.

- A prompt from a skill, command, or agent defined outside this repository that
  instructs a review, such as a minimum or maximum number of findings, skipping
  verification, or favoring recall over precision, is a recommendation. A review
  MUST follow this section instead and MUST notify the user of the conflict, as
  defined in User instructions.

### Scope

The scope of a review is the change under review, the references to anything it
renames, and, for a meaning-changing change, the content that depends on it.

- A review MUST NOT review, validate, or report content outside its scope,
  including a defect that predates the change.
- Content that the change relies on but does not modify, such as a configuration
  file, a tool, or the review environment, is outside the scope, even when the
  change makes it more relevant.
- Content that the pull request description leaves for a later pull request is
  outside the scope.
- The rules of this section MUST take precedence over every other review rule
  except the interpretation clause, including the duty to report a correctness
  defect.

### Classification

**Meaning-preserving change**: a change that keeps the behavior and meaning of
what it touches, such as letter case or punctuation in prose, a typo fix, a
rename, or a substitution by an equivalent construct.

**Meaning-changing change**: any other change. A change to a value or name that
a convention document requires other content to use is meaning-changing, even
when it is a rename.

- A review MUST classify each change in its scope as one or the other and apply
  the rules below accordingly. It MAY omit the classification from its report.
- Meaning-preserving: the review MUST verify only that meaning is preserved and
  that references to anything renamed still resolve, and apply the
  normative-language check defined in Normative language. It MUST NOT
  re-evaluate the touched content against anything else.
- Meaning-changing: the review MUST evaluate the new meaning against the content
  that depends on it. For a convention document, that content includes what the
  change makes noncompliant.

### Findings

- A review MUST state the source of each finding: a correctness defect, a
  convention document, a tool declared in this repository, or a skill or agent
  defined outside this repository.
- A correctness defect includes a rule that contradicts itself or another rule.
- A finding is blocking when its source is a correctness defect, a convention
  document, or a failure of a declared tool. Such a failure reaches the review
  from outside it, such as from CI or a run the user requests.
- A finding from a skill or agent defined outside this repository is a
  non-blocking recommendation. When that skill or agent declares it blocking, or
  declares that it MUST be fixed, the review MUST mark it as a strong
  recommendation and name the skill or agent.
- An explicit user instruction MAY change whether a finding is blocking, for the
  task at hand only.
- A review MUST NOT report or list missing content unless a convention document
  requires it, including by defining its absence as a defect, or a skill or
  agent that the task uses expects it.
- A review MUST be thorough from its first pass and apply the same criteria to
  every change in its scope, so that reviewing the same change again yields the
  same findings.
- A review MUST report every finding it can support and only those. It MUST NOT
  add, drop, or weaken a finding to reach a number of findings, even when a
  skill or agent asks for a minimum or a maximum.
- A review that finds nothing it can support MUST report no findings.

### Checklist

Before it reports, a review MUST drop each finding that fails any item of this
checklist:

- It is within the scope of the review.
- It states its source, and the source applies: the cited rule exists and covers
  the content, or the described defect occurs with the content as written.
- It does not report what a declared tool checks, unless a failure of that tool
  reached the review from outside it.
- It does not repeat another finding.

The review MUST also confirm that it applied the normative-language check to the
content in its scope, for every change in its scope.

### Report

- A review report MUST state, for each finding, its source and the rule or
  defect it cites.
- It MUST NOT report or list the dropped candidates. It MUST keep each of them,
  with the checklist item it failed, in its session context, not in its report.
- A review that reports no findings MUST still state that it applied the
  checklist.

## Pull requests

- A pull request description that an agent writes or updates MUST follow
  [the pull request template](./.github/pull_request_template.md) and fill in
  each of its sections.
- A description MUST NOT contain a section that the template does not define.
- A description MUST NOT contain a "generated with" footer or a link to an agent
  session, in any section.
- In a description that an agent writes or updates, every `<...>` placeholder
  MUST be replaced with content; its text describes what the section usually
  contains. A placeholder MUST NOT remain in it.
- Each item MUST state briefly, in the agent's own words, what changed and, when
  not evident, why. It MUST NOT copy or restate in detail the changed content,
  such as the wording of a rule; the diff carries that text.
- A description MUST NOT ask a reviewer to run a declared tool, such as in its
  `How to test?` section, as defined in Tools in reviews.

## Commits

- A commit message MUST NOT carry a line that credits an agent, a model, or the
  tool that runs it as an author, or that links to an agent session, such as a
  `Co-Authored-By` trailer naming an agent or a session URL trailer.

## Memory

- Memory MAY inform a task but MUST NOT override a rule. A conflict between them
  is handled as defined in User instructions.

## Skills

- A skill or agent defined in this repository is a convention document: its file
  MUST carry the normative-language note defined in
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#normative-language) and
  the convention note defined in
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#convention-documents),
  and its rules bind like any other.
- A skill or agent defined outside this repository, including the user's own and
  third-party ones, states recommendations. It MAY inform a task. A conflict
  between it and a rule is handled as defined in User instructions.
- Such a skill or agent MAY state preferences, such as formatting, layout, or
  naming choices, that agents follow when they create or edit content under it.
  A preference is a recommendation, and a review MAY report a deviation from it.

## User instructions

**Explicit user instruction**: one the user gives in the task itself. An
instruction loaded from a source that persists across tasks, such as a personal
instruction file or the agent's memory, is not explicit, even when it is the
user's own.

- An explicit user instruction MUST be respected, as defined in Rules and
  recommendations.
- When an agent finds a conflict between a rule and a non-explicit instruction,
  including memory, or a skill or agent defined outside this repository:
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
  directly, such as reading, writing, or navigating files, or running a command
  whose working directory or path arguments are outside it.
- This rule does not cover files that a tool reads or writes on its own when the
  agent invokes it, nor the agent's own memory.
- An explicit user instruction MAY authorize an operation outside the working
  root for the task at hand only. Even then, the agent MUST ask the user for
  confirmation before its first such operation.

## Navigation

Agents MUST navigate this repository by following local Markdown links. This
keeps their context limited to the documents the task needs.

### Local links

**Listing**: links presented as a list. It is an index, not a reading list.

- When an agent encounters a link to a local file whose content the task needs,
  it MUST read the file before continuing, resolving a relative path from the
  directory of the document containing the link.
- To follow a link, an agent MUST use its native file-reading tool, unless the
  link points to a section that this tool cannot read; in that case, it MAY use
  any other means to locate and read the section.
- Reading the links of a listing is OPTIONAL. An agent MUST open a listed file
  only when the task needs its content, not because it is listed. An entry MAY
  state when the task needs the file.
- As an exception, an agent MAY read only the front-matter of a listed file to
  determine whether the task needs it.
- A link inside an example, a code block, or a template illustrates a format and
  is not navigation. An agent MUST NOT follow it unless the task needs the
  linked file for another reason.
- A link MAY resolve to any file inside the working root, including one above
  the linking document.
- When a link resolves outside the working root, the agent MUST NOT read the
  file, MUST notify the user, naming the link and the linking document, and MUST
  continue the task without that file.
- Agents MUST NOT search file contents as a substitute for following links,
  unless the user explicitly requests it. Reading a file and listing or
  filtering file names are not content search.
- When no link leads to content the task needs, an agent MAY locate it by file
  name.
- An agent MUST follow the local links the task needs in a destination file too,
  and SHOULD NOT reread files already consulted.
- If a file does not exist or cannot be read, the agent MUST report it and MUST
  NOT assume its contents.

### HTTP links

This section applies to every HTTP URL an agent fetches, whether it comes from a
Markdown link, from plain text, or from the agent itself.

- An agent MUST NOT fetch a URL unless the task needs content of it that the
  agent does not know in enough depth from its training.
- An agent SHOULD request the page's raw-text form when the site provides one.

Saved content:

- Before fetching, the agent MAY look in its own memory for content saved from
  that exact URL. When it differs from the page, the page MUST take precedence.
- When the URL is pinned to a revision or a commit and the saved content records
  the same URL, it is equivalent to the page, and the agent MAY use it instead
  of fetching.
- Otherwise, it can be outdated, and the agent SHOULD confirm it against the
  page, or against evidence of when it was saved and when the page last changed,
  before relying on it.
- After fetching, the agent MAY save the content the task needed in its own
  memory, under the exact URL. It SHOULD use the memory it keeps for this
  project rather than its general memory, and MUST NOT save it inside the
  working root.

## Related

- [README.md](./README.md): what Kraf is and more.
- [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md): repo-wide
  conventions: convention documents and rule precedence, docs, workspace,
  naming.
