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

Agents MUST start here and follow these repository references.

## Docs

- [README.md](./README.md) — what Kraf is.
- [ARCHITECTURE.md](./ARCHITECTURE.md) — cross-domain architecture (system
  context, modules, ports and adapters, dependency rule).
- [BUILDING.md](./BUILDING.md) — repo conventions (organization, build, test,
  CI, naming, docs front-matter).
- [CODE_STANDARDS.md](./CODE_STANDARDS.md) — non-negotiable code invariants
  (amend, never break).
- [docs/README.md](./docs/README.md) — documentation & spec conventions (intent,
  specification, acceptance, ADR).

## Skills

- [.agents/skills/](./.agents/skills/) — each skill's `SKILL.md` front-matter
  says when to use it.
