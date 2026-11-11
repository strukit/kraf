# Review instructions for Copilot

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_STANDARDS.md](../docs/standards/GENERAL_STANDARDS.md#normative-language).
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords.

## Overview

When a reviewer reviews a pull request in this repository, it MUST read
[AGENTS.md](../AGENTS.md) in full before any other document, then follow it,
starting with the traversal of [INDEX.md](../INDEX.md) that its opening section
defines. The reviewer MUST NOT skip any section of AGENTS.md, including the
Reviews, Findings, and Report sections.

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

**Regra quebrada:** <the rule text, quoted, with its file and section, or
"nenhuma regra explícita" when the issue is a correctness defect>

### Correção

**Atual** and **Sugestão** in a diff block:

```diff
- <current line>
+ <suggested line>
```

A field that the source does not inform MUST be written as `não informado`. This
applies to Severity, Category, and every Environment field. Environment MUST be
filled for every reviewer, not only for GitHub Copilot.

The fields MUST keep the order above and MUST NOT change their names.

A comment that does not follow one of these forms MUST NOT be published.

## Links

- [AGENTS.md](../AGENTS.md): Entry point for agent instructions and the
  repository standards they MUST follow.
- [TOOLS.md](../docs/TOOLS.md): Tools declared in this repository.
