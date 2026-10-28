---
title: CODE CONVENTIONS
summary: Non-negotiable code rules for the Kraf codebase — rules you amend, not break.
tags: [code-standards, mandatory-rules, conventions, rust]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md#normative-language). This
> document MUST NOT use alternative words or lowercase variants as normative
> keywords.

---

> [!IMPORTANT]
> This is a convention document, as defined in
> [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md#convention-documents): its
> rules are mandatory and non-negotiable. Treat them like a constitution: a rule
> changes only through a pull request that edits this document and MUST NOT be
> bypassed by an ad-hoc exception in code.

## Rust modules

`mod.rs` MUST only declare/re-export submodules (`pub mod ...`); it MUST NOT
hold types, logic, or implementation. If a folder-module needs its own core
type/logic file, it MUST be named after the folder with a suffix (e.g.
`env_var/env_var_core.rs`, not `env_var/env_var.rs`) — a submodule can't share
its parent's exact name (`clippy::module_inception`), and the suffix keeps it
consistent with the folder's other files.

## Tests

- Tests SHOULD be small and direct — no over-engineering.
- Tests MAY repeat content instead of extracting a shared helper/const just to
  avoid repetition between tests.
- Tests SHOULD be self-contained: small setup SHOULD live inside the test so the
  reader can see what it uses without navigating elsewhere. A shared
  const/fixture MAY be used when inlining it would make the test unnecessarily
  large or obscure its behavior.

## Related

- [README.md](../README.md): what Kraf is.
