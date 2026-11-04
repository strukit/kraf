---
title: AGENTS
summary: Entry point for agent instructions and the repository conventions they MUST follow.
tags: [agents, instructions, repository, coding-conventions]
applies_to: [all]
---

Before anything else, when a task targets a branch or a pull request, including
a review, you MUST read this document from that branch, the head of the pull
request, not from the default branch or the base branch. If the copy of this
document you loaded comes from another branch, you MUST discard it and read the
copy from the target branch.

After that, you MUST read
[GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md). Except for the rule
above, you MUST NOT continue with this document, or with any other action, until
you have read it. After it, you MUST read [README.md](./README.md).

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
documents the task needs.

Rules and recommendations are different things. A rule is a requirement that a
convention document defines, including a skill or agent defined in this
repository, and it binds. A recommendation comes from a skill or agent defined outside this
repository, as defined in Skills; it is not a rule, it does not bind, and it
MUST NOT override a rule.

Agents MUST follow the rules. They MAY act on a recommendation from a skill or
agent that the task uses. They MUST NOT raise, flag, or fix a concern that
neither a rule nor such a recommendation defines, such as choosing one approach,
name, or style over another.

Agents MUST report a correctness defect they find, such as code that produces a
wrong result, fails, or contradicts its spec, even when no convention document
defines it.

When an agent raises a concern, it MUST cite its source: the rule that defines
it, the skill or agent that recommends it, or, for a correctness defect, the
wrong result, failure, or contradiction it found.

Besides the exceptions this document defines, an explicit user instruction MAY
set aside a rule for the task at hand only, and does not change the rule. Agents
MUST NOT override a rule on their own preference or judgment.

## Normative language

When an agent writes or edits a document, it MUST verify that the content it
writes follows the normative-language rules that apply to that document: those
of [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#normative-language)
and, for a specification or an intent, those of
[SPEC_CONVENTIONS.md](./docs/SPEC_CONVENTIONS.md).

A review MUST verify that the content in its scope follows those rules, for
every change in its scope, and MUST report each deviation as a finding with
those rules as its source.

## Tools

A tool is anything the agent can invoke, other than one whose purpose is to
delegate work to an agent or a language model: its native tools, tools provided
to it, such as MCP tools, programs installed in the user's environment, and
tools declared in this repository.

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

When an agent writes content, it MAY ignore conventions that a declared tool
applies or checks, such as formatting, spelling, or lint rules, and it MUST run
those tools on the files it changed once it finishes writing.

A review MUST NOT run a declared tool and MUST NOT ask anyone to run one. When
an agent writes content in the same session as a review, such
as applying a fix, that writing is not part of the review, and the rules above
for writing content apply to it.

When a tool is not available, the agent MUST inform the user and continue the
task. When a tool it runs reports a problem in content the task changed, the
agent MUST fix it. When the tool can fix the problem, the agent MUST fix it with
the tool; otherwise, it MUST fix it by its own means.

An agent MUST NOT do work unless the user explicitly requests it, a convention
document or a skill or agent that the task uses requires, recommends, or permits
it as part of the work the task performs, or the task cannot be completed
without it.

## Reviews

> [!IMPORTANT]
> Golden rule: a review MUST NOT review anything outside its scope. This rule
> MUST take precedence over every other review rule, including the duty to
> report a correctness defect. Content that the change relies on but does not modify,
> such as a configuration file, a tool, or the environment the review runs in,
> is outside the scope, even when the change makes it more relevant.

The scope of a review is the change under review, the references to anything the
change renames, and, for a meaning-changing change, the content that depends on
it. A review MUST NOT validate or report content outside that scope, including a
defect that predates the change.

A review MUST determine, for each change in its scope, whether it is
meaning-preserving or meaning-changing, and MUST apply the rules below
accordingly. A meaning-preserving change keeps the behavior and meaning of what
it touches, such as letter case or punctuation in prose, a typo fix, a rename,
or a substitution by an equivalent construct. A change to a value or name that a
convention document requires other content to use is meaning-changing. A review
MAY omit this classification from its report.

For a meaning-preserving change, the review MUST verify only that the change
preserves meaning and that references to anything it renames still resolve, and
apply the normative-language check defined in Normative language. It MUST NOT
re-evaluate the touched content against anything else.

For a meaning-changing change, the review MUST evaluate the new meaning against
the content that depends on it.

For a meaning-changing change to a convention document, the content that depends
on it includes the content that the change makes noncompliant. Content that the
pull request description leaves for a later pull request is outside the scope.

A review MUST state the source of each finding: a correctness defect, a
convention document, a tool declared in this repository, or a skill or agent
defined outside this repository. A finding is blocking when its source is a
correctness defect, a convention document, or a failure of a tool declared in
this repository. Since a review MUST NOT run declared tools, such a failure
reaches it from outside the review, such as from CI or from a run that the user
requests. A correctness defect includes a rule that contradicts itself or
another rule. A finding from a skill or agent defined outside this repository is
a recommendation and is non-blocking. When that skill or agent declares the
finding blocking or required, the review MUST mark it as strongly recommended
and name the skill or agent. An explicit user instruction MAY change whether a
finding is blocking, for the task at hand only.

A review MUST NOT report or list missing content unless a convention document
requires that content, including by defining its absence as a defect, or a skill
or agent that the task uses expects it.

## Pull requests

When an agent writes or updates a pull request description, it MUST follow
[the pull request template](./.github/pull_request_template.md) and fill in each
of its sections. Every `<...>` placeholder in the template MUST be replaced with
content; its text is a recommendation, not a rule. A placeholder MUST NOT remain in the
description.

Each item MUST state briefly, in the agent's own words, what changed and, when
it is not evident, why. An item MUST NOT copy or restate in detail the text of
the content it changes, such as the wording of a rule; the diff carries that
text.

A pull request description MUST NOT ask a reviewer to run a declared tool, such
as in its `How to test?` section, because a review MUST NOT run one, as defined
in Tools.

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
it is the user's own. An explicit user instruction MUST be respected, as defined
above.

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
keeps the agent's context limited to documents the task needs, instead of
unrelated content pulled in by other means.

### Local links

When an agent encounters a link to a local file whose content the task needs, it
MUST open and read the file before continuing. It MUST resolve relative paths
from the directory of the document containing the link.

To follow a link to a whole file, an agent MUST use its native file-reading
tool. To follow a link to a section of a file, an agent MUST use its native
file-reading tool when that tool can read the section. Otherwise, it MAY use any
other means to locate and read the section in the linked file.

Links presented as a list form a listing, which is an index, not a reading list:
reading them is OPTIONAL. An agent MUST open a file linked from a listing only
when the task needs its content, and MUST NOT open it only because it is listed.
An entry MAY state when the task needs the file. As an exception, an agent MAY
read only the front-matter of a file linked from a listing to determine whether
the task needs its content.

A link inside an example, a code block, or a template illustrates a format and
is not navigation. An agent MUST NOT follow it unless the task needs the linked
file for a reason other than the example.

A link MAY resolve to any file inside the working root, including one above the
document containing the link. When a link resolves outside the working root, the
agent MUST NOT read the file, MUST notify the user, naming the link and the
document that contains it, and MUST ignore the link and continue the task
without that file.

Agents MUST NOT search file contents as a substitute for following those links,
unless the user explicitly requests it. Reading a file, and listing or filtering
file names, are not content search. When no link leads to content the task
needs, an agent MAY locate it by file name.

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
