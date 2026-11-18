---
description: Where a person new to Kraf starts, how the repository's documents and configuration connect, why, and how to find what applies to a change.
tags: [development, onboarding, discovery, guide]
type: ["guide"]
scope: ["**/*"]
language: en
---

# Development

> [!NOTE]
> The keywords follow RFC 2119 and RFC 8174 across three levels of force:
>
> - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
> - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
> - **Optional:** MAY, OPTIONAL
>
> See
> [GENERAL_STANDARDS.md](./docs/standards/GENERAL_STANDARDS.md#normative-language).

## Overview

This guide is for a person new to Kraf. It does not repeat what other documents
say: it tells where to start, how the pieces of the repository connect, why they
are arranged that way, and how to find on your own what applies to your change.

## Where to start

You MUST read in this order, and you MAY stop when you have what you need:
1. [README.md](./README.md): what Kraf is, and the index of everything else.
2. [TOOLS.md](./docs/TOOLS.md): the tools the repository uses.
   [What you need](#what-you-need) says which of them to install.
3. The [standards index](./docs/standards/README.md): the rules a change
   follows.
4. The [architecture overview](./docs/ARCHITECTURE.md), then the `README.md` of
   the module you will change.

[AGENTS.md](./AGENTS.md) is written for agents. You do not need it to work, but
it explains how agents behave in this repository. Agents MUST read
[AGENTS.md](./AGENTS.md), even when a copy of it is already in their memory,
because that copy can be out of date.

## What you need

- **Required:** the Rust toolchain, through [rustup](./docs/TOOLS.md#rustup),
  which installs the version and components pinned in
  [rust-toolchain.toml](./rust-toolchain.toml). On Windows, also the
  [Visual Studio Build Tools](./docs/TOOLS.md#visual-studio-build-tools). This
  is enough to build, test, and lint.
- **For other files:** [dprint](./docs/TOOLS.md#dprint), to format JSON,
  Markdown, TOML, and Dockerfile files, and [cspell](./docs/TOOLS.md#cspell), to
  check the spelling of any file.
- **To reproduce CI:** Docker, with the
  [build container tools](./docs/TOOLS.md#build-container). Writing and testing
  code does not need them.

## How the pieces connect

- **Documents form a graph.** Each document links, in its Links section, to its
  [nearest index](./docs/standards/GENERAL_STANDARDS.md#link-navigation), and
  that index links back. From [README.md](./README.md) you reach every document,
  and from every document you can go back.
- **Metadata says what a document is about.** Each document starts with a
  [metadata block](./docs/standards/GENERAL_STANDARDS.md#document-metadata). Its
  `scope` names the files the document applies to.
- **TOOLS.md points, the configuration holds.** Each entry of
  [TOOLS.md](./docs/TOOLS.md) declares a tool. When its `version` or `config`
  field names a file, that file holds the version or the settings, such as
  [rust-toolchain.toml](./rust-toolchain.toml) and [tools.toml](./tools.toml),
  and the tools read that same file. A tool whose `version` is `not-pinned` has
  no version file.
- **CI uses the same files.** The
  [pull request workflow](./.github/workflows/pull-request.yml) calls the
  [Docker Bake action](./.github/actions/bake/action.yml), which builds the
  targets of [docker-bake.hcl](./docker-bake.hcl) from the repository's own
  Dockerfiles, with the toolchain that the repository pins.

## Why it is arranged this way

- **One source for each fact.** A version or a setting lives only in the file
  that the tool reads, so no document goes stale when it changes.
- **Read only what a task needs.** The graph and the `scope` of each document
  let people and agents find the rules for their files without reading the whole
  repository.
- **CI matches local work.** CI builds in containers from the files of the
  repository, so what it checks follows the same pinned toolchain and the same
  configuration that you use locally.

## Find what applies to your change

1. **Tools.** In [TOOLS.md](./docs/TOOLS.md), find the entries whose `files`
   match the files you changed. Their `capabilities` say whether a tool checks,
   fixes, or tests, and `run_by` says what invokes it. The help of each tool
   says how to run it.
2. **Rules.** In the [standards index](./docs/standards/README.md), open each
   standard whose `scope` matches your files. A `scope` is read as
   [File Patterns](./docs/standards/GENERAL_STANDARDS.md#file-patterns) defines.
3. **CI.** Follow the
   [pull request workflow](./.github/workflows/pull-request.yml) through the
   files it uses to see what runs on your pull request.

## Links

- [README.md](./README.md): What Kraf is and its vision, and the index of the
  repository's documents, modules and other files.
- [TOOLS.md](./docs/TOOLS.md): Tools declared in this repository, with their
  purpose, configuration, capabilities, and version.
- [STANDARDS.md](./docs/standards/README.md): Index of the standards that govern
  this repository, to help people and agents navigate it on their own through
  the document graph.
- [ARCHITECTURE.md](./docs/ARCHITECTURE.md): Cross-domain map of Kraf's system
  context, modules, and how they depend on each other.
