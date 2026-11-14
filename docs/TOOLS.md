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

This document defines the rules for tools declared in this repository. Tools and
their configurations are listed in [tools.toml](../tools.toml).

## Rules

- Agents MUST update [tools.toml](../tools.toml) in a pull request that adds or
  removes the configuration of a tool.
- Agents MUST NOT list in [tools.toml](../tools.toml) a file read only by
  editors or IDEs (e.g. `.editorconfig`), because it configures no tool an agent
  can invoke.

## Links

- [README.md](../README.md): What Kraf is and its vision, and the index of the
  repository's documents, modules and other files.
- [AGENTS.md](../AGENTS.md): Entry point for agent instructions and the rules
  they MUST follow.
