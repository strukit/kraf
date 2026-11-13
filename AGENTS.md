---
title: AGENTS
summary: Entry point for agent instructions and the repository conventions they MUST follow.
tags: [agents, instructions, repository, coding-conventions]
type: ["root","standard"]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_STANDARDS.md](./docs/standards/GENERAL_STANDARDS.md#normative-language).
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords.

> [!IMPORTANT]
> This is a standard document. It works like a constitution. Its rules MUST NOT
> be bypassed by ad-hoc exceptions

This document is the entry point for every agent task in this repository. Before
it performs any task, an agent completes the steps below, in this order.

1. An agent MUST read **[AGENTS.md](./AGENTS.md)** in full, to the end, before
   proceeding. It MUST read it again whenever its context may not hold the
   current version—when it resumes a session, when its context is summarized, or
   when the file changes during the session. This ensures the agent always has
   the current instructions.
2. It MUST then search the tools available to it, including those that require
   loading, as defined in [Choosing a tool](#choosing-a-tool), and MUST list,
   for each one it may use, what it does and what it does not do that the
   discovery or the task needs. For example, a native file-reading tool MAY not
   stop at the end of a front matter, so it might not be the best choice for
   reading only a document's front matter.
3. It MUST then complete the repository discovery, as defined in
   [Discovery](#discovery):
   - It MUST read [README.md](./README.md) as an index and follow each link that
     points to an index.
   - It MUST read each index it encounters as defined in
     [Discovery](#discovery).
   - When an index has no more links to other indexes, it MUST backtrack and
     follow other branches.
   - It MUST complete the entire discovery before it opens any task document.
4. Only after these steps, it MUST open the documents the task needs, as defined
   in [Navigation](#navigation).

## Rules and recommendations

**Rule**: a requirement defined by a standard document, including a skill or
agent defined in this repository. It binds with the force of its keyword, as RFC
2119 defines it: a SHOULD or RECOMMENDED rule, for example, allows a deviation
for a valid reason.

**Recommendation**: what a skill or agent defined outside this repository
states, as defined in [Skills](#skills). It is not a rule and does not bind.

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
- An agent MUST NOT do work unless the user explicitly requests it, a standard
  document or a skill or agent that the task uses requires, recommends, or
  permits it as part of the task's work, or the task cannot be completed without
  it.

## Normative language

- An agent that writes or edits a document MUST verify that what it writes
  follows the normative-language rules for that document: those of
  [GENERAL_STANDARDS.md](./docs/standards/GENERAL_STANDARDS.md#normative-language)
  and, for a specification or an intent, those of
  [SPEC_STANDARDS.md](./docs/standards/SPEC_STANDARDS.md).

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

**Same work**: an operation whose entire result an available tool produces in
one call, such as showing the content of a file, searching the content of files,
listing files, or editing a file. An operation that composes steps, such as
reading, filtering, transforming, and counting, and whose entire result no
available tool produces in one call, is not the same work, even when one of its
steps is one that an available tool performs alone.

- Before executing a task, and before performing work through the shell or by
  its own means, an agent MUST search the tools available to it, including those
  that require loading, for the work that the discovery or the task needs, and
  MUST load only those tools. An agent MUST NOT search or load tools unrelated
  to the discovery or the task.
- An agent MUST NOT perform or verify by its own means work that an available
  tool performs.
- When an agent uses a shell command for the same work because the native option
  is unavailable or cannot do it, it MUST state in its reply which native option
  was unavailable or could not do the work, and the command it ran.
- A shell command that performs only a step that an available tool performs
  alone MUST NOT be used (e.g. `cat` of a file to show its content, or `sed` to
  edit a file).
- As an exception, an agent MAY read by any means, including a shell command,
  the lines that a [structure lookup](#local-links) found.
- An agent MUST NOT replace a shell command that does the same work with another
  program, script, or language that does it (e.g. `awk`, a Python script):
  changing the means does not change the work.
- An agent MAY use the shell for an operation that is not the same work,
  including its read steps (e.g. a pipeline that reads a file, keeps the lines
  that contain a word, and keeps the first five of them, or reads a large file
  and filters it, where a native read would load all of it into context). When
  it does, it MUST state in its reply the command and that no available tool
  produces the entire result. It MUST NOT build such an operation only to avoid
  using a tool for a step.
- An agent MUST use a declared tool for the purpose it is declared for.
- When [the list](./docs/TOOLS.md#tools) states that a declared tool is invoked
  by another, using the invoking tool counts as using the invoked one. An agent
  SHOULD use the invoking tool and SHOULD NOT invoke the other directly.
- When a tool is not available, the agent MUST inform the user and continue.
- An agent SHOULD invoke in parallel the tool calls that do not depend on each
  other, especially native ones.

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
- When writing, an agent MUST use the declared tool for the work it is declared
  for. Native tools and anything that works as a custom script or a shell script
  do not replace it. If it cannot use the declared tool, or would deviate from
  it, the agent MUST inform the user and MUST NOT deviate unless the user
  explicitly instructs it.
- Once it finishes writing, it MUST run each declared tool for which
  [TOOLS.md](./docs/TOOLS.md) states capabilities and that checks the changed
  files, as stated under [Run](./docs/TOOLS.md#tools), if any, and according to
  its capabilities:
  - With autofix: the agent MUST run the fix. It MAY skip the check.
  - Otherwise: the agent MUST run the check, fix each reported problem, with the
    tool's fix when it has one and by its own means otherwise, and MUST run the
    check again to confirm that no problem remains.

## Delegation

**Delegation**: an agent hands work to another agent, such as a sub-agent.

- A delegating prompt MUST explicitly identify which permissions and authority
  have been delegated by the caller. Any permission not explicitly delegated
  MUST be treated as unavailable.
- A delegating prompt MUST preserve the authority and intent of the original
  instructions. It MUST NOT restate, reinterpret, summarize, reorder, or replace
  them in a way that changes their meaning or precedence.
- **Caller**: the user, or the agent, that hands the work to the delegating
  agent.
- A delegating prompt MUST give the delegate the work context that identifies
  the state under review or modification: the repository, the pull request or
  task reference when applicable, and, when applicable, the base revision,
  target revision, branch, and commit hashes.
- A delegating prompt MUST mark each instruction that comes from the caller as a
  caller-provided instruction. A caller-provided instruction MUST NOT be treated
  as an implicit or inferred permission.
- A delegating prompt MUST instruct the delegate to read
  [AGENTS.md](./AGENTS.md) and to follow its [Navigation](#navigation) before
  any delegated work, with the working tree at the target revision. The delegate
  MUST NOT skip that process.
- A delegating agent MUST pass the current content of this document to the
  delegate when it knows the delegate will not receive this document through its
  own context, including when the prompt is generated by a skill.
- The delegating agent MUST NOT replace the delegate's navigation by handing it
  a manually assembled set of documents or instructions that bypasses
  [Navigation](#navigation).

## Pull requests

- A pull request description that an agent writes or updates MUST follow
  [the pull request template](./.github/pull_request_template.md) and fill in
  each of its sections.
- A description MUST NOT contain a section that
  [the template](./.github/pull_request_template.md) does not define.
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
  section, as defined in [Tools in reviews](./REVIEW.md#tools-in-reviews).

## Commits

- A commit message MUST NOT carry a line that credits an agent, a model, or the
  tool that runs it as an author, or that links to an agent session, such as a
  `Co-Authored-By` trailer naming an agent or a session URL trailer.

## Memory

- Memory MAY inform a task but MUST NOT override a rule. A conflict between them
  is handled as defined in [User instructions](#user-instructions).

## Skills

- A skill or agent defined in this repository is a standard document: its file
  MUST carry the normative-language note defined in
  [GENERAL_STANDARDS.md](./docs/standards/GENERAL_STANDARDS.md#normative-language)
  and the standard note defined in
  [DOC_STANDARDS.md](./docs/standards/DOC_STANDARDS.md#standard-documents), and
  its rules bind like any other.
- A skill or agent defined outside this repository, including the user's own and
  third-party ones, states recommendations. It MAY inform a task. A conflict
  between it and a rule is handled as defined in
  [User instructions](#user-instructions).
- Such a skill or agent MAY state preferences, such as formatting, layout, or
  naming choices, that agents follow when they create or edit content under it.
  A preference is a recommendation.

## User instructions

**Explicit user instruction**: one the user gives in the task itself. An
instruction loaded from a source that persists across tasks, such as a personal
instruction file or the agent's memory, is not explicit, even when it is the
user's own.

- An explicit user instruction MUST be respected, as defined in
  [Rules and recommendations](#rules-and-recommendations).
- When an agent finds a conflict between a rule and a non-explicit instruction,
  including memory, or a skill or agent defined outside this repository:
  - It MUST continue the task following the rule.
  - It MUST notify the user, briefly stating which instruction and which rule
    are involved and that, if the content is pertinent, it needs to be committed
    to a standard document to take effect.
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

This section defines how agents navigate this repository.

- Agents MUST navigate this repository by following local Markdown links, to
  keep their context limited to the documents the task needs.
- Following links MUST always be the first means of navigation. An agent MUST
  NOT use another means, such as locating a file by name or making a
  [content search](#local-links), before it has followed the links that lead
  toward what the task needs.
- An agent MAY use another means only when no link leads to what the task needs,
  or when the task cannot be completed by following links, and only as
  [this section](#navigation) permits.

### Local links

**Index**: the `README.md` of a folder, including the repository root, as the
[Docs graph](./docs/standards/GENERAL_STANDARDS.md#link-navigation-document-graph)
rules define.

**Listing**: a list of links inside a document. A listing is not an index.

**Discovery**: the phase, before any task work, in which an agent starts at
[README.md](./README.md) and follows each link to an index, until it has visited
every index reachable from the root.

**Content search**: matching a pattern or a string against the content of files
to find a file, or content in it, that no link in a document the agent has read
points to.

**Structure lookup**: matching the lines that delimit a part of a Markdown
document that a link in a document the agent has read points to, such as its
front matter, a heading, or its Links section, to find where that part starts
and ends, whether or not the agent has read that document yet. It is not content
search.

#### Following links

- When an agent encounters a link to a local file whose content the task needs,
  it MUST read the file before continuing, resolving a relative path from the
  directory of the document containing the link.
- To follow a link, an agent MUST use its native file-reading tool whenever that
  tool can read what the rule requires, except as [Discovery](#discovery)
  permits.
- A link MAY resolve to any file inside the working root, including one above
  the linking document.
- When a link resolves outside the working root, the agent MUST NOT read the
  file, MUST notify the user, naming the link and the linking document, and MUST
  continue the task without that file.
- A link inside an example, a code block, or a template illustrates a format and
  is not navigation. An agent MUST NOT follow it unless the task needs the
  linked file for another reason.
- An agent MUST follow the local links the task needs in a destination file too,
  and SHOULD NOT reread files already consulted.
- If a file does not exist or cannot be read, the agent MUST report it and MUST
  NOT assume its contents.
- Outside discovery, reading the links of a listing is OPTIONAL. An agent MUST
  open a listed file only when the task needs its content, not because it is
  listed, and MAY read only its front matter to decide. An entry MAY state when
  the task needs the file.

#### Discovery

- An agent MUST identify an index by its name and location, not by its `type`,
  and MUST then read its front matter to confirm its `type`. A `type` that does
  not match the Docs graph rules is a finding; it does not make the document not
  an index.
- An agent MUST choose which documents to read only from the Links sections of
  the indexes it visits, and MUST NOT use any other source, such as a pull
  request description, the task prompt, or its memory, to choose them.
- For each index, an agent MUST read its front matter and its whole Links
  section, including its subsections.
- For each document that is not an index and that an index links to in its Links
  section, an agent MUST read only its front matter, to learn its `type` and
  `scope`.
- An agent MUST read only the part that a rule requires, ending exactly at its
  end. It MUST NOT read it with a number of lines that a structure lookup did
  not find, even one large enough to hold the whole part.
- When its native file-reading tool cannot find that part alone, the agent MAY
  use a structure lookup and then read the lines it found, as
  [Choosing a tool](#choosing-a-tool) permits.
- When neither can, the agent MUST search once, before the discovery, the tools
  available to it for one that can, and MAY use the shell or a script only when
  none can.
- Whatever the means, an agent MUST NOT locate files by name or name pattern,
  and MUST NOT make a content search.
- An agent MUST note the hash of the `HEAD` commit at which it completes the
  discovery.

#### After discovery

- An agent MUST read in full each standard document whose `scope` matches a file
  that the task reads or changes.
- At the start of each task, an agent MUST list the files changed since the
  commit it noted, including uncommitted ones.
- When a document whose `type` lists `root`, `standard`, or `index` changes
  after the agent read it, including by a change under review, the agent MUST
  read it again and follow the new version. For an index, it MUST also redo the
  discovery from it.
- An agent SHOULD read the front matter of a document first, to decide whether
  to continue reading.
- When the task requires it, such as reviewing the documents that depend on a
  standard, an agent MAY locate a document by the `type` and `scope` fields of
  its front matter, which counts as following a link. It MUST read only the
  front matter for that, and MUST NOT use other content to locate documents.

#### Other means

- When no link leads to content the task needs, an agent MAY locate it by file
  name.
- An agent MUST NOT make a content search as a substitute for following links.
  Reading a file, a structure lookup, and listing or filtering file names are
  not content search.
- An agent MAY make a content search only:
  - when the user explicitly requests it;
  - to find the references that scope requires a change to update, or that a
    review's scope requires, and no other content;
  - when the task cannot be completed by following links, such as finding the
    documents that link to a file, after it has tried to reach the content
    through them.
- A search under the last exception MUST target specific content, such as a
  pattern or a defined string, MUST be limited to the files or folders where
  that content can be, and MUST serve only that step. The agent MUST then return
  to following links.

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

## Links

- [README.md](./README.md): What Kraf is and its vision, and the index of the
  repository's documents, modules and other files.
- [REVIEW.md](./REVIEW.md): Review standards for this repository, their
  interpretation, scope, classification, normative-language check, findings,
  checklist, report template, and use of tools.
- [GENERAL_STANDARDS.md](./docs/standards/GENERAL_STANDARDS.md): repo-wide
  conventions: standard documents and rule precedence, docs, workspace, naming
