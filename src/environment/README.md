---
description: Cross-platform environment variable resolution and mapping in Kraf.
tags: [kraf, environment, env-var, resolution]
type: ["readme", "index"]
scope: ["src/environment/**/*"]
language: en
---

# kraf-environment

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

`kraf-environment` resolves and maps environment variables across platforms. It
builds on `kraf-core` and is used by `kraf-lib`.

## Links

- [Environment Variables](./src/env_var/README.md): Declarative model,
  resolution, and projection of environment variables in Kraf.
- [README.md](../../README.md): What Kraf is and its vision, and the index of
  the repository's documents, modules and other files.
