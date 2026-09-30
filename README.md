# Kraf

Kraf is a neutral layer for humans and autonomous agents, designed to provide a
consistent way to work with software projects across tools, environments, and
vendors — without vendor lock-in. The goal is to start a development environment
with the least possible friction, and fast.

## Summary

Kraf explores a portable, vendor-neutral foundation for developer and agent
environments. Rather than replacing the tools, configuration files, or workflows
a project already uses, Kraf resolves them into an environment plan, then
materializes it wherever it's needed — a local shell today; CI, containers, or
an airgapped agent later. Materialization targets change; the resolved plan
doesn't.

Kraf's guiding rule: **materialize only what the workload proves it needs.**

## Vision

Kraf's larger aim is to expose a project as a headless IDE runtime for agents:
not an editor, and not a closed platform, but a layer beneath or beside them.
Optionally paired with isolated environments, it will surface a project's files,
diagnostics, and language tooling — LSP, task runners, debugger, tests — as
tools that replace or complement an agent's own built-in primitives. A
structured `read`, for example, returns project knowledge where a raw file read
returns only bytes.

These tools are reachable through whatever host fits the workload — an agent
CLI, or embedded in code as a library (C or WASM) — all calling the same
runtime. Kraf aims to interoperate with the editors and toolchains a project
already uses rather than replacing them.

## Related docs

- [ARCHITECTURE.md](./ARCHITECTURE.md): architecture.
- [BUILDING.md](./BUILDING.md): coding rules and conventions.
- [AGENTS.md](./AGENTS.md): AI agent instructions.
