---
description: Tools declared in this repository, with their purpose, configuration, capabilities, and version.
tags: [tools, tooling, formatting, linting, toolchain, build]
type: ["guide", "standard"]
scope: ["**/*"]
language: en
---

# Tools

> [!NOTE]
> The keywords follow RFC 2119 and RFC 8174 across three levels of force:
>
> - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
> - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
> - **Optional:** MAY, OPTIONAL
>
> See
> [GENERAL_STANDARDS.md](./standards/GENERAL_STANDARDS.md#normative-language).

</br>

> [!IMPORTANT]
> This is a standard document:
>
> - Works like a constitution
> - Rules and Instructions MUST be mandatory and normative
> - Rules and Instructions MUST NOT be ignored
>
> See [DOC_STANDARDS.md](./standards/DOC_STANDARDS.md#standard-documents).

## Overview

This document defines the rules for tools declared in this repository, and
declares each tool, with its purpose, configuration, capabilities, and version.

## Terms

- **Check**: a run of a tool that reports problems and changes no file.
- **Autofix**: a run of a tool that fixes, by itself, every problem that its
  check would report, such as a formatter that rewrites the files. Its check
  does not need to run again after it.
- **Partial autofix**: a run of a tool that fixes most of the problems that its
  check reports, and reports the ones it could not fix.
- **Uncommitted files**: the files with uncommitted changes in the git working
  tree, staged, unstaged, or untracked.
- **Staged files**: the files whose changes are staged in git.
- **Category**: the kind of work a tool does, declared in its `category` field
  and used to group the [Tools](#tools) section:
  - [**format**](#format): rewrites files to a consistent layout.
  - [**lint**](#lint): reports problems in content, such as code, spelling, or
    diagnostics.
  - [**setup**](#setup): installs the toolchain and components that other tools
    need.
  - [**build**](#build): builds, tests, and lints code locally.
  - [**build-container**](#build-container): builds in containers, as CI does,
    or provides what those builds run on.
  - [**ci**](#ci): maintains what the CI workflows use.

## Rules

- Agents MUST treat the changes that an autofix makes as part of the unit of
  work, and MUST NOT report them to the user.
- Agents MUST run the check of a tool with partial autofix in the same unit of
  work as its fix, and MAY run it at any point after the fix.
- Agents MUST verify, through the help of a tool, whether it can run only on the
  uncommitted files or only on the staged files.
- Agents SHOULD run a tool only on the uncommitted files when it can, instead of
  naming each file that the unit of work changed.
- Agents MAY stage files and run a tool only on the staged files when they will
  commit only part of the changes.
- Agents MUST make sure that each run of a tool covers the files that the unit
  of work changed, and MAY let it cover other files with changes.
- Agents MUST NOT run a tool on a file without changes.
- Agents MUST update the [Tools](#tools) section in a pull request that adds or
  removes the configuration of a tool.
- Agents MUST NOT list in the [Tools](#tools) section a file read only by
  editors or IDEs (e.g. `.editorconfig`), because it configures no tool an agent
  can invoke.

## Tools

### Format

#### dprint

Formats files, Rust code through rustfmt.

```yaml
name: dprint
category: format
version: 0.64.3
repository: https://github.com/dprint/dprint
config: ./.dprint.jsonc
files: [json, markdown, toml, rust, other]
capabilities: [check, autofix]
```

#### rustfmt

Formats Rust code, invoked by dprint.

```yaml
name: rustfmt
category: format
run_by: [dprint]
config: ./rustfmt.toml
files: [rust]
capabilities: [check, autofix]
version: ./rust-toolchain.toml
```

### Lint

#### clippy

Lints Rust code, invoked by cargo.

```yaml
name: clippy
category: lint
run_by: [cargo]
config: ./Cargo.toml
files: [rust]
capabilities: [check, partial-autofix]
version: ./rust-toolchain.toml
```

#### cspell

Checks spelling.

```yaml
name: cspell
category: lint
config: ./cspell.jsonc
files: [any]
capabilities: [check]
version: not-pinned
```

#### rust-analyzer

Provides language-server features for Rust code, such as diagnostics.

```yaml
name: rust-analyzer
category: lint
config: ./rust-analyzer.toml
files: [rust]
capabilities: [check, partial-autofix]
version: ./rust-toolchain.toml
```

### Setup

#### rustup

Installs the pinned Rust toolchain and its components, including cargo, clippy,
rustfmt, and rust-analyzer.

```yaml
name: rustup
category: setup
config: ./rust-toolchain.toml
version: [./Dockerfile.linux, ./Dockerfile.windows]
```

#### Visual Studio Build Tools

Installs, on Windows only, the VC++ Tools components and the Windows SDK needed
to build.

```yaml
name: Visual Studio Build Tools
category: setup
config: ./.msvc.json
version: ./.msvc.json
```

### Build

#### cargo

Builds, tests, and lints Rust code, the latter through clippy.

```yaml
name: cargo
category: build
config: ./.cargo/config.toml
files: [rust]
capabilities: [check]
version: ./rust-toolchain.toml
```

### Build container

#### Docker Bake

Builds release artifacts in containers, as CI does.

```yaml
name: Docker Bake
category: build-container
version: 0.39.0
repository: https://github.com/docker/buildx
config: ./docker-bake.hcl
capabilities: [check, build]
```

#### Docker Compose

Starts the BuildKit daemon service, in CI and locally.

```yaml
name: Docker Compose
category: build-container
version: 5.7.1
repository: https://github.com/docker/compose
config: ./compose.yaml
```

#### BuildKit

Provides the daemon that Docker Bake uses as a remote builder.

```yaml
name: BuildKit
category: build-container
version: 0.35.0
repository: https://github.com/moby/buildkit
config: ./compose.yaml
```

#### Dockerfile frontend

Provides the Dockerfile syntax that Docker Bake uses.

```yaml
name: Dockerfile frontend
category: build-container
version: 1.29.0
repository: https://github.com/moby/buildkit
config: ./docker-bake.hcl
```

### CI

#### gh actions-lock

Locks the GitHub Actions that the workflows use to their commits.

```yaml
name: gh actions-lock
category: ci
version: 0.4.1
config: ./.github/workflows/actions.lock
```

## Links

- [README.md](../README.md): What Kraf is and its vision, and the index of the
  repository's documents, modules and other files.
- [AGENTS.md](../AGENTS.md): Entry point for agent instructions and the rules
  they MUST follow.
