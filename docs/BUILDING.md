---
title: BUILDING
summary: Conventions for organizing, building, testing, and documenting the Kraf monorepo.
tags: [monorepo, conventions, testing, continuous-integration, documentation, agents]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
>
> This document MUST use these terms, in uppercase, when expressing normative
> requirements, permissions, recommendations, or prohibitions. It MUST NOT use
> alternative words or lowercase variants as normative keywords.

## Summary (TL;DR)

This section is a quick orientation for contributors and agent skills, not a
separate source of truth. The linked documents below define the rules. This
summary MUST be reviewed and updated when those rules change; matching skills
and automations SHOULD be updated as part of the same change.

- Organize the repo by feature/domain, not by language: see
  [Module map](./ARCHITECTURE.md#module-map).
- Dependencies MUST point inward and MUST NOT form a cycle: see
  [Dependency rule](./ARCHITECTURE.md#dependency-rule).
- Real logic MUST live in the lib, with the binary consuming it: see
  [Lib-first](#lib-first).
- Design for Linux, MacOS, and Windows from the start, isolating
  platform-specific code: see [Multi-platform](#multi-platform).
- Declare build/tool requirements in dedicated, versioned files: see
  [Tools declared in files, with version](#tools-declared-in-files-with-version).
- Keep feature documentation with its feature and make its docs graph navigable:
  see [Docs layout](./SPEC_CONVENTIONS.md#docs-layout) and
  [Docs graph](./SPEC_CONVENTIONS.md#docs-graph).
- Keep portable repo guidance in shared skills/personas, not tool-specific
  configuration: see [Agents](#agents).

This file and [CODE_STANDARDS.md](./CODE_STANDARDS.md) are the repo's binding
standards — its constitution. You amend them through a pull request; you MUST
NOT break them. A rule stated as a preference is still binding, and its
exceptions are part of the rule.

## Overview

Kraf is a monorepo. It started in Rust, but the organization is **not by
language** — it's by **feature/domain**. Each domain is its own folder
(folder-per-feature); if a domain needs another language later, it lives right
next to the rest instead of becoming a separate per-language workspace. Language
is an implementation detail of each feature, not the repo's organizing axis.

## Docs standards

> The root [`README.md`](../README.md) does not require front-matter or the
> normative-language note because it introduces the repository rather than
> defining conventions.

### Markdown front-matter

Every doc MUST have front-matter with `title`, `summary`, and `tags` — it makes
docs easy to index for agents. Module `README.md` files MUST have it — see
[Module README](./SPEC_CONVENTIONS.md#readme-readmemd).

This applies to docs, not to every `.md`. A file that is not a doc but uses
`.md` and carries its own front-matter schema — for example a skill's `SKILL.md`
(`name`/`description`) — is out of scope.

Repo instruction files, including [`AGENTS.md`](../AGENTS.md) and skill `SKILL.md` files, MUST
follow the normative-language requirements below, even when their own
front-matter schema is exempt from this section's front-matter requirement.

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
[Specifications](./SPEC_CONVENTIONS.md#specification-specmd) — as long as it cites
RFC 2119 and RFC 8174 near the top.

### Docs graph

Repo-level docs form a connected graph rooted at [README.md](../README.md): every
doc MUST link to the docs adjacent to it, so none is reachable only by its file
path. Feature docs (intent, specification, acceptance, ADR) follow their own
graph — see [Specifications](./SPEC_CONVENTIONS.md#docs-graph).

Every reference to a specific file MUST be a navigable link, not a bare name —
so the reader can follow it. (Generic references, like "a module's `README.md`",
are not links.)

### Language-neutral docs

Cross-domain docs (e.g. [`ARCHITECTURE.md`](./ARCHITECTURE.md), specs) describe the shape, not the
implementation language: they MUST NOT use language-specific terms (e.g.
"trait") or language-specific file names and extensions (e.g. `filesystem.rs`,
`lib.rs`), and MUST refer to roles instead — "interface" for a trait, and the
`lib` / `cli` / `mcp` entrypoints without an extension.

## Workspace standards

### Std-first

Code SHOULD prefer Rust's standard library over external crates, and SHOULD keep
dependencies few and consolidated. Exceptions: official platform bindings,
foundational runtimes/ packages, big-tech / Linux Foundation and friends
packages. Code SHOULD NOT add convenience wrappers on top of another lib, or a
"wrapper of a wrapper".

Example: terminal handling uses `std::io::IsTerminal` + minimal syscalls, not
crates like crossterm/portable-pty.

### Lib-first

Real logic MUST live in the lib; the binary only consumes it. This keeps logic
testable and reusable outside the CLI (another binary, another consumer, etc.).

### Multi-platform

Every domain MUST be designed for multiple platforms (MacOS/Linux/Windows) from
the start, not bolted on later. Platform-specific code MUST be isolated (e.g.
`_macos.rs`, `_linux.rs`, `_windows.rs`), and shared logic MUST stay separate
from platform-specific logic.

### Tools declared in files, with version

Build/tool requirements MUST be declared in dedicated, versioned files, not
hardcoded in scripts or docs. See [`.msvc.json`](../.msvc.json): it declares what's needed to
build on Windows (VC++ Tools components, Windows SDK version).

### Tests

- Tests SHOULD be small and direct — no over-engineering.
- Duplication is fine — there's no need to extract a shared helper/const just to
  avoid repeating content between tests.
- Tests MUST be self-contained: a test's setup lives inside the test itself, not
  in a shared const/fixture defined elsewhere in the file, so the reader doesn't
  have to scroll up to find out what a test uses.

### Ignored files

[`.gitignore`](../.gitignore) denies everything by default and only allowlists what the repo
needs (see the file's own header comment). [`.dockerignore`](../.dockerignore) is a symlink to it,
so the same allowlist keeps the Docker build context clean — no dirty/leftover
files sneak into the builder.

### CI

Everything that runs in CI (test, build, release) MUST be reproducible locally.
We use Docker Bake for that — build in it for every platform, except MacOS,
which Docker can't build for.

### Agents

Config exclusive to a single agent tool (Claude-only, Codex-only, etc.) MUST NOT
be committed — it lives in the dev's own environment. What is portable across
agents MAY be committed and, when it encodes a repo rule, SHOULD be: skills, and
agent personas bound to this repo's standards — for example, a Rust reviewer
that enforces [CODE_STANDARDS.md](./CODE_STANDARDS.md). A persona MUST be
committed in a vendor-neutral form; only the tool-specific glue stays personal.

### Naming

#### Platform names

Platform names MUST be written as `Linux`, `MacOS`, and `Windows` — in prose,
docs, code, and diagrams. In particular, use `MacOS`; Apple's `macOS` styling
MUST NOT be used.

## Code standards

See [CODE_STANDARDS.md](./CODE_STANDARDS.md) for the code standards.

## Related docs

- [README.md](../README.md): what Kraf is.
