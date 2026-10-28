---
title: AGENTS
summary: Entry point for agent instructions and the repository conventions they MUST follow.
tags: [agents, instructions, repository, coding-conventions]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#normative-language). This
> document MUST NOT use alternative words or lowercase variants as normative
> keywords.

---

> [!IMPORTANT]
> This is a convention document, as defined in
> [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#convention-documents): its
> rules are mandatory and non-negotiable. Treat them like a constitution: a rule
> changes only through a pull request that edits this document and MUST NOT be
> bypassed by an ad-hoc exception in code.

This document is the entry point for every agent task in this repository. Before
starting any task, agents MUST start here and follow the documentation links to
documents relevant to the task.

Agents MUST act only on rules defined in convention documents and on skills that
apply them. They MUST NOT raise, flag, or fix concerns that no convention
document defines, and MUST NOT take on work that deterministic tooling performs.

The only exception is an explicit user instruction, which applies to the task at
hand only and does not change the rule. Agents MUST NOT override a rule on their
own preference or judgment.

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

A skill file (`SKILL.md`) MUST carry the normative-language note defined in
[GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md#normative-language) and
follow its requirements.

## Workspace boundary

Agents MUST NOT leave the working root for any operation, including reading,
writing, executing commands, or navigating files.

An explicit user instruction MAY authorize an operation outside the working
root for the task at hand only. Before its first operation outside the working
root, an agent MUST ask the user for confirmation, even when the user has
provided that authorization.

## Navigation

Following local Markdown links is how agents MUST navigate this repository. It
keeps the agent's context limited to documents relevant to the task, instead of
unrelated content pulled in by other means.

When an agent encounters a link to a local file relevant to the task, it MUST
open and read the file before continuing. It MUST resolve relative paths from
the directory of the document containing the link.

A link MAY resolve to any file inside the working root, including one above the
document containing the link. When a link resolves outside the working root,
the agent MUST NOT read the file, MUST notify the user, naming the link and the
document that contains it, and MUST ignore the link and continue the task
without that file.

Agents MUST NOT use content search (such as grep or ripgrep) as a substitute for
following those links, unless the user explicitly requests it.

If the destination also contains local links needed for the task, the agent MUST
follow them and SHOULD NOT reread files already consulted.

If a file does not exist or cannot be read, the agent MUST report the limitation
and MUST NOT assume its contents.

## Related

- [README.md](./README.md) — what Kraf is.
- [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md) — repo-wide
  conventions: convention documents and rule precedence, docs, workspace,
  naming.
- [ARCHITECTURE.md](./docs/ARCHITECTURE.md) — cross-domain architecture: system
  context, containers, modules, ports and adapters, dependency rule, decisions.
- [CODE_CONVENTIONS.md](./docs/CODE_CONVENTIONS.md) — code conventions: modules
  and tests.
- [SPEC_CONVENTIONS.md](./docs/SPEC_CONVENTIONS.md) — spec conventions: layout,
  identifiers, intent, specification, acceptance, ADR.
