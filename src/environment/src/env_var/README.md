---
title: Environment variables
summary: Declarative model, resolution, and projection of environment variables in Kraf.
tags: [environment, env-var, resolution, projection]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_CONVENTIONS.md](../../../../docs/GENERAL_CONVENTIONS.md#normative-language).
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords.

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

- **`Host`:** the declaration resolves from its import aliases, which can
  include its canonical key, or from its fallback; the canonical key is not read
  on its own.
- **`Provided`:** Kraf generates the canonical value, such as a session value.
  The host value of the same key can be read, but it is not the value that is
  resolved or projected.

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

## Documentation

- [Intent](./docs/env-var-intent.md): purpose, boundaries, and evolution of this
  module.
- [Declarations](./docs/env-var-declarations/env-var-declarations.spec.md):
  declaration requirements and acceptance scenarios.
- [Resolution](./docs/env-var-resolution/env-var-resolution.spec.md): resolution
  requirements and acceptance scenarios.
- [Projection](./docs/env-var-projection/env-var-projection.spec.md): projection
  requirements and acceptance scenarios.
- [Host mappings](./docs/env-var-host/env-var-host.spec.md): platform mapping
  requirements and acceptance scenarios.
- [Access](./docs/env-var-access/env-var-access.spec.md): access requirements
  and acceptance scenarios.
- [README.md](../../../../README.md): what Kraf is.
