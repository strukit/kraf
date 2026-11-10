# Review instructions for Copilot

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_STANDARDS.md](../docs/standards/GENERAL_STANDARDS.md#normative-language).
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords.

## Overview

When a reviewer reviews a pull request in this repository, it MUST follow
[AGENTS.md](../AGENTS.md), mainly its Reviews, Findings, and Report sections.

## Finding forms

Each finding MUST follow one of these forms, in this order:

1. Cite the AGENTS.md rule that the change violates, by file and section name,
   with the starting line of the code.
2. Describe a correctness defect: a wrong result or a contradiction with the
   specification, with the file and the starting line.
3. As a last resort, use the template below, written in Markdown. It MUST start
   with the `ID:` field, and the `Issuer:` field MUST name the tool, the harness,
   the model, and the reasoning effort that produced the finding.

- **ID**: <number, from the review tool>
- **Issue**: <header, title, or phrase that identifies the issue>
- **Issue Level**: <blocking | recommendation>
- **Detail**: `<issue detail here>`
- **Issue Location**: <file:line:column>
- **Issuer**: <tool name>, <harness>, <model>, <effort>
- **Raw**: <original issue note>

The fields MUST keep the order above and MUST NOT change their names.

A comment that does not follow one of these forms MUST NOT be published.

## Additional rules

- The AGENTS.md rules MUST be applied literally, without extending them or
  replacing them with the reviewer's own reading.
- A finding MUST NOT report what a tool declared in [TOOLS.md](../docs/TOOLS.md)
  already checks on its own.
- A finding MUST NOT report content outside the scope of the change.
- The project memory MUST NOT be used as a review criterion.

## Links

- [AGENTS.md](../AGENTS.md): Entry point for agent instructions and the
  repository standards they MUST follow.
- [TOOLS.md](../docs/TOOLS.md): Tools declared in this repository.
