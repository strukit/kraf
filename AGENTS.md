---
title: AGENTS
summary: Entry point for agent instructions and the repository conventions they MUST follow.
tags: [agents, instructions, repository, coding-conventions]
applies_to: [all, agents]
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

This document is the entry point for every agent task in this repository. Before
starting any task, agents MUST start here and follow the documentation links to
documents relevant to the task.

Agents MUST act only on rules defined in convention documents and on skills and
agents that apply them. They MUST NOT raise, flag, or fix a concern that no
convention document defines, such as choosing one approach, name, or style over
another, except as a recommendation from a skill or agent defined outside this
repository that the task uses, as defined in Skills.

Agents MUST report a correctness defect they find, such as code that produces a
wrong result, fails, or contradicts its spec, even when no convention document
defines it.

Besides the exceptions this document defines, an explicit user instruction MAY
set aside a rule for the task at hand only, and does not change the rule. Agents
MUST NOT override a rule on their own preference or judgment.

## Tools

A tool is any program or service the agent can invoke, other than one that runs
an agent or a language model: its native tools, tools provided to it, programs
installed in the user's environment, and tools declared in this repository.

An agent MUST NOT perform or verify by its own means work that an available tool
performs. It MUST use its native tools instead of shell commands that do the
same work, and a provided tool instead of reaching the same service through the
shell.

When this repository declares a tool for a purpose, an agent MUST use that tool
for that purpose. A tool is declared only when it is listed in
[Tools configured in this repository](./README.md#tools-configured-in-this-repository).
A pull request that adds or removes a tool's configuration MUST update that
list. An agent SHOULD build with cargo rather than Docker Bake, unless the task
needs a clean, containerized build.

When an agent writes content, it MUST write it to be readable and MUST NOT adjust
it to match the output that a tool would produce from it.

An agent MUST NOT do work that the task does not need. A task needs work only
when the user explicitly requests it, a convention document or a skill or agent
that the task uses requires, recommends, or permits it as part of the work the
task performs, or the task cannot be completed without it.

## Reviews

The scope of a review is the change under review, the references to anything the
change renames, and, for a meaning-changing change, the content that depends on
it. A review MUST NOT validate or report content outside that scope, including a
defect that predates the change. Within reviews, this rule MUST take precedence
over the duty to report a correctness defect.

A review MUST determine, for each change in its scope, whether it is
meaning-preserving or meaning-changing, and MUST apply the rules below
accordingly. A meaning-preserving change keeps the behavior and meaning of what
it touches, such as letter case or punctuation in prose, a typo fix, a rename,
or a substitution by an equivalent construct. A change to a value or name that a
convention document requires other content to use is meaning-changing. A review
MAY omit this classification from its report.

For a meaning-preserving change, the review MUST verify only that the change
preserves meaning and that references to anything it renames still resolve. It
MUST NOT re-evaluate the touched content against anything else.

For a meaning-changing change, the review MUST evaluate the new meaning against
the content that depends on it.

For a meaning-changing change to a convention document, the content that depends
on it includes the content that the change makes noncompliant. Content that the
pull request description leaves for a later pull request is outside the scope.

A review MUST state the source of each finding: a correctness defect, a
convention document, a tool declared in this repository, or a skill or agent
defined outside this repository. A finding is blocking when its source is a
correctness defect, a convention document, or a failure of a tool declared in
this repository. A correctness defect includes a rule that contradicts itself or
another rule. A finding from a skill or agent defined outside this repository is
a recommendation and is non-blocking. When that skill or agent declares the
finding blocking or required, the review MUST mark it as strongly recommended
and name the skill or agent. An explicit user instruction MAY change whether a
finding is blocking, for the task at hand only.

A review MUST NOT report or list missing content unless a convention document
requires that content, including by defining its absence as a defect, or a skill
or agent that the task uses expects it.

## Memory

Memory MAY inform a task but MUST NOT override a rule. When it conflicts with a
rule, the agent MUST follow the rule and MUST notify the user of the conflict,
stating that, if the content is pertinent, it needs to be committed to a
convention document to take effect.

## Skills

A skill or agent defined in this repository is a convention document: its file
MUST carry the normative-language note defined in
[GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#normative-language) and
the convention note defined in
[GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#convention-documents),
and its rules bind like those of any other convention document.

A skill or agent defined outside this repository, including the user's own and
third-party ones, states recommendations, not rules. It MAY inform a task but
MUST NOT override a rule. When it conflicts with a rule, the agent MUST follow
the rule and MUST notify the user of the conflict, stating that, if the content
is pertinent, it needs to be committed to a convention document to take effect.

Such a skill or agent MAY state preferences, such as formatting, layout, or
naming choices, that agents follow when they create or edit content under it. A
preference is a recommendation: it MUST NOT override a convention document. A
review MAY report a deviation from it, as defined in Reviews.

## User instructions

An explicit user instruction is one the user gives in the task itself. An
instruction the agent loads from a source that persists across tasks, including
a personal instruction file and the agent's memory, is not explicit, even when
it is the user's own. An
explicit user instruction MUST be respected, as defined above.

An instruction that is not explicit can conflict with a rule. When an agent
finds such a conflict, it MUST notify the user, briefly stating that the
conflict exists and which instruction or rule is involved, and it MUST continue
the task following the rule. When the conflict is with memory, the notice MUST
also state what the Memory section requires. The agent MUST NOT suppress the
conflict silently.

When following the rule would cause an effect that is hard to reverse or that
reaches outside the working root, the agent MUST stop and ask the user before
continuing.

## Workspace boundary

Agents MUST NOT leave the working root for any operation they perform directly,
including reading a file, writing a file, navigating files, or executing a
command whose working directory or path arguments are outside the working root.

The rule applies to the agent's direct operations only. Files that a tool reads
or writes on its own when the agent invokes it are not operations by the agent
and are outside the scope of this rule.

Reading or writing the agent's own memory is outside the scope of this rule.

An explicit user instruction MAY authorize an operation outside the working root
for the task at hand only. Before its first operation outside the working root,
an agent MUST ask the user for confirmation, even when the user has provided
that authorization.

## Navigation

Following local Markdown links is how agents MUST navigate this repository. It
keeps the agent's context limited to documents relevant to the task, instead of
unrelated content pulled in by other means.

### Local links

When an agent encounters a link to a local file relevant to the task, it MUST
open and read the file before continuing. It MUST resolve relative paths from
the directory of the document containing the link.

To follow a link to a whole file, an agent MUST use its native file-reading
tool. To follow a link to a section of a file, an agent MUST use its native file-reading tool when that tool can read the section.
Otherwise, it MAY use any other means to locate and read the section in the
linked file.

Links presented as a list form a listing, which is an index, not a reading list:
reading them is OPTIONAL. An agent MUST open a file linked
from a listing only when the task needs its content, and MUST NOT open it only
because it is listed. An entry MAY state when the task needs the file. As an
exception, an agent MAY read only the front-matter of a file linked from a
listing to determine whether the task needs its content.

A link inside an example, a code block, or a template illustrates a format and
is not navigation. An agent MUST NOT follow it unless the task needs the linked
file for a reason other than the example.

A link MAY resolve to any file inside the working root, including one above the
document containing the link. When a link resolves outside the working root, the
agent MUST NOT read the file, MUST notify the user, naming the link and the
document that contains it, and MUST ignore the link and continue the task
without that file.

Agents MUST NOT search file contents as a substitute for following those links, unless the user explicitly requests it.
Reading a file, and listing or filtering file names, are not content search.
When no link leads to content the task needs, an agent MAY locate it by file
name.

If the destination also contains local links needed for the task, the agent MUST
follow them and SHOULD NOT reread files already consulted.

If a file does not exist or cannot be read, the agent MUST report the limitation
and MUST NOT assume its contents.

### HTTP links

This section applies to every HTTP URL an agent fetches, whether it comes from a
Markdown link, from plain text, or from the agent itself.

An agent MUST NOT fetch an HTTP URL unless the task needs content of the URL
that the agent does not know in enough depth from its training.

Before fetching such a URL, the agent MAY look for content previously saved from
that exact URL in its own memory. Memory MAY be used, but it MUST NOT override
the convention documents, and when it differs from the page at the URL, the page
MUST take precedence.

When the URL is pinned to a revision or a commit and the saved content records
the same URL, the saved content is equivalent to the page, and the agent MAY use
it instead of fetching the URL. Otherwise, the saved content can be outdated,
and the agent SHOULD confirm the saved content against the page, or against
evidence of when the content was saved and when the page last changed, before
relying on it.

After fetching such a URL, the agent MAY save the content the task needed in its
own memory, under the exact URL. It SHOULD save it in the memory it keeps for
this project rather than in its general memory, and it MUST NOT save it inside
the working root.

When an agent fetches such a URL, it SHOULD request the raw-text form of the
page when the site provides one.

## Related

- [README.md](./README.md): what Kraf is and more.
- [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md): repo-wide
  conventions: convention documents and rule precedence, docs, workspace,
  naming.
