---
title: Tools
summary: Tools declared in this repository, with their purpose, configuration, capabilities, and version.
tags: [tools, tooling, formatting, linting, toolchain, build]
applies_to: [all]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md#normative-language). This
> document MUST NOT use alternative words or lowercase variants as normative
> keywords.

## Tools

Each entry states the tool's purpose and, when they apply:

- Config: the file that configures the tool.
- Run: how an agent runs the tool, when it is not through the tool's own
  command.
- Capabilities: what the tool does with the content it checks.
  - check: reports problems.
  - autofix: fixes every problem it reports.
  - partial autofix: fixes some of the problems it reports.
- Version: the file that pins the tool's version.

### Formatting and linting

- dprint: formats files, Rust code through rustfmt.
  - Config: [.dprint.jsonc](../.dprint.jsonc).
  - Capabilities: check, autofix.
  - Version: [.dprint.jsonc](../.dprint.jsonc) pins the plugins; the dprint CLI
    is not pinned.
- rustfmt: formats Rust code (invoked by dprint).
  - Config: [rustfmt.toml](../rustfmt.toml).
  - Capabilities: check, autofix.
  - Version: [rust-toolchain.toml](../rust-toolchain.toml).
- clippy: lints Rust code (invoked by cargo and by rust-analyzer).
  - Config: [Cargo.toml](../Cargo.toml).
  - Capabilities: check, partial autofix.
  - Version: [rust-toolchain.toml](../rust-toolchain.toml).
- cspell: checks spelling.
  - Config: [cspell.jsonc](../cspell.jsonc).
  - Capabilities: check.
  - Version: not pinned.

### Language server

- rust-analyzer: provides language-server features for Rust code, such as
  diagnostics. Its checks run `cargo lint`, an alias for clippy defined in
  [.cargo/config.toml](../.cargo/config.toml).
  - Config: [rust-analyzer.toml](../rust-analyzer.toml).
  - Run: through an LSP tool, when one is available to the agent.
  - Capabilities: check, partial autofix.
  - Version: [rust-toolchain.toml](../rust-toolchain.toml).

### Toolchain

- rustup: installs the pinned Rust toolchain and its components, including
  cargo, clippy, rustfmt, and rust-analyzer.
  - Config: [rust-toolchain.toml](../rust-toolchain.toml).
  - Version: [Dockerfile.linux](../Dockerfile.linux) and
    [Dockerfile.windows](../Dockerfile.windows), for container builds.
- Visual Studio Build Tools (Windows only): installs the VC++ Tools components
  and the Windows SDK needed to build on Windows.
  - Config: [.msvc.json](../.msvc.json).
  - Version: [.msvc.json](../.msvc.json) pins the Windows SDK; the Build Tools
    are not pinned.

### Build

- cargo: builds, tests, and lints Rust code, the latter through clippy. Used for
  everyday builds.
  - Config: [.cargo/config.toml](../.cargo/config.toml).
  - Version: [rust-toolchain.toml](../rust-toolchain.toml).
- Docker Bake: builds release artifacts in containers, as CI does, for a clean,
  containerized build, on the BuildKit daemon that Docker Compose starts.
  - Config: [docker-bake.hcl](../docker-bake.hcl).
  - Version: not pinned.
- Docker Compose: starts the BuildKit daemon that Docker Bake uses as a remote
  builder, in CI and locally.
  - Config: [compose.yaml](../compose.yaml).
  - Version: [compose.yaml](../compose.yaml) pins the BuildKit image; Docker
    Compose is not pinned.

### CI

- gh actions-lock: locks the GitHub Actions that the workflows use to their
  commits.
  - Config: [actions.lock](../.github/workflows/actions.lock).
  - Version: not pinned.

## Related

- [README.md](../README.md): what Kraf is.
- [AGENTS.md](../AGENTS.md): how agents use the tools declared here.
