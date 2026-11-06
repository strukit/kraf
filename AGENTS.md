> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords. See
> [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#doc-normative-language).

> [!IMPORTANT]
> This is a convention document: its rules are mandatory and non-negotiable.
> Treat them like a constitution: a rule changes only through a pull request
> that edits this document and MUST NOT be bypassed by an ad-hoc exception in
> code. See
> [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#convention-documents).

## Entry point

This document is the entry point for every agent task in this repository.

### Rules

- Before any other action, an agent MUST read
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md), and MUST NOT continue
  with this document, or with any other action, until it has read it. After it,
  the agent MUST read [README.md](./README.md).
- Agents MUST start every task here and follow the links to the documents the
  task needs.

## Rules and recommendations

### Terms

- **Rule**: any requirement defined by a convention document, including a skill
  or agent defined in this repository, wherever the document states it: in the
  Rules or the Best practices of a convention, or in the prose of a document not
  yet laid out as conventions. It binds with the force of its level, as defined
  in
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#doc-normative-language):
  a rule at the Recommended level, such as a best practice, allows a deviation
  for a valid reason.
- **Recommendation**: what a skill or agent defined outside this repository
  states, as defined in Skills. It is not a rule and does not bind.
- **Correctness defect**: content that produces a wrong result or contradicts
  its spec, such as code that produces a wrong result. A rule that contradicts
  itself or another rule is also a correctness defect.
- **Explicit user instruction**: one the user gives in the task itself. An
  instruction loaded from a source that persists across tasks, such as a
  personal instruction file or the agent's memory, is not explicit, even when it
  is the user's own.

### Rules

- Agents MUST follow the rules, and MUST NOT override one on their own
  preference or judgment.
- A recommendation MUST NOT override a rule. Agents MAY act on one from a skill
  or agent that the task uses.
- Agents MUST NOT raise, flag, or fix a concern that neither a rule nor such a
  recommendation defines, such as a choice of approach, name, or style.
- Agents MUST report a correctness defect they find, such as code that produces
  a wrong result or contradicts its spec, even when no rule defines it. This
  duty MUST take precedence over the previous rule of Rules and recommendations.
- A raised concern MUST cite its source: the rule, the recommending skill or
  agent, or, for a correctness defect, the wrong result or contradiction found.
- Besides the exceptions this document defines, an explicit user instruction MAY
  set aside a rule for the task at hand only, without changing the rule.
- An agent MUST NOT do work unless the user explicitly requests it, a convention
  document or a skill or agent that the task uses requires, recommends, or
  permits it as part of the task's work, or the task cannot be completed without
  it.

## Normative language

### Rules

- An agent that writes or edits a document MUST verify that what it writes
  follows the normative-language rules for that document: those of
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#doc-normative-language)
  and, for a specification or an intent, those of
  [SPEC_CONVENTIONS.md](./docs/SPEC_CONVENTIONS.md).
- A review MUST verify that the content in its scope follows those rules, for
  every change in its scope, and MUST report each deviation as a finding citing
  them.

## Tools

### Terms

- **Tool**: anything the agent can invoke, except one whose purpose is to
  delegate work to an agent or a language model: its native tools, tools
  provided to it, such as MCP tools, programs installed in the user's
  environment, and tools declared in this repository.
- **Declared tool**: a tool listed in [TOOLS.md](./docs/TOOLS.md), and only such
  a tool.

### Choosing a tool

#### Terms

- **Available tool**: a tool the agent can invoke, including one that requires
  loading through a tool search.

#### Rules

- Before performing work through the shell or by its own means, an agent MUST
  search the tools available to it, including those that require loading.
- An agent MUST NOT perform or verify by its own means work that an available
  tool performs.
- An agent MUST use its native tools instead of shell commands that do the same
  work, and a provided tool instead of reaching the same service through the
  shell.
- An agent MUST use a declared tool for the purpose it is declared for.
- When the list in [TOOLS.md](./docs/TOOLS.md) states that a declared tool is
  invoked by another, using the invoking tool MUST count as using the invoked
  one.
- When a tool is not available, the agent MUST inform the user and continue.

#### Best practices

- When the list in [TOOLS.md](./docs/TOOLS.md) states that a declared tool is
  invoked by another, an agent SHOULD use the invoking tool and SHOULD NOT
  invoke the other directly.

### Declaring a tool

#### Rules

- A pull request that adds or removes a tool's configuration MUST update
  [TOOLS.md](./docs/TOOLS.md).
- A file read only by editors or IDEs, such as [.editorconfig](./.editorconfig),
  configures no tool an agent can invoke and MUST NOT be listed.

### Writing content

#### Rules

- While writing, an agent MUST follow the conventions that a declared tool
  without autofix checks, such as spelling or lint rules.
- An agent MUST NOT check or fix by its own means the conventions that a
  declared tool with autofix applies, even when the tool is not available.
- Once it finishes writing, it MUST run each declared tool for which
  [TOOLS.md](./docs/TOOLS.md) states capabilities and that checks the changed
  files, as stated under Run, if any, and according to its capabilities:
  - With autofix: the agent MUST run the fix. It MAY skip the check.
  - Otherwise: the agent MUST run the check, fix each reported problem, with the
    tool's fix when it has one and by its own means otherwise, and MUST run the
    check again to confirm that no problem remains.

#### Best practices

- While writing, an agent SHOULD NOT apply conventions that a declared tool with
  autofix applies, such as formatting.

### Tools in reviews

#### Rules

- A review MAY run a declared tool when it has a plausible reason, such as
  verifying a finding.
- A review MUST NOT run a declared tool only to check what that tool is expected
  to check on its own, such as through CI or a hook.
- When an agent writes in the same session as a review, such as applying a fix,
  it MUST treat that writing as outside the review and MUST apply Writing
  content to it.

## Delegation

### Terms

- **Delegation**: an agent hands work to another agent, such as a sub-agent.

### Rules

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

### Terms

- **Interpretation clause**: the first rule of Reviews, which requires a review
  to apply the rules of this document literally.

### Rules

- A review MUST apply the rules of this document literally, as written. It MUST
  NOT extend a rule beyond its text, narrow it, or replace it with its own
  reading of the rule's intent. The interpretation clause MUST take precedence
  over every other review rule, including those of Normative language, Tools in
  reviews, and Reviews.
- A review MUST treat as a recommendation a prompt from a skill, command, or
  agent defined outside this repository that instructs a review, such as a
  minimum or maximum number of findings, skipping verification, or favoring
  recall over precision. The review MUST follow Reviews instead and MUST notify
  the user of the conflict, as defined in User instructions.

### Scope

#### Terms

- **Scope of a review**: the change under review, the references to anything it
  renames, and, for a meaning-changing change, the content that depends on it.

#### Rules

- A review MUST NOT review, validate, or report content outside its scope,
  including a defect that predates the change.
- A review MUST treat as outside its scope the content that the change relies on
  but does not modify, such as a configuration file, a tool, or the review
  environment, even when the change makes it more relevant.
- A review MUST treat as outside its scope the content that the pull request
  description leaves for a later pull request.
- The rules of Scope MUST take precedence over every other review rule except
  the interpretation clause, including those of Normative language, Tools in
  reviews, and Reviews, and over the duty to report a correctness defect defined
  in Rules and recommendations.

### Classification

#### Terms

- **Meaning-preserving change**: a change that keeps the behavior and meaning of
  what it touches, such as letter case or punctuation in prose, a typo fix, a
  rename, or a substitution by an equivalent construct.
- **Meaning-changing change**: any other change. A change to a value or name
  that a convention document requires other content to use is meaning-changing,
  even when it is a rename.

#### Rules

- A review MUST classify each change in its scope as one or the other and apply
  the rules below accordingly. It MAY omit the classification from its report.
- Meaning-preserving: the review MUST verify only that meaning is preserved and
  that references to anything renamed still resolve, and apply the
  normative-language check defined in Normative language. It MUST NOT
  re-evaluate the touched content against anything else.
- Meaning-changing: the review MUST evaluate the new meaning against the content
  that depends on it. For a convention document, the review MUST include in that
  content what the change makes noncompliant.

### Findings

#### Terms

- **Blocking finding**: a finding whose source is a correctness defect, a
  convention document, or a failure of a declared tool. A failure of a declared
  tool reaches the review from a run of the tool, such as in CI, one the user
  requests, or one the review makes as permitted in Tools in reviews.

#### Rules

- A review MUST state the source of each finding: a correctness defect, a
  convention document, a tool declared in this repository, or a skill or agent
  defined outside this repository.
- A review MUST NOT report as a finding a deviation from a rule at the
  Recommended level when the deviation has a valid reason.
- A review MUST report a finding from a skill or agent defined outside this
  repository as a non-blocking recommendation. When that skill or agent declares
  it blocking, or declares that it MUST be fixed, the review MUST mark it as a
  strong recommendation and name the skill or agent.
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

#### Rules

- Before it reports, a review MUST drop each finding that fails any item of this
  checklist:
  - A finding MUST be within the scope of the review.
  - A finding MUST state its source, and the source MUST apply: the cited rule
    MUST exist and cover the content, or the described defect MUST occur with
    the content as written.
  - A finding MUST NOT report what a declared tool checks, unless a failure of
    that tool reached the review as defined in Findings.
  - A finding MUST NOT repeat another finding.
- The review MUST also confirm that it applied the normative-language check to
  the content in its scope, for every change in its scope.

### Report

#### Rules

- A review report MUST state, for each finding, its source and the rule or
  defect it cites.
- It MUST NOT report or list the dropped candidates. It MUST keep each of them,
  with the checklist item it failed, in its session context, not in its report.
- A review that reports no findings MUST still state that it applied the
  checklist.

## Pull requests

### Rules

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
- A description MUST NOT ask a reviewer to run a declared tool only to check
  what that tool is expected to check on its own, such as in its `How to test?`
  section, as defined in Tools in reviews.

## Commits

### Rules

- A commit message MUST NOT carry a line that credits an agent, a model, or the
  tool that runs it as an author, or that links to an agent session, such as a
  `Co-Authored-By` trailer naming an agent or a session URL trailer.

## Memory

### Rules

- Memory MAY inform a task but MUST NOT override a rule, and an agent MUST
  handle a conflict between them as defined in User instructions.

## Skills

### Rules

- A skill or agent defined in this repository is a convention document: its file
  MUST carry the normative-language note defined in
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#doc-normative-language)
  and the convention note defined in
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#convention-documents),
  and its rules MUST bind like any other.
- A skill or agent defined outside this repository, including the user's own and
  third-party ones, MAY inform a task. An agent MUST treat what it states as
  recommendations, and MUST handle a conflict between it and a rule as defined
  in User instructions.
- Such a skill or agent MAY state preferences, such as formatting, layout, or
  naming choices, that agents follow when they create or edit content under it.
  An agent MUST treat a preference as a recommendation, and a review MAY report
  a deviation from it.

## User instructions

### Rules

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

### Rules

- Agents MUST NOT leave the working root for any operation they perform
  directly, such as reading, writing, or navigating files, or running a command
  whose working directory or path arguments are outside it.
- The previous rule MUST NOT be applied to files that a tool reads or writes on
  its own when the agent invokes it, nor to the agent's own memory.
- An explicit user instruction MAY authorize an operation outside the working
  root for the task at hand only. Even then, the agent MUST ask the user for
  confirmation before its first such operation.

## Navigation

### Rules

- Agents MUST navigate this repository by following local Markdown links, which
  keeps their context limited to the documents the task needs.

### Local links

#### Terms

- **Listing**: links presented as a list. It is an index, not a reading list.

#### Rules

- When an agent encounters a link to a local file whose content the task needs,
  it MUST read the file before continuing, resolving a relative path from the
  directory of the document containing the link.
- To follow a link, an agent MUST use its native file-reading tool, unless the
  link points to a section that this tool cannot read; in that case, it MAY use
  any other means to locate and read the section.
- Reading the links of a listing is OPTIONAL. An agent MUST open a listed file
  only when the task needs its content, not because it is listed. An entry MAY
  state when the task needs the file.
- As an exception, an agent MAY read only the metadata of a listed file, as
  defined in
  [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#doc-metadata), to
  determine whether the task needs it.
- An agent MUST treat a link inside an example, a code block, or a template as
  illustrating a format, not as navigation, and MUST NOT follow it unless the
  task needs the linked file for another reason.
- A link MAY resolve to any file inside the working root, including one above
  the linking document.
- When a link resolves outside the working root, the agent MUST NOT read the
  file, MUST notify the user, naming the link and the linking document, and MUST
  continue the task without that file.
- Agents MUST NOT search file contents as a substitute for following links,
  unless the user explicitly requests it. Reading a file and listing or
  filtering file names MUST NOT be counted as content search.
- When no link leads to content the task needs, an agent MAY locate it by file
  name.
- An agent MUST follow the local links the task needs in a destination file too.
- If a file does not exist or cannot be read, the agent MUST report it and MUST
  NOT assume its contents.

#### Best practices

- An agent SHOULD NOT reread files already consulted.

### HTTP links

This convention applies to every HTTP URL an agent fetches, whether it comes
from a Markdown link, from plain text, or from the agent itself.

#### Rules

- An agent MUST NOT fetch a URL unless the task needs content of it that the
  agent does not know in enough depth from its training.

#### Best practices

- An agent SHOULD request the page's raw-text form when the site provides one.

#### Saved content

##### Rules

- Before fetching, the agent MAY look in its own memory for content saved from
  that exact URL. When it differs from the page, the page MUST take precedence.
- When the URL is pinned to a revision or a commit and the saved content records
  the same URL, it is equivalent to the page, and the agent MAY use it instead
  of fetching.
- After fetching, the agent MAY save the content the task needed in its own
  memory, under the exact URL, and MUST NOT save it inside the working root.

##### Best practices

- When saved content is not equivalent to the page, as defined in Rules, it can
  be outdated, and the agent SHOULD confirm it against the page, or against
  evidence of when it was saved and when the page last changed, before relying
  on it.
- When it saves fetched content, the agent SHOULD use the memory it keeps for
  this project rather than its general memory.

## Related

- [README.md](./README.md): What Kraf is, its vision, and the index of the
  repository's documents.
- [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md): Repository
  conventions: how documents are written, how a convention works, and how the
  workspace is organized.
