---
title: Environment variables intent
summary: Why Kraf resolves and projects environment variables through a declarative model.
tags: [environment, env-var, intent, security, projection]
---

## Problem

Development environments inherit a large, implicit set of environment variables
from the host. Their names, formats, availability, and meaning vary across
platforms. Values may also come from personal preferences, project
configuration, organization policy, or the current Kraf session.

When each tool or module reads the host environment directly, the resulting
behavior is difficult to understand, reproduce, audit, and evolve. It also
allows undeclared host state to reach managed processes by accident.

## Intent

This module should resolve environment variables through one declarative model
and produce an explicit projection for each managed process.

The model must make it possible to:

- normalize platform-specific aliases behind canonical declarations;
- combine values from Host, User, Organization, Project, and Session sources;
- evolve source loading and resolution rules without spreading environment logic
  across consumers;
- make the environment received by a managed process explicit and reviewable.

## Principles

### Static declarations

Environment declarations are static capabilities.

Declarations are embedded in source code. Plugin declarations are embedded in
plugin source code or immutable plugin metadata distributed with the plugin. A
declaration identifies an allowed variable and its behavior; it does not contain
the final runtime value.

Dynamic sources may provide values for existing declarations. They must not
create new declarations, request undeclared host values, or expand the set of
variables that a process may receive.

### Security access

Only the environment module may read raw environment values from the host.

Other modules and plugins may obtain a resolved value only through a declaration
they own or explicitly declare as a dependency. This module must not expose an
operation that enumerates all host environment variables, nor arbitrary
string-based host lookups outside this module.

### Explicit projection

Importing an alias and exporting a value are separate permissions.

Knowing that a host alias exists does not authorize exporting it. Resolving a
value does not authorize projecting it. A managed process receives only the
canonical keys and export aliases explicitly permitted by declarations.

This makes the projected environment an allowlist rather than an inherited copy
of the host environment.

## Boundaries

This module owns declaration, resolution, and projection of environment values.

It does not:

- create or manage process sessions;
- choose a terminal or launch a process;
- sandbox filesystem, network, permissions, or host processes;
- define every future source format or precedence rule.

Session creation and process launch belong to the trap module. The trap provides
session-owned values, requests a resolved projection, and applies that
projection to its managed process.
