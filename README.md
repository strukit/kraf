---
title: Kraf
description: What Kraf is, its vision, and the index of the repository's documents.
tags: [kraf, overview, vision, index]
type: ["guide"]
include: ["**/*"]
---

# Kraf

Kraf is a neutral layer for humans and autonomous agents that provides a
consistent way to work with software projects across tools, environments, and
vendors, without vendor lock-in. Rather than replacing the tools, configuration
files, or workflows a project already uses, Kraf reads them to work out what the
project needs, then sets up only what a workload requires, wherever it runs.

It provides development environments fast and with the least possible friction,
and it gives agents tools that run on only what each task needs. Reading a file
needs no operating system or container and can run in a browser; debugging a
test needs only the pieces that test touches, not a full system. A full
development environment and a single agent tool come from the same project
files.

## Vision

> **Materialize only what the workload proves it needs.**

Today Kraf materializes environments as a local shell; CI, containers, browsers,
and airgapped agents come later. The targets change; what Kraf reads from the
project stays the same.

Kraf's larger aim is to expose a project as a headless IDE runtime for agents:
not an editor, and not a closed platform, but a layer beneath or beside them. It
will surface a project's files, diagnostics, and language tooling (LSP, task
runners, debugger, tests) as tools that replace or complement an agent's own
built-in primitives. Each tool materializes only what its task needs: it runs
inside an isolated environment when the task calls for one, or with only the
pieces the task touches, even in a browser.

Agents often work on several branches at once. Kraf will give each branch or
worktree its own environment, derived from the environment of the branch it came
from rather than built from scratch, so parallel work starts fast and branches
run side by side without competing for resources such as ports.

These tools are reachable through whatever host fits the workload, such as an
agent CLI, a browser, or code that embeds them as a library (C or WASM), all
calling the same runtime. Kraf aims to interoperate with the editors and
toolchains a project already uses rather than replacing them.

## Modules

- [Environment variables](./src/environment/src/env_var/README.md): declarative
  model, resolution, and projection of environment variables.

## Related

- [AGENTS.md](./AGENTS.md): Entry point for agent instructions and the
  repository conventions they MUST follow.
- [GENERAL_STANDARDS.md](./docs/GENERAL_STANDARDS.md): General standards for
  repository, document layout and metadata, normative language, the document
  graph, links, and language neutrality.
- [DOC_STANDARDS.md](./docs/DOC_STANDARDS.md): Conventions for document layout,
  metadata, precedence, scope, and normative language.
- [SPEC_CONVENTIONS.md](./docs/SPEC_CONVENTIONS.md): Conventions for documenting
  feature intent, requirements, and acceptance scenarios.
- [ARCHITECTURE.md](./docs/ARCHITECTURE.md): Cross-domain map of Kraf's system
  context, modules, and how they depend on each other.
- [CODE_CONVENTIONS.md](./docs/CODE_CONVENTIONS.md): Non-negotiable code rules
  for the Kraf codebase — rules you amend, not break.
- [TOOLS.md](./docs/TOOLS.md): Tools declared in this repository, with their
  purpose, configuration, capabilities, and version.
- [Environment variables](./src/environment/src/env_var/README.md): Declarative
  model, resolution, and projection of environment variables in Kraf.
- [example-intent.md](./docs/example/example-intent.md): Illustrative intent
  statement used to demonstrate the specification format.
- [example.spec.md](./docs/example/example.spec.md): Illustrative specification
  used to demonstrate the requirement format.
- [example.acceptance.md](./docs/example/example.acceptance.md): Illustrative
  acceptance scenario used to demonstrate the acceptance format.
