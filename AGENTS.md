---
title: AGENTS
summary: Entry point for agent instructions and the repository conventions they MUST follow.
tags: [agents, instructions, repository, coding-conventions]
type: ["standard", "root"]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_STANDARDS.md](./docs/standards/GENERAL_STANDARDS.md#normative-language).
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords.

This document is the entry point for every agent task in this repository. Before
anything else:

- Every agent MUST read this whole document in full, not only a part of it,
  whenever its context may not hold the current version: when it resumes a
  session, when its context is summarized, and when the file changes during the
  session.
- Every agent, including a subagent, MUST read [INDEX.md](./INDEX.md) when it
  starts, and MUST follow the links that lead to other index documents, going
  down the tree until no further index can be reached, then backtracking to
  follow another branch. While traversing, agents MUST NOT open any document
  that is not an index. Agents MUST finish the traversal before continuing with
  the file or task at hand; only then do they open the documents that the task
  needs.
- Agents MUST start every task here and follow the links to the documents the
  task needs.

## Rules and recommendations

**Rule**: a requirement defined by a standard document, including a skill or
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

**Same work**: an operation whose entire result an available tool produces in
one call, such as showing the content of a file, searching the content of files,
listing files, or editing a file. An operation that composes steps, such as
reading, filtering, transforming, and counting, and whose entire result no
available tool produces in one call, is not the same work, even when one of its
steps is one that an available tool performs alone.

- Before executing a task, and before performing work through the shell or by
  its own means, an agent MUST search the tools available to it, including those
  that require loading, for the work that the task needs, and MUST load only
  those tools. An agent MUST NOT search or load tools unrelated to the task.
- An agent MUST NOT perform or verify by its own means work that an available
  tool performs.
- In a review, an agent MUST use native tools instead of shell commands, custom
  scripts, or other custom means that do the same work, unless a declared tool
  is declared for that work, in which case the declared tool MUST be used. A
  provided tool instead of reaching the same service through the shell or a
  custom means.
- When an agent uses a shell command for the same work because the native option
  is unavailable or cannot do it, it MUST state in its reply which native option
  was unavailable or could not do the work, and the command it ran.
- A shell command that performs only a step that an available tool performs
  alone MUST NOT be used (e.g. `cat` of a file to show its content, or `sed` to
  edit a file).
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
- When writing, an agent MUST use the declared tool for the work it is declared
  for. Native tools and anything that works as a custom script or a shell script
  do not replace it. If it cannot use the declared tool, or would deviate from
  it, the agent MUST inform the user and MUST NOT deviate unless the user
  explicitly instructs it.
- Once it finishes writing, it MUST run each declared tool for which
  [TOOLS.md](./docs/TOOLS.md) states capabilities and that checks the changed
  files, as stated under Run, if any, and according to its capabilities:
  - With autofix: the agent MUST run the fix. It MAY skip the check.
  - Otherwise: the agent MUST run the check, fix each reported problem, with the
    tool's fix when it has one and by its own means otherwise, and MUST run the
    check again to confirm that no problem remains.

### Tools in reviews

- A review MAY run a declared tool when it has a plausible reason, such as
  verifying a finding.
- A review MUST NOT run a declared tool only to check what that tool is expected
  to check on its own, such as through CI or a hook.
- When an agent writes in the same session as a review, such as applying a fix,
  that writing is not part of the review, and Writing content applies to it.

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
- A review MUST find each reference to anything the change renames or removes,
  by the search that Navigation permits for that purpose, and MUST evaluate each
  reference found.
- A review of a change to a standard document MUST read that document in full,
  not only the parts that changed.
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
a standard document requires other content to use is meaning-changing, even when
it is a rename.

- A review MUST classify each change in its scope as one or the other and apply
  the rules below accordingly. It MAY omit the classification from its report.
- Meaning-preserving: the review MUST verify only that meaning is preserved and
  that references to anything renamed still resolve, and apply the
  normative-language check defined in Normative language. It MUST NOT
  re-evaluate the touched content against anything else.
- Meaning-changing: the review MUST evaluate the new meaning against the content
  that depends on it. For a standard document, that content is each file that
  its `scope` field matches, as defined in its front matter, and it includes
  what the change makes noncompliant.

### Findings

- A review MUST state the source of each finding: a correctness defect, a
  standard document, a tool declared in this repository, or a skill or agent
  defined outside this repository.
- A correctness defect includes a rule that contradicts itself or another rule.
- A finding is blocking when its source is a correctness defect, a standard
  document, or a failure of a declared tool. Such a failure reaches the review
  from a run of the tool, such as in CI, one the user requests, or one the
  review makes as permitted in Tools in reviews.
- A finding from a skill or agent defined outside this repository is a
  non-blocking recommendation. When that skill or agent declares it blocking, or
  declares that it MUST be fixed, the review MUST mark it as a strong
  recommendation and name the skill or agent.
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

### Checklist

Before it reports, a review MUST drop each finding that fails any item of this
checklist:

- It is within the scope of the review.
- It states its source, and the source applies: the cited rule exists and covers
  the content, or the described defect occurs with the content as written.
- It does not report what a declared tool checks, unless a failure of that tool
  reached the review as defined in Findings.
- It does not repeat another finding.

The review MUST also confirm that it applied the normative-language check to the
content in its scope, for every change in its scope.

### Report

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
- It MUST NOT report or list the dropped candidates. It MUST keep each of them,
  with the checklist item it failed, in its session context, not in its report.
- A review that reports no findings MUST still state that it applied the
  checklist.

Template of the report:

````markdown
## Findings

1. ID: <id>
   - Recommendation: <blocking | strong recommendation, must be fixed |
     recommendation>
   - Name: <name>
   - Detail: <detail, with the line of the origin>
   - Source: <repository rule, declared tool, built-in skill, external skill, or
     correctness defect>
   - File: <path relative to the repository>
   <details>
   <summary>

   #<id>

   ```text
   <raw finding>
   ```

   </summary>

   </details>

## Files read

- <path>
````

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
- A description MUST NOT ask a reviewer to run a declared tool only to check
  what that tool is expected to check on its own, such as in its `How to test?`
  section, as defined in Tools in reviews.

## Commits

- A commit message MUST NOT carry a line that credits an agent, a model, or the
  tool that runs it as an author, or that links to an agent session, such as a
  `Co-Authored-By` trailer naming an agent or a session URL trailer.

## Memory

- Memory MAY inform a task but MUST NOT override a rule. A conflict between them
  is handled as defined in User instructions.

## Skills

- A skill or agent defined in this repository is a standard document: its file
  MUST carry the normative-language note defined in
  [GENERAL_STANDARDS.md](./docs/standards/GENERAL_STANDARDS.md#normative-language)
  and the standard note defined in
  [DOC_STANDARDS.md](./docs/standards/DOC_STANDARDS.md#standard-documents), and
  its rules bind like any other.
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
- A review MAY search file contents for the name of anything the change renames
  or removes, to find the references that its Scope requires. This is the only
  content search that a review MAY make, and it MUST NOT be used to find other
  content.
- An index is identified first by its name and location, as the
  [Docs graph](./docs/standards/GENERAL_STANDARDS.md#link-navigation-document-graph)
  rules define: the `README.md` of a folder is an index, and `INDEX.md` is an
  index only at the root. An agent MUST then read the front matter of an index
  to confirm its `type` and `scope`: a folder `README.md` that is an index
  declares both `readme` and `index` in `type`, and the root `INDEX.md` declares
  `root`. A document without front matter, or with a different `type`, is a
  finding; it does not make the document not an index.
- During traversal, an agent MUST read the full content of a document whose
  `type` is only `index`.
- During traversal, for a document whose `type` lists `index` and another type,
  an agent MUST read its front matter and its Links section, from the Links
  heading to the next level-2 heading or to the end of the file, whichever comes
  first.
- During traversal, for any other document, an agent MUST read only its front
  matter.
- The traversal rules above apply only while the agent is traversing the
  document graph.
- Outside traversal, an agent MAY read a document in any way, including in full.
- Outside traversal, an agent SHOULD read the front matter first to decide
  whether to continue reading.
- A document is also reached by the `type` and `scope` fields of its front
  matter. Locating a document by those fields counts as following a link, when
  the task requires it, such as reviewing the documents that depend on a
  standard. An agent MUST read only the front matter for that, and MUST NOT use
  other content to locate documents.
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

## Links

- [README.md](./README.md): what Kraf is and more.
- [INDEX.md](./INDEX.md): Index of the repository's documents, modules and other
  files.
- [GENERAL_STANDARDS.md](./docs/standards/GENERAL_STANDARDS.md): repo-wide
  conventions: standard documents and rule precedence, docs, workspace, naming

```
```
