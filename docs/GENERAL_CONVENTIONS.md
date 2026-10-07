---
title: GENERAL CONVENTIONS
summary: First conventions for organizing, code, docs the Kraf monorepo.
tags: [monorepo, conventions, documentation, agents]
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
>
> A convention document is a document whose purpose is to define rules and that
> carries the invariants note near the top.
>
> Rules MUST be defined only in dedicated convention documents. Code comments,
> commit messages, pull request descriptions, and any other content MUST NOT
> introduce new rules; they MAY only reference or apply existing ones.
>
> Rules explicitly scoped to a particular document type or context MUST take
> precedence over these general rules wherever they conflict, and only within
> that scope. All non-conflicting general rules MUST remain in effect. Such
> overrides do not require changing this file, but MUST themselves be defined in
> a convention document.

## Overview

Kraf is a monorepo. It started in Rust, but the organization is **not by
language** — it's by **feature/domain**. Each domain is its own folder
(folder-per-feature); if a domain needs another language later, it lives right
next to the rest instead of becoming a separate per-language workspace. Language
is an implementation detail of each feature, not the repo's organizing axis.

## Docs Conventions

> The root [`README.md`](../README.md) does not require front-matter or the
> normative-language note because it introduces the repository rather than
> defining conventions.

### Normative language

Every document MUST carry, near the top, the note below (the Template) declaring
that its normative keywords follow RFC 2119 and RFC 8174. A document carrying
the note MUST use the listed terms, in uppercase, whenever it expresses a
normative requirement, permission, recommendation, or prohibition. It MUST NOT
express normative meaning through alternative keywords or lowercase variants of
the listed terms.

The note does not require a document to contain normative requirements or to use
every listed term. A purely descriptive document MAY contain the note without
adding an artificial requirement.

**Template:**

```md
> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
>
> This document MUST use these terms, in uppercase, when expressing normative
> requirements, permissions, recommendations, or prohibitions. It MUST NOT use
> alternative words or lowercase variants as normative keywords.
```

A family of documents MAY define its own equivalent note that lists only the
terms it uses — for example, specifications use the EARS block defined in
[Specifications](./SPEC_CONVENTIONS.md#specification-specmd) — as long as it
cites RFC 2119 and RFC 8174 near the top.

### Markdown front-matter

Every doc MUST have front-matter with `title`, `summary`, and `tags` — it makes
docs easy to index for agents.

This applies to docs, not to every `.md`. A file that is not a doc but uses
`.md` and carries its own front-matter schema — for example a skill's `SKILL.md`
(`name`/`description`).

Repo instruction files, including [`AGENTS.md`](../AGENTS.md) and `SKILL.md`
files, MUST follow the normative-language requirements below, even when their
own front-matter schema is exempt from this section's front-matter requirement.

### Docs graph

Repo-level docs form a connected graph rooted at [README.md](../README.md):
every doc MUST link to every parent doc it references, so no doc is reachable
only by its file path. A link already in the body satisfies this rule; a
separate Related docs section is OPTIONAL and MAY be used to consolidate
outbound links in one place. A set of docs MAY define its own graph; when it
does, that graph MUST be documented alongside the set. Always read the graph
before navigating or linking across a set of docs.

Every reference to a specific file MUST be a navigable link, not a bare name —
so the reader can follow it. (Generic references, like "a module's `README.md`",
are not links.)

### Language-neutral docs

Cross-domain docs (e.g. [`ARCHITECTURE.md`](./ARCHITECTURE.md), specs) describe
the shape, not the implementation language: they MUST NOT use language-specific
terms (e.g. "trait") or language-specific file names and extensions (e.g.
`filesystem.rs`, `lib.rs`), and MUST refer to roles instead — "interface" for a
trait, and the `lib` / `cli` / `mcp` entrypoints without an extension.

## Workspace Conventions

### Std-first

Code SHOULD prefer standard library over external libs, and SHOULD keep
dependencies few and consolidated. Exceptions: official platform bindings,
foundational runtimes/packages, big-tech, Linux Foundation and friends packages.
Code SHOULD NOT add convenience wrappers on top of another lib, or a "wrapper of
a wrapper".

Example: terminal handling uses `std::io::IsTerminal` + minimal syscalls, not
lib/package like crossterm/portable-pty.

### Lib-first

Real logic MUST live in the lib; the binary only consumes it. This keeps logic
testable and reusable outside the CLI (another binary, another consumer, etc.).

### Multi-platform

Every domain MUST be designed for multiple platforms (Mac/Linux/Windows) from
the start, not bolted on later. Platform-specific code MUST be isolated (e.g.
`_macos.rs`, `_linux.rs`, `_windows.rs`), and shared logic MUST stay separate
from platform-specific logic.

### Tools declared in files, with version

Build/tool requirements MUST be declared in dedicated, versioned files, not
hardcoded in scripts or docs. See [`.msvc.json`](../.msvc.json): it declares
what's needed to build on Windows (VC++ Tools components, Windows SDK version).

### Ignored files

[`.gitignore`](../.gitignore) denies everything by default and only allowlists
what the repo needs (see the file's own header comment).

### CI

Everything that runs in CI (test, build, release) MUST be reproducible locally.
We use Docker Bake for that — build in it for every platform, except Mac, which
Docker can't build for.

### Agents

Config exclusive to a single agent tool (Claude-only, Codex-only, etc.) MUST NOT
be committed — it lives in the dev's own environment. What is portable across
agents MAY be committed and, when it encodes a repo rule, SHOULD be: skills, and
agent personas bound to this repo's standards — for example, a Rust reviewer
that enforces [CODE_CONVENTIONS.md](./CODE_CONVENTIONS.md). A persona MUST be
committed in a vendor-neutral form; only the tool-specific glue stays personal.

### Naming

#### Platform names

Platform names MUST be written as ```Linux```, ```MacOS```, and ```Windows```—
in prose, docs, code, and diagrams. In particular, use```MacOS```; Apple's
``macOS` styling MUST NOT be used.

## Related

- [README.md](../README.md): what Kraf is.
