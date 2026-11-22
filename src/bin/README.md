---
description: Command-line interface for the kraf development environment layer.
tags: [kraf, cli, binary]
type: ["readme"]
scope: ["src/bin/**/*"]
language: en
---

# kraf

> [!NOTE]
> The keywords follow RFC 2119 and RFC 8174 across three levels of force:
>
> - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
> - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
> - **Optional:** MAY, OPTIONAL
>
> See
> [GENERAL_STANDARDS.md](../../docs/standards/GENERAL_STANDARDS.md#normative-language).

## Overview

This crate provides the `kraf` command. It builds on `kraf-core`, including its
`environment` module, and holds the experimental `process` module that starts
the child process.

## Links

- [README.md](../../README.md): What Kraf is and its vision, and the index of
  the repository's documents, modules and other files.
- [ARCHITECTURE.md](../../docs/ARCHITECTURE.md): Cross-domain map of Kraf's
  system context, modules, and how they depend on each other.
