---
title: AGENTS
summary: Entry point for agent instructions and the repository conventions they MUST follow.
tags: [agents, instructions, repository, coding-conventions]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
>
> This document MUST use these terms, in uppercase, when expressing normative
> requirements, permissions, recommendations, or prohibitions. It MUST NOT use
> alternative words or lowercase variants as normative keywords.

---

> [!IMPORTANT]
> These are the Kraf agents' **invariants** — mandatory rules that agents MUST
> follow. You MUST NOT introduce ad-hoc exceptions. Changing an invariant
> requires a deliberate decision in a pull request that edits this file.
>
> The only exception is an explicit user instruction, which applies to the task
> at hand only and does not change the rule. Agents MUST NOT override a rule on
> their own preference or judgment.
>
> Memory and skills MAY inform a task but MUST NOT override a rule. When they
> conflict with a rule, the agent MUST follow the rule and MUST notify the user
> of the conflict, stating that, if the content is pertinent, it needs to be
> committed to a convention document to take effect.

This document is the entry point for every agent task in this repository. Before
starting any task, agents MUST start here and follow the documentation links to
documents relevant to the task.

Agents MUST act only on rules defined in convention documents and on skills that
apply them. They MUST NOT raise, flag, or fix concerns that no convention
document defines, and MUST NOT take on work that deterministic tooling performs.

## Markdown Links

Following local Markdown links is how agents MUST navigate this repository. It
keeps the agent's context limited to documents relevant to the task, instead of
unrelated content pulled in by other means.

When an agent encounters a link to a local file relevant to the task, it MUST
open and read the file before continuing. It MUST resolve relative paths from
the directory of the document containing the link.

Agents MUST NOT use content search (such as grep or ripgrep) as a substitute for
following those links, unless the user explicitly requests it.

Example: in `docs/README.md`, the link [agent instructions](../AGENTS.md) points
to `AGENTS.md` at the project root.

If the destination also contains local links needed for the task, the agent MUST
follow them and SHOULD NOT reread files already consulted.

If a file does not exist or cannot be read, the agent MUST report the limitation
and MUST NOT assume its contents.

## Docs

- [README.md](./README.md) — what Kraf is.
- [GENERAL_CONVENTIONS.md](./docs/GENERAL_CONVENTIONS.md) — repo-wide
  conventions: convention documents and rule precedence, docs, workspace,
  naming.
- [CODE_CONVENTIONS.md](./docs/CODE_CONVENTIONS.md) — code conventions: modules
  and tests.
- [SPEC_CONVENTIONS.md](./docs/SPEC_CONVENTIONS.md) — spec conventions: layout,
  identifiers, intent, specification, acceptance, ADR.
- [ARCHITECTURE.md](./docs/ARCHITECTURE.md) — cross-domain architecture: system
  context, containers, modules, ports and adapters, dependency rule, decisions.

## Skills

- [.agents/skills/](./.agents/skills/) — each skill's `SKILL.md` front-matter
  says when to use it.
