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
another.

Agents MUST report a correctness defect they find, such as code that produces a
wrong result, fails, or contradicts its spec, even when no convention document
defines it.

The only exception is an explicit user instruction, which applies to the task at
hand only and does not change the rule. Agents MUST NOT override a rule on their
own preference or judgment.

## Deterministic tools

Agents MUST NOT perform or verify by their own means work that a deterministic
tool declared in this repository already performs, such as formatting, line
width, or linting, for example by counting characters. When the work needs to be
verified, the agent MUST use the declared tool and SHOULD run it only at the end
of the task. When no such tool is declared for the work, the work remains with
the agent only if a convention document requires it. Otherwise, the agent MUST
NOT do it, such as wrapping lines at a width that no declared tool or convention
document defines.

## Reviews

A review MUST NOT validate or report content outside the scope of the change
under review, including a defect that predates the change. Within reviews, this
takes precedence over the duty to report a correctness defect.

A review MUST NOT report or list missing content unless a convention document
requires that content.

Within that scope, code behavior that no specification defines is a correctness
defect, even when an intent describes it. A review MUST report it, even though
the missing requirement is missing content.

A review MUST determine, for each change in its scope, whether it is
meaning-preserving or meaning-changing, and MUST apply the rules below
accordingly. A meaning-preserving change keeps the behavior and meaning of what
it touches, such as letter case, punctuation, a typo fix, a rename, or a
substitution by an equivalent construct. A review MAY omit this classification
from its report.

For a meaning-preserving change, the review MUST verify only that the change
preserves meaning. It MUST NOT re-evaluate the touched content against code,
specifications, or acceptance.

For a meaning-changing change, the review MUST evaluate the new meaning against
what depends on it, such as linked acceptance, implementing code, and callers.

## Memory

Memory MAY inform a task but MUST NOT override a rule. When it conflicts with a
rule, the agent MUST follow the rule and MUST notify the user of the conflict,
stating that, if the content is pertinent, it needs to be committed to a
convention document to take effect.

## Skills

Skills MAY inform a task but MUST NOT override a rule. When a skill conflicts
with a rule, the agent MUST follow the rule and MUST notify the user of the
conflict, stating that, if the content is pertinent, it needs to be committed to
a convention document to take effect.

A skill or agent MAY state preferences, such as formatting, layout, or naming
choices, that agents follow when they create or edit content under it. A
preference is not a rule: it MUST NOT override a convention document, and agents
MUST NOT report a deviation from it as a defect, including in a code review.

A skill file (`SKILL.md`) MUST carry the normative-language note defined in
[GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#normative-language) and
follow its requirements.

## User instructions

An explicit user instruction is one the user gives in the task itself. An
instruction the agent loads from a standing source, such as a personal
instruction file or memory, is not explicit, even when it is the user's own. An
explicit user instruction MUST be respected, as defined above.

A standing instruction can conflict with a rule, for example an instruction not
to read external links when a rule allows following them. When an agent finds
such a conflict, it MUST notify the user, briefly stating that the conflict
exists and which instruction or rule is involved, and it MUST continue the task
following the rule. When the conflict is with memory, the notice also states
what the Memory section requires. The agent MUST NOT suppress the conflict
silently.

When following the rule would cause a severe effect, such as one that is hard to
reverse or that reaches outside the working root, the agent MUST stop and ask
the user before continuing.

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

Links that form a listing, likes sections or a list of anything, are an index,
not a reading list: reading them is OPTIONAL. An agent MUST open a file linked
from a listing only when the task needs its content, and MUST NOT open it only
because it is listed. An entry MAY state when the task needs the file.

A link inside an example, a code block, or a template illustrates a format and
is not navigation. An agent MUST NOT follow it unless the task needs the linked
file for a reason other than the example.

A link MAY resolve to any file inside the working root, including one above the
document containing the link. When a link resolves outside the working root, the
agent MUST NOT read the file, MUST notify the user, naming the link and the
document that contains it, and MUST ignore the link and continue the task
without that file.

Agents MUST NOT use content search (such as grep, ripgrep or cat) as a
substitute for following those links, unless the user explicitly requests it.

If the destination also contains local links needed for the task, the agent MUST
follow them and SHOULD NOT reread files already consulted.

If a file does not exist or cannot be read, the agent MUST report the limitation
and MUST NOT assume its contents.

### HTTP links

This section applies only to Markdown links to an HTTP URL. Any other HTTP URL,
such as one written as plain text, is outside it.

An agent MUST NOT follow a Markdown link to an HTTP URL unless the task needs
content of the link that the agent does not know in enough depth from its
training.

Before following such a link, the agent MAY look for content previously saved
from its exact URL, in its own memory or wherever it can best find it. Memory
MAY be used, but it MUST NOT override the convention documents, and the linked
page remains the source of truth.

When the link is pinned to a revision or a commit and the saved content records
the same URL, the saved content is equivalent to the linked page, and the agent
MAY use it instead of following the link. Otherwise, the saved content MAY be
outdated, and the agent SHOULD confirm the saved content against the linked
page, or against empirical evidence such as dates, before relying on it.

After following such a link, the agent MAY save the content the task needed,
wherever it can best keep it, under the exact URL of the link.

When an agent follows such a link, it SHOULD request the raw-text form of the
page when the site provides one. For example:

- `https://www.rfc-editor.org/rfc/<rfc_id>` becomes
  `https://www.rfc-editor.org/rfc/<rfc_id>.txt`.
- `https://en.wikipedia.org/w/index.php?title=<title>&oldid=<oldid>` becomes
  `https://en.wikipedia.org/w/index.php?title=<title>&oldid=<oldid>&action=raw`.
- `https://github.com/<owner>/<repo>/blob/<ref>/<path>` becomes
  `https://raw.githubusercontent.com/<owner>/<repo>/<ref>/<path>`.

## Related

- [README.md](./README.md): what Kraf is and more.
- [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md): repo-wide
  conventions: convention documents and rule precedence, docs, workspace,
  naming.
