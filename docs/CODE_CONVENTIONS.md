---
title: CODE CONVENTIONS
summary: Non-negotiable code invariants for the Kraf codebase — rules you amend, not break.
tags: [code-standards, invariants, conventions, rust]
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
> These are the Kraf codebase's **invariants** — mandatory rules that the
> codebase MUST follow. You MUST NOT introduce ad-hoc exceptions. Changing an
> invariant requires a deliberate decision in a pull request that edits this
> file.

## Rust modules

`mod.rs` MUST only declare/re-export submodules (`pub mod ...`); it MUST NOT
hold types, logic, or implementation. If a folder-module needs its own core
type/logic file, it MUST be named after the folder with a suffix (e.g.
`env_var/env_var_core.rs`, not `env_var/env_var.rs`) — a submodule can't share
its parent's exact name (`clippy::module_inception`), and the suffix keeps it
consistent with the folder's other files.

## Tests

- Tests SHOULD be small and direct — no over-engineering.
- Duplication is fine — there's no need to extract a shared helper/const just to
  avoid repeating content between tests.
- Tests MUST be self-contained: a test's setup lives inside the test itself, not
  in a shared const/fixture defined elsewhere in the file, so the reader doesn't
  have to scroll up to find out what a test uses.

## Related docs

- [README.md](../README.md): what Kraf is.
- [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md): repo conventions.
