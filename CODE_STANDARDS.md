---
title: CODE STANDARDS
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

## Overview

These are the Kraf codebase's **invariants** — non-negotiable rules that code
MUST follow. Treat them like a constitution: you MUST NOT break an invariant, you
**amend** it. Changing one is a deliberate decision made in a pull request that
edits this file — it MUST NOT be an ad-hoc exception slipped into passing code.

## Rust modules

`mod.rs` MUST only declare/re-export submodules (`pub mod ...`); it MUST NOT hold
types, logic, or implementation. If a folder-module needs its own core
type/logic file, it MUST be named after the folder with a suffix (e.g.
`env_var/env_var_core.rs`, not `env_var/env_var.rs`) — a submodule can't share
its parent's exact name (`clippy::module_inception`), and the suffix keeps it
consistent with the folder's other files.

## Related docs

- [BUILDING.md](./BUILDING.md): repo conventions.
- [README.md](./README.md): what Kraf is.
