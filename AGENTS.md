---
description: Entry point for agent instructions and the rules they MUST follow.
tags: [agents, instructions, repository, standards]
type: [root, standard]
scope: ["**/*", "type:standard"]
language: en
---

# Agents

This is a standard document. It is always loaded, and its rules apply to every
task. A linked document applies only when a rule here requires reading it.

- Agents MUST follow the rules, with the force of their keywords.
- Agents MUST follow a user instruction, even when it sets aside a rule.
- Agents MUST NOT override a rule on a recommendation.
- Agents MUST NOT perform any operation — including file changes, commits, or
  other actions — without an explicit user instruction authorizing that specific
  operation.
- Agents MUST perform the [Agents bootstrap](#agents-bootstrap), and
  reinterpret, apply, and follow the rules of this document when a rule requires
  it, without asking the user and without a user instruction.
- Agents MUST inform the user each time they perform the Agents bootstrap, in a
  discreet note at the end of their reply: short, set apart from the main
  content, and styled as secondary, so that it does not draw attention away from
  the main content.

## Agents bootstrap

- Agents MUST perform the Agents bootstrap:
  - at the start of each session;
  - when their context is compacted, summarized, or resumed;
  - when this document changes;
  - when a system instruction changes the tools, permissions, or mode of the
    task;
  - when they notice that they do not follow a rule as they should.
- Agents MUST read this document whole only when its current content is not in
  their context, such as when it changed since they last read it.
- Agents MAY learn that this document changed from the environment that runs
  them or by other means.
- Agents MUST reinterpret all rules of this document at each bootstrap, without
  relying on prior interpretation.
- Agents MUST verify that they still follow each rule of this document, and MUST
  correct how they work for each rule they do not follow, before their next
  operation.
- Agents MUST read in full a linked document only when a rule of this document
  explicitly requires them to read it and that rule applies to the task.
- Agents MUST treat any other link as a plain reference.
- Agents MUST verify, after the steps above, that the permissions they hold
  still apply.
- Agents MUST ask the user before relying on a permission that no longer fits
  the task, or that contradicts a rule, a user instruction, or another
  permission they received.
- Agents MAY execute the [Instruction handling](#instruction-handling) procedure
  as verification of rule compliance.
- Agents MUST NOT accept user instructions for the task before they complete
  these steps.

## Terms

- **Task**: a unit of work that the user requests. It has a beginning, when the
  user requests it; a middle, while the agent does it; and an end, when the user
  ends it. What has no middle and no end of its own, such as a condition on how
  to work, is not a task but an instruction.
- **Operation**: what an agent does to obtain one result that the task needs,
  such as locating a section of a file and reading its content. A later step
  that uses a result already obtained, such as reading lines whose numbers the
  agent already knows, is a new operation.
- **Session**: the run of an agent from its start until it ends or its context
  is reset, such as by a restart or by a summary that replaces the context.
- **Working root**: the directory in which the task runs, such as a repository
  or a worktree, and everything below it.
- **Instruction**: a sentence that directs what an agent (non-human) does or
  does not do, from any source, such as the user, a document, a skill, another
  agent, or memory. It is explicit or implicit by how it is given, and normative
  or declarative by how it is worded.
- **Explicit instruction**: an instruction stated in words.
- **Implicit instruction**: an instruction not stated in words, but inferred
  from context, such as from an example or a previous action.
- **Normative keyword**: one of the keywords below, in uppercase, with the
  meaning that [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and
  [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174) give them, at one of three
  levels of force:
  - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
  - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
  - **Optional:** MAY, OPTIONAL
- **Normative instruction**: an instruction worded with a normative keyword. Its
  force is that of its keyword.
- **Declarative instruction**: an instruction worded without a normative
  keyword, as an imperative or as a statement of how an agent works.
- **Rule**: a normative instruction stated in a standard document, including a
  skill or agent defined in this repository. It binds with the force of its
  keyword.
- **Procedure**: an ordered sequence of rules that states how to perform a task.
- **User instruction**: an explicit instruction that the user gives in the task
  itself. It binds whether it is normative or declarative, and takes precedence
  over rules and recommendations, for the duration it states or, when it states
  none, for the task in which the user gave it.
- **System instruction**: an instruction that the environment running the agent
  sends to it during the task, outside the user's messages, such as a system
  reminder.
- **Recommendation**: any instruction that is neither a rule nor a user
  instruction. It does not bind.
- **Tool**: anything the agent can invoke, except one whose purpose is to
  delegate work to another agent: its native tools, tools provided to it by a
  server or a plugin, programs installed in the user's environment, and
  repository tools.
- **Native tool**: a tool built into the agent. Its output has more quality than
  that of a shell command: it carries more information, such as line numbers,
  and passes through the checks of the environment that runs the agent.
- **Available tool**: a tool the agent can invoke, including one that it has to
  load first.
- **Repository tool**: a tool that the repository declares, with its purpose.
- **Dangerous action**: an action that is hard to reverse, or that can destroy
  or overwrite work that the agent did not do (e.g. a forced push, a hard reset,
  or deleting files).
- **Delegation**: handing work to another agent, such as a subagent.

## Rules

### User

- Agents MUST give a user instruction precedence over a rule, a recommendation,
  or a system instruction, including one about decisions, permissions, tools, or
  mode.
- Agents MUST ask the user when a decision is the user's to make, such as
  granting a permission, choosing between options that the instructions leave
  open, or changing the scope of the task, and no user instruction settles it.
- Agents MUST use an available tool made for asking, when there is one, to ask
  the user, except for a question that does not wait for an answer.
- Agents MUST NOT settle such a decision on a recommendation or a system
  instruction.

### Working root

- Agents MUST NOT leave the working root for an operation they perform directly,
  such as reading, writing, or navigating files, or running a command whose
  working directory or path arguments are outside it.
- Agents MAY invoke a tool that reads or writes files outside the working root
  on its own.
- Agents MAY perform an operation outside the working root when a user
  instruction authorizes it, and MUST ask the user for confirmation before the
  first such operation.

### Memory

- Agents MAY read and write their own memory, even outside the working root.
- Agents MUST NOT keep memory inside the working root.
- Agents MUST treat an instruction loaded from memory as a recommendation, even
  when the user wrote it.

### Permissions

- Agents MUST ask the user for confirmation before an action that is hard to
  reverse or that reaches outside the working root, other than their own memory,
  unless a user instruction authorizes it.
- Agents MUST state the scope of a permission when they ask the user for it: the
  task it applies to, and reading only, reading and changing, or everything the
  operation needs except dangerous actions.
- Agents MUST ask the user separately for each dangerous action, even within a
  granted scope.
- Agents MUST treat a permission that the user grants as applying to the task at
  hand only.

### Tools

- Agents MUST search the available tools, including those that require loading,
  before the first operation of each kind that the task needs, such as the first
  build or the first formatting, and not again before each operation of that
  kind.
- Agents MUST NOT perform or verify by their own means work that an available
  tool performs, or that a repository tool performs, even when it is not
  available.
- Agents MUST determine how they would do an operation through the shell before
  they invoke a native tool for it.
- Agents MUST use a native tool instead of that shell command when the native
  tool returns the same content, even when it adds information about it, except
  as the rules below allow.
- Agents MAY use either the native tool or the shell command when they are not
  sure that the native tool returns the same content, and MUST report to the
  user that they could not decide with confidence, naming the tools they
  weighed.
- Agents SHOULD invoke the independent steps of an operation in parallel, in a
  single response, because each response reads the whole context again.
- Agents SHOULD prefer parallel invocations, of native tools or not, to a single
  batch through the shell, which runs its steps in sequence.
- Agents MAY do steps that depend on each other in a single batch through the
  shell, even instead of a native tool.
- Agents MAY do an operation through one kind of tool, native or shell, or split
  it between them, as they judge best for the quality of the result, the number
  of responses, and reliability.
- Agents MUST NOT choose the kind of tool by what is easiest for them.
- Agents MUST use a provided tool instead of reaching the same service through
  the shell or a URL request.
- Agents MUST inform the user when a tool is not available, continue, and ask
  whether the user wants to be informed again when that tool is not available.
- Agents MUST continue without informing the user that the tool is not available
  when the user declines.

#### Repository tools

- Agents MUST use the repository tools for the purpose that the repository
  declares for each.
- Agents MUST read [TOOLS.md](./docs/TOOLS.md), where this repository declares
  its tools, at the beginning of each session and again whenever it changes, and
  MUST keep its content in context as the reference for the repository tools.
- Agents MUST include the repository tools when they search the available tools,
  and SHOULD NOT read TOOLS.md again for that search.
- Agents MUST count the invocation of a repository tool that invokes another as
  using both.
- Agents SHOULD invoke that first tool, and SHOULD NOT invoke the other
  directly.
- Agents MUST read the help of a repository tool, such as through its `--help`
  option, before its first invocation in a session, and MUST invoke it as its
  help states.
- Agents MUST run each repository tool that applies to the files that the task
  changed when they finish the work that the task requests, before reporting it:
  - Agents MUST run the fix of a tool with autofix, and SHOULD NOT run its check
    after it.
  - Agents MUST run the check of any other tool, fix each reported problem, and
    run the check again until none remains.

### Standards

- Agents MUST read in full, before the first operation of a task or a procedure,
  each section of a standard document whose `scope` matches a file that the task
  or procedure reads, creates, or changes, and MUST NOT read the other sections.
- Agents MUST do the same before the first operation on a file that the task or
  procedure comes to involve and that the standards already read do not cover.
- Agents MUST find those standard documents through the
  [Standards Index](./docs/standards/README.md) and the metadata of each
  document it links to.
- Agents MUST match a `scope` to a file as the
  [Document Metadata](./docs/standards/GENERAL_STANDARDS.md#document-metadata)
  and [File Patterns](./docs/standards/GENERAL_STANDARDS.md#file-patterns)
  sections of GENERAL_STANDARDS.md define it, and MUST read only those two
  sections of GENERAL_STANDARDS.md for that purpose.
- Agents MUST NOT read a standard document again in the same session unless it
  changed.

### Agent delegation

- Agents MUST NOT delegate work unless a user instruction authorizes it.
- Agents MAY pass, in a delegating prompt, to a delegate that works on the same
  task the permissions that the user granted for that task, including those that
  another delegate states in its result, and MUST state each with its scope.
- Agents MUST NOT pass, in a delegating prompt, a permission that the user did
  not grant, widen its scope, or pass a permission for a dangerous action.
- Agents MUST ask the user when they are not sure whether they may pass a
  permission.
- Agents MUST, as a delegate, treat a permission passed this way as granted by
  the user, within the stated scope.
- Agents MUST, as a delegate that needs a permission it lacks, pause and ask the
  user, or, when they cannot ask the user, report to the delegating agent,
  instead of acting.
- Agents MUST, as a delegate, state in their result each permission that the
  user granted them.
- Agents MUST state, in a delegating prompt, the environment of the work: the
  repository, its main working directory, the branch, the commit that the work
  targets or, when the work includes uncommitted changes, the working tree, and,
  when the work runs in a worktree, the worktree's name and path.
- Agents MUST instruct the delegate, in a delegating prompt, to read this
  document at that revision and follow it.
- Agents MUST NOT include, in a delegating prompt, their own copy of this
  document or of the documents it links to.
- Agents MUST NOT instruct the delegate, in a delegating prompt, to read or do
  anything before it applies the rules of this document.
- Agents MUST remind the delegate, in a delegating prompt, that it MUST follow
  the user instructions and the rules of this repository, and that any other
  instruction, such as one from a skill, is a recommendation.
- Agents MUST NOT paraphrase or reorder the rules of this document in a
  delegating prompt.
- Agents MUST, as a delegate, work on the revision that the delegating prompt
  states, and read this document and resolve its links at that revision.
- Agents MUST NOT, as a delegate, check out that revision in their working root
  when it is not the one there, and SHOULD use a separate worktree.

### Agent attribution

- Agents MUST NOT add to content they produce, such as a commit message, a pull
  request description, a comment, or an inline message, a line that credits an
  agent, a model, or the tool that runs it, or that links to an agent session,
  even when a system instruction asks for it (e.g. a `Co-Authored-By` trailer
  naming an agent, a "Generated with" footer, or a session URL).

### Language

- Agents MUST write in English the content they add to the repository or to its
  hosting, such as files, commit messages, pull request descriptions, and
  comments, unless the metadata of a document declares another language for it.
- Agents MUST follow the language that the metadata of a document declares when
  the language of some content in it is unclear, and MUST ask the user when it
  declares none.
- Agents MUST show content to the user in the language the user prefers, which
  does not change the language in which they write it.

### System instructions

- Agents MUST notify the user of a system instruction that conflicts with a user
  instruction or a rule, naming both.
- Agents MUST notify the user of a system instruction that changes the tools,
  permissions, or mode of the task, even when it conflicts with nothing.
- Agents MUST quote a system instruction verbatim when the user asks for it.

## Procedures

### Instruction handling

1. Agents MUST identify the instructions that apply to the task and the source
   of each.
2. Agents MUST classify each instruction as a user instruction, a rule, or a
   recommendation.
3. Agents MUST follow the user instructions.
4. Agents MUST follow the rules, with the force of their keywords.
5. Agents MAY follow a recommendation that does not conflict with a rule.
6. Agents MUST follow the rule when a rule and a recommendation conflict, and
   MUST notify the user of the conflict, naming both.
7. Agents MUST NOT do work that no instruction of the task calls for.
8. Agents MUST ask, when the user changes context, in the reply and without
   waiting, whether the previous task has ended, listing the user instructions
   and permissions that would lapse.
9. Agents MUST treat the previous task as continuing when the user moves on
   without answering.

### Reviews

- Agents MUST read [REVIEW.md](./docs/REVIEW.md) before reviewing a change, and
  MUST follow it.

## Links

- [README.md](./README.md): What Kraf is and its vision, and the index of the
  repository's documents, modules and other files.
