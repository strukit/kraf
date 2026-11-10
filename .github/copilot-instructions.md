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

1. Cite the rule that the change violates, from the repository document that
   defines it, by file and section name, with the starting line of the code. The
   rule MAY be in AGENTS.md or in any other convention document.
2. Describe a correctness defect: a wrong result or a contradiction with the
   specification, with the file and the starting line.
3. As a last resort, use the template below.

Every report MUST use the template below, whether it is posted explicitly or
implicitly, including any report that is sent under a request to send it.

#### Issue <number, from the review tool>: <title that identifies the issue> [<blocking | recommendation>]

- **Severity**: <High | Medium | Low>
- **Category**: <category>
- **Environment**:
  - Skill: <skill name>
  - Versão: <version>
  - Harness: <harness>
  - Modelo: <model>
  - Esforço: <effort>
- **File**: `<path relative to the repository>`
- **Lines**: <start-end>

### Problem

<what the issue is and why it matters>

**Regra quebrada:** <the rule text, quoted, with its file and section, or "nenhuma regra explícita" when the issue is a correctness defect>

### Correção

**Atual** and **Sugestão** in a diff block:

```diff
- <current line>
+ <suggested line>
```

A field that the source does not inform MUST be written as `não informado`. This applies to Severity, Category, and every Environment field. Environment MUST be filled for every reviewer, not only for GitHub Copilot.

The fields MUST keep the order above and MUST NOT change their names.

A comment that does not follow one of these forms MUST NOT be published.

## Additional rules

- The AGENTS.md rules MUST be applied literally, without extending them or
  replacing them with the reviewer's own reading.
- A finding MUST NOT report what a tool declared in [TOOLS.md](../docs/TOOLS.md)
  already checks on its own.
- A finding MUST NOT report content outside the scope of the change.
- The project memory MAY be consulted, as the Reviews section of AGENTS.md
  defines, but it MUST NOT be used as the criterion that decides whether
  something is an error. A finding MUST be sustainable from the repository's
  documentation or from a correctness defect.
- When a memory bears on the change, the review MUST finish from the repository
  documentation alone. Then it MUST cite the memory and ask the user at the end.
- If a scenario that a memory points to cannot be sustained from the
  documentation, the review MUST say so in its description, state that it
  depends on the memory and needs deeper validation, and MUST NOT report it as
  an error.

## Links

- [AGENTS.md](../AGENTS.md): Entry point for agent instructions and the
  repository standards they MUST follow.
- [TOOLS.md](../docs/TOOLS.md): Tools declared in this repository.
