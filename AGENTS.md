---
title: AGENTS
summary: Entry point for agent instructions and the repository conventions they MUST follow.
tags: [agents, instructions, repository, coding-conventions]
---

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
another, and MUST NOT take on work that deterministic tooling performs.

Agents MUST report a correctness defect they find, such as code that produces a
wrong result, fails, or contradicts its spec, even when no convention document
defines it.

The only exception is an explicit user instruction, which applies to the task at
hand only and does not change the rule. Agents MUST NOT override a rule on their
own preference or judgment.

## Reviews

A review MUST NOT report or list missing content unless a convention document
requires that content.

When the user explicitly requests a deep review, the review MUST cover
everything in its scope, including missing content.

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

## Workspace boundary

Agents MUST NOT leave the working root for any operation they perform directly,
including reading a file, writing a file, navigating files, or executing a
command whose working directory or path arguments are outside the working root.

The rule applies to the agent's direct operations only. Files that a tool reads
or writes on its own when the agent invokes it are not operations by the agent
and are outside the scope of this rule.

An explicit user instruction MAY authorize an operation outside the working root
for the task at hand only. Before its first operation outside the working root,
an agent MUST ask the user for confirmation, even when the user has provided
that authorization.

## Navigation

Following local Markdown links is how agents MUST navigate this repository. It
keeps the agent's context limited to documents relevant to the task, instead of
unrelated content pulled in by other means.

When an agent encounters a link to a local file relevant to the task, it MUST
open and read the file before continuing. It MUST resolve relative paths from
the directory of the document containing the link.

A link MAY resolve to any file inside the working root, including one above the
document containing the link. When a link resolves outside the working root, the
agent MUST NOT read the file, MUST notify the user, naming the link and the
document that contains it, and MUST ignore the link and continue the task
without that file.

Agents MUST NOT use content search (such as grep or ripgrep) as a substitute for
following those links, unless the user explicitly requests it.

If the destination also contains local links needed for the task, the agent MUST
follow them and SHOULD NOT reread files already consulted.

If a file does not exist or cannot be read, the agent MUST report the limitation
and MUST NOT assume its contents.

### HTTP links

An agent SHOULD follow an HTTP link only when the task needs its content, and
MAY skip it otherwise.

When an agent follows an HTTP link, it SHOULD request the raw-text form of the
page when the site provides one, for example:

- `https://www.rfc-editor.org/rfc/<rfc_id>` becomes
  `https://www.rfc-editor.org/rfc/<rfc_id>.txt`.
- `https://en.wikipedia.org/w/index.php?title=<title>&oldid=<oldid>` becomes
  `https://en.wikipedia.org/w/index.php?title=<title>&oldid=<oldid>&action=raw`.
- `https://github.com/<owner>/<repo>/blob/<ref>/<path>` becomes
  `https://raw.githubusercontent.com/<owner>/<repo>/<ref>/<path>`.

## Related

- [README.md](./README.md): what Kraf is.
- [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md): repo-wide
  conventions: convention documents and rule precedence, docs, workspace,
  naming.
- [ARCHITECTURE.md](./docs/ARCHITECTURE.md): cross-domain architecture: system
  context, containers, modules, ports and adapters, dependency rule, decisions.
- [CODE_CONVENTIONS.md](./docs/CODE_CONVENTIONS.md): code conventions: modules
  and tests.
- [SPEC_CONVENTIONS.md](./docs/SPEC_CONVENTIONS.md): spec conventions: layout,
  identifiers, intent, specification, acceptance, ADR.
