---
title: CODE STANDARDS
description: "Code standards for dependencies, program structure, platforms, tooling, CI, agent configuration, and review."
tags: [standards, code, dependencies, platforms, tooling, ci]
type: ["standard"]
scope: ["type:code"]
---

# Code Standards

> [!NOTE]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> See
> [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#normative-language).

</br>

> [!IMPORTANT]
> This is a standard document. It works like a constitution. Its rules MUST NOT
> be bypassed by ad-hoc exceptions. See
> [DOC_STANDARDS.md](./DOC_STANDARDS.md#standard-documents).

## Overview

This document defines the code standards of the repository: dependencies,
program structure, platforms, declared tool requirements, ignored files, CI,
agent configuration, and review.

## Rust Modules

```yaml
scope:
  include: ["**/*.rs"]
```

This section defines the structure of Rust folder modules.

### Rules

- A Rust `mod.rs` file MUST only declare or re-export submodules, such as with
  `pub mod ...`, and MUST NOT hold types, logic, or implementation.
- When a folder-module needs its own core type or logic file, that file MUST be
  named after the folder with a suffix, such as `foo/foo_core.rs`; a submodule
  MUST NOT share its parent's exact name because that causes
  `clippy::module_inception`.

## Tests

This section defines the preferred size and structure of tests.

### Rules

- Tests MAY repeat content instead of extracting a shared helper or constant
  solely to avoid repetition between tests.
- A shared constant or fixture MAY be used when inlining it would make the test
  unnecessarily large or obscure its behavior.

### Best practices

- Tests SHOULD be small and direct, without over-engineering.
- Tests SHOULD be self-contained: small setup SHOULD live inside the test so
  readers can see what it uses without navigating elsewhere.

## Review

This section defines an additional review requirement for features with a
specification.

### Rules

- In a feature that has a specification, a review MUST also apply
  [the review rules in SPEC_STANDARDS.md](./SPEC_STANDARDS.md#review).

## Dependencies

This section defines how code chooses its dependencies.

### Rules

- Code MAY depend on official platform bindings, foundational runtimes and
  packages, and packages from big-tech companies, the Linux Foundation, and
  similar organizations.

### Examples

- Terminal handling uses `std::io::IsTerminal` and minimal syscalls, not an
  external terminal library.

### Best practices

- Code SHOULD prefer the standard library over external libraries, except the
  categories stated in Rules, and SHOULD keep dependencies few and consolidated.
- Code SHOULD NOT add a convenience wrapper on top of another library, or a
  wrapper of a wrapper.

## Program Structure

This section defines where program logic lives.

### Rules

- Real logic MUST live in the lib, and the binary MUST only consume it, so that
  the logic stays testable and reusable outside the CLI, such as by another
  binary or another consumer.

## Multi-platform Support

This section defines how a domain supports more than one platform.

### Rules

- Every domain MUST be designed for Linux, Mac, and Windows from the start, not
  added later.
- Platform-specific code MUST be isolated, and shared logic MUST stay separate
  from platform-specific logic.

### Examples

- Platform-specific code is isolated in files such as `_macos.rs`, `_linux.rs`,
  and `_windows.rs`.

## Declared Tool Requirements

This section defines where build and tool requirements are declared.

### Rules

- Build and tool requirements MUST be declared in dedicated, versioned files,
  not hardcoded in scripts or docs.

### Examples

- [`.msvc.json`](../../.msvc.json) declares what is needed to build on Windows:
  the VC++ Tools components and the Windows SDK version.

## Ignored Files

This section defines which files the repository and the Docker build context
ignore.

### Rules

- [`.gitignore`](../../.gitignore) MUST deny everything by default and allowlist
  only what the repository needs, as its header comment explains.
- [`.dockerignore`](../../.dockerignore) MUST be a symlink to
  [`.gitignore`](../../.gitignore), so that the same allowlist keeps leftover
  files out of the Docker build context.

## CI

This section defines the reproducibility and build environment of CI.

### Rules

- Everything that runs in CI, such as tests, builds, and releases, MUST be
  reproducible locally.
- CI builds MUST run in Docker Bake for every platform except Mac, which Docker
  cannot build for. Local builds MAY run outside Docker Bake.

## Agent Configuration

This section defines which agent configuration the repository commits.

### Rules

- Configuration exclusive to a single agent tool, such as Claude-only or
  Codex-only configuration, MUST NOT be committed.
- Configuration portable across agents MAY be committed.
- A persona MAY be committed. If it is committed, it MUST be in a vendor-neutral
  form, and only its tool-specific glue MUST stay in the developer's own
  environment.
- An agent-related document that is committed MUST be vendor-neutral, and MUST
  NOT name a specific agent tool or model.

### Best practices

- Configuration portable across agents SHOULD be committed when it encodes a
  repository rule, such as skills and agent personas bound to this repository's
  standards, like a Rust reviewer that enforces
  [CODE_STANDARDS.md](./CODE_STANDARDS.md).

## Platform Names

This section defines how platform names are written.

### Rules

- In prose, docs, and diagrams, platform names MUST be written as `Linux`,
  `Mac`, and `Windows`, and the `macOS` styling MUST NOT be used.
- In code, each language MAY have its own standard for platform names, and
  that standard MUST take precedence over the preceding rule, only within the
  code of that language.

## Links

- [README.md](./README.md): Index of the standards that govern this repository, to help people and agents navigate it on their own through the document graph.
- [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md): General standards for this
  repository, document layout and metadata, normative language, the document
  graph, links, navigation, and language neutrality.
- [DOC_STANDARDS.md](./DOC_STANDARDS.md): Standards for standard documents:
  what they are, their layout, precedence between rules, and how they word
  their rules.
