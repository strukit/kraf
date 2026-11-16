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
