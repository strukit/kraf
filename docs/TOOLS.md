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
- clippy: lints Rust code (invoked by cargo).
  - Config: [Cargo.toml](../Cargo.toml).
  - Capabilities: check, partial autofix.
  - Version: [rust-toolchain.toml](../rust-toolchain.toml).
- cspell: checks spelling.
  - Config: [cspell.jsonc](../cspell.jsonc).
  - Capabilities: check.
  - Version: not pinned.
- EditorConfig: helps editors and IDEs keep the line ending of every file as LF.
  It has no command to run; editors and IDEs read it.
  - Config: [.editorconfig](../.editorconfig).

### Language server

- rust-analyzer: provides language-server features for Rust code, such as
  diagnostics, with checks run through clippy.
  - Config: [rust-analyzer.toml](../rust-analyzer.toml).
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

- cargo: builds, tests, and lints Rust code, the latter through clippy. It is
  the tool for everyday builds.
  - Config: [.cargo/config.toml](../.cargo/config.toml).
  - Version: [rust-toolchain.toml](../rust-toolchain.toml).
- Docker Bake: builds release artifacts in containers, as CI does, for a clean,
  containerized build.
  - Config: [docker-bake.hcl](../docker-bake.hcl).
  - Version: [action.yml](../.github/actions/bake/action.yml), for CI; not
    pinned for local builds.

## Related

- [README.md](../README.md): what Kraf is.
- [AGENTS.md](../AGENTS.md): how agents use the tools declared here.
