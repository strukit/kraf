---
description: Vendor-neutral instructions for reviewing pull requests in this repository, with the finding forms and report template.
tags: [review, pull-request, agents, instructions]
type: ["guide"]
scope: ["**/*"]
---

# Review instructions

> [!NOTE]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> See
> [GENERAL_STANDARDS.md](./standards/GENERAL_STANDARDS.md#normative-language).

## Overview

When a reviewer reviews a pull request in this repository, it MUST follow
[AGENTS.md](../AGENTS.md), mainly its Reviews, Findings, and Report sections.

## Finding forms

Each finding MUST follow one of these forms, in this order:

1. Cite the AGENTS.md rule that the change violates, by file and section name,
   with the starting line of the code.
2. Describe a correctness defect: a wrong result or a contradiction with the
   specification, with the file and the starting line.
3. As a last resort, use the template below. It MUST start with the `Origem:`
   line, which marks where the template came from, and that line MUST be copied
   unchanged.

```text
Origem: docs/REVIEW_INSTRUCTIONS.md, modelo v1
ID: <id>
Recommendation: <blocking | recommendation>
Name: <name>
Detail: <detail, with the starting line>
Source: <AGENTS.md rule, declared tool, external skill, or correctness defect>
File: <path relative to the repository>
```

A comment that does not follow one of these forms MUST NOT be published.

## Additional rules

- The AGENTS.md rules MUST be applied literally, without extending them or
  replacing them with the reviewer's own reading.
- A finding MUST NOT report what a tool declared in [TOOLS.md](./TOOLS.md)
  already checks on its own.
- A finding MUST NOT report content outside the scope of the change.
- The project memory MUST NOT be used as a review criterion.

## Links

- [AGENTS.md](../AGENTS.md): Entry point for agent instructions and the
  repository standards they MUST follow.
- [TOOLS.md](./TOOLS.md): Tools declared in this repository.
