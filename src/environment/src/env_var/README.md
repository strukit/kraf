---
description: Declarative model, resolution, and projection of environment variables in Kraf.
tags: [environment, env-var, resolution, projection]
type: ["readme", "index"]
scope: ["src/environment/src/env_var/**/*"]
language: en
---

This module defines and resolves Kraf's declarative model for environment
variables: their sources (host, organization, user, project, and session),
aliases, and projections exported to Kraf-managed processes.

> **Note:** this document describes Kraf's environment-variable model, including
> concepts and sources planned for its evolution. It does not imply that all of
> these capabilities are already implemented.

## Overview

- [**Sources**](#sources): where environment-variable values, configuration, and
  restrictions come from.
- [**Declaration**](#declarations): the canonical definition of a variable.
- [**Resolver**](#resolution): the component that applies declarations to values
  from sources to produce an effective value.
- [**Projection**](#projection): the set of effective variables that Kraf
  exports to a managed process.

### Sources

- **Host:** provides values available in the current environment and platform.
- **User:** provides persistent personal preferences.
- **Organization:** provides shared configuration and policies enforced by the
  organization.
- **Project:** provides versioned, project-specific configuration.
- **Session:** provides ephemeral values controlled by Kraf.

### Declarations

A canonical definition of a variable in Kraf. A declaration specifies:

- its canonical key;
- import and export aliases;
- sources eligible to provide a value;
- resolution strategy;
- fallback;
- keys eligible for projection to a child process.

### Resolution

The resolver applies declarations to values available from sources and produces
the effective value of each variable.

For each declaration, it defines:

- precedence between sources;
- how values are combined, when applicable;
- how conflicts or missing values are handled;
- when a fallback is applied;
- how placeholders in values and fallbacks are expanded.

### Projection

A projection is the set of effective variables exported by Kraf to a managed
process.

A declaration can produce one internal canonical key and zero or more export
keys. Not every resolved value is projected to a child process.

## Links

- [Intent](./docs/env-var-intent.md): Why Kraf resolves and projects environment
  variables through a declarative model.
- [Declarations specification](./docs/env-var-declarations/env-var-declarations.spec.md):
  Requirements for declaring environment variables.
- [Declarations acceptance](./docs/env-var-declarations/env-var-declarations.acceptance.md):
  Acceptance scenarios for declaring environment variables.
- [Resolution specification](./docs/env-var-resolution/env-var-resolution.spec.md):
  Requirements for resolving declared environment variables.
- [Resolution acceptance](./docs/env-var-resolution/env-var-resolution.acceptance.md):
  Acceptance scenarios for environment-variable resolution.
- [Projection specification](./docs/env-var-projection/env-var-projection.spec.md):
  Requirements for projecting resolved environment variables to managed
  processes.
- [Projection acceptance](./docs/env-var-projection/env-var-projection.acceptance.md):
  Acceptance scenarios for projecting environment variables.
- [Host specification](./docs/env-var-host/env-var-host.spec.md): Requirements
  for platform-specific environment-variable mappings.
- [Host acceptance](./docs/env-var-host/env-var-host.acceptance.md): Acceptance
  scenarios for platform-specific environment-variable mappings.
- [Access specification](./docs/env-var-access/env-var-access.spec.md):
  Requirements for how modules and plugins access resolved environment values.
- [Access acceptance](./docs/env-var-access/env-var-access.acceptance.md):
  Acceptance scenarios for how modules and plugins access environment values.
- [README.md](../../../../README.md): What Kraf is and its vision, and the index
  of the repository's documents, modules and other files.
