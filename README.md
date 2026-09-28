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

The project is currently under active development. Its first working slice is
environment variable resolution and isolation — an isolated `HOME` and working
directory per run, real system state passed through deliberately rather than by
accident — materialized into a shell you can use today.

## Contributing

See [BUILDING.md](./BUILDING.md) for coding rules and conventions, and
[AGENTS.md](./AGENTS.md) for AI agent instructions.
