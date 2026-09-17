---
title: BUILDING
summary: Conventions for organizing, building, testing, and documenting the Kraf monorepo.
tags: [monorepo, conventions, testing, continuous-integration, documentation, agents]
---

## Overview

Kraf is a monorepo. It started in Rust, but the organization is **not by
language** — it's by **feature/domain**. Each domain is its own folder
(folder-per-feature); if a domain needs another language later, it lives right
next to the rest instead of becoming a separate per-language workspace. Language
is an implementation detail of each feature, not the repo's organizing axis.

## Workspace standards

### Std-first

Prefer Rust's standard library over external crates. Keep dependencies few, and
only consolidated ones. Exceptions: official platform bindings, foundational
runtimes/ packages, big-tech / Linux Foundation and friends packages. Avoid:
convenience wrappers on top of another lib, or "wrapper of a wrapper".

Example: terminal handling uses `std::io::IsTerminal` + minimal syscalls, not
crates like crossterm/portable-pty.

### Lib-first

Real logic lives in the lib; the binary only consumes it. Keeps logic testable
and reusable outside the CLI (another binary, another consumer, etc.).

### Multi-platform

Every domain is designed for multiple platforms (MacOS/Linux/Windows) from the
start, not bolted on later. Platform-specific code is isolated (e.g.
`_macos.rs`, `_linux.rs`, `_windows.rs`); shared logic stays separate from
platform-specific logic.

### Tools declared in files, with version

Build/tool requirements are declared in dedicated, versioned files — not
hardcoded in scripts or docs. See `.msvc.json`: declares what's needed to build
on Windows (VC++ Tools components, Windows SDK version).

### Tests

- Small and direct — no over-engineering.
- Duplication is fine. No need to extract a shared helper/const just to avoid
  repeating content between tests.
- Self-contained. A test's setup lives inside the test itself, not in a shared
  const/fixture defined elsewhere in the file — don't make the reader scroll up
  to find out what a test uses.

## Ignored files

`.gitignore` denies everything by default and only allowlists what the repo
needs (see the file's own header comment). `.dockerignore` is a symlink to it,
so the same allowlist keeps the Docker build context clean — no dirty/leftover
files sneak into the builder.

## CI

Everything that runs in CI (test, build, release) must be reproducible locally.
We use Docker Bake for that — build in it for every platform, except macOS,
which Docker can't build for.

## Markdown front-matter

Every `.md` file requires frontmatter with `title`, `summary`, and `tags` —
makes docs easy to index for agents. Exception: `README.md`, which doesn't need
frontmatter.

## Agents

Don't commit AI agent config that's exclusive to a single agent (Claude-only,
Codex-only, etc.). Config meant to work across agents still doesn't belong in
the repo — it lives in the dev's own environment.

## Code standards

### Rust modules

`mod.rs` only declares/re-exports submodules (`pub mod ...`) — it never holds
types, logic, or implementation. If a folder-module needs its own core
type/logic file, name it after the folder with a suffix (e.g.
`env_var/env_var_core.rs`, not `env_var/env_var.rs`) — a submodule can't share
its parent's exact name (`clippy::module_inception`), and the suffix keeps it
consistent with the folder's other files.
