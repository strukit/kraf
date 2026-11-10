# Review instructions for Copilot

When you review a pull request in this repository, the reviewer MUST follow
[AGENTS.md](../AGENTS.md), mainly its Reviews, Findings, and Report sections.

Each finding MUST follow one of these forms, in this order:

1. Cite the AGENTS.md rule that the change violates, by file and section name,
   with the starting line of the code.
2. Describe a correctness defect: a wrong result or a contradiction with the
   specification, with the file and the starting line.
3. As a last resort, use the template below. It MUST start with the `Origem:`
   line, which marks where the template came from, and that line MUST be copied
   unchanged.

```text
Origem: .github/copilot-instructions.md, modelo v1
ID: <id>
Recommendation: <blocking | recommendation>
Name: <name>
Detail: <detail, with the starting line>
Source: <AGENTS.md rule, declared tool, external skill, or correctness defect>
File: <path relative to the repository>
```

A comment that does not follow one of these forms MUST NOT be published.

Additional rules:

- Apply the AGENTS.md rules literally, without extending them or replacing them
  with your own reading.
- Do not report what a tool declared in `docs/TOOLS.md` already checks on its
  own.
- Do not report content outside the scope of the change.
- The project memory MUST NOT be used as a review criterion.
