---
title: GENERAL CONVENTIONS
description: "Repository conventions: how documents are written, how a convention works, and how the workspace is organized."
tags: [conventions, documentation, agents]
type: ["convention"]
include: ["**/*"]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords. See
> [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md#doc-normative-language).

> [!IMPORTANT]
> This is a convention document: its rules are mandatory and non-negotiable.
> Treat them like a constitution: a rule changes only through a pull request
> that edits this document and MUST NOT be bypassed by an ad-hoc exception in
> code. See
> [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md#convention-documents).

## Overview

This document describes how convention documents are structured and how their
rules work. For repository-wide conventions such as normative language, document
graph, and language neutrality, see
[REPOSITORY_CONVENTIONS.md](./REPOSITORY_CONVENTIONS.md).

**Rule Conventions** define how a convention is laid out, scoped, and worded,
and how rules take precedence. **General Conventions** apply to the workspace:
dependencies, platforms, tools, ignored files, CI, agents, and naming.

## Docs Conventions

```yaml
scope:
  include: ["**/*.md"]
  exclude: [".github/**"]
```

### Doc metadata

This convention defines the metadata that every document declares at its top, so
that an agent can index it and decide whether the task needs it.

```md
---
title: <title>
description: <one-line description>
tags: [<tag>, ...]
type: [<type>, ...]
include: [<type | glob>, ...]
exclude: [<type | glob>, ...] # optional
---
```

#### Terms

- **`type`**: what the document is, with the values of the table below.
- **`include`** and **`exclude`**: the files whose tasks need the document,
  given as types or file patterns.

| Value        | Document                                             |
| ------------ | ---------------------------------------------------- |
| `convention` | a convention document                                |
| `spec`       | an intent, a specification, an acceptance, or an ADR |
| `guide`      | a descriptive document, such as an architecture      |

#### Rules

- Every document MUST start with the metadata of the template above, with every
  field that the template does not mark as optional.
- The metadata MUST be a valid YAML block.
- `type` MUST list one or more types that state what the document is.
- Each value of `type` MUST be taken from the table in Terms.
- Each item of `include` and `exclude` MUST be a type or a file pattern. An item
  equal to a value of the table in Terms MUST be read as a type, and any other
  item MUST be read as a file pattern.
- An item that is a type MUST match every document that has that type in its
  `type` field.
- An item that is a file pattern MUST be read as defined in
  [Convention scope](#convention-scope).
- An agent MAY use `include` and `exclude` to decide whether the task needs a
  document: the task needs it when it creates, edits, or reviews a file that
  matches `include` and does not match `exclude`.
- When a rule that applies to the task requires the agent to read or apply a
  document or a section of it, the agent MUST read that content, whatever the
  document's `include` and `exclude` state.

#### Best practices

- To check the metadata, a review SHOULD verify that it is valid YAML, that each
  field that the template does not mark as optional is present, and that `type`
  lists at least one type.

For normative language, document graph rules, links, and language neutrality,
see [REPOSITORY_CONVENTIONS.md](./REPOSITORY_CONVENTIONS.md).

## Rule Conventions

### Convention documents

This convention defines what a convention document is, how its rules bind and
change, and which rule prevails in a conflict.

```md
> [!IMPORTANT]
> This is a convention document: its rules are mandatory and non-negotiable.
> Treat them like a constitution: a rule changes only through a pull request
> that edits this document and MUST NOT be bypassed by an ad-hoc exception in
> code. See
> [GENERAL_CONVENTIONS.md](<relative-link-to-docs>/GENERAL_CONVENTIONS.md#convention-documents).
```

#### Terms

- **Convention document**: a document whose purpose is to define rules.
- **Convention**: a section of a convention document that groups related rules,
  with its own scope, terms, and rules.

#### Rules

- A convention document MUST carry the note of the template above near the top,
  right after the normative-language note, separated from it only by a blank
  line.
- The rules of a convention document are mandatory: the codebase and agents MUST
  follow them.
- Ad-hoc exceptions MUST NOT be introduced. A rule MUST change only through a
  pull request that edits the convention document defining it.
- A pull request that changes a convention document MUST bring into compliance
  the content that the change makes noncompliant, unless its description lists
  what it leaves for a later pull request.
- Rules MUST be defined only in convention documents. A document that is not a
  convention document MUST NOT introduce rules and MAY only reference or apply
  existing ones.

#### Best practices

- To check the note, a review SHOULD compare it with the template, ignoring line
  breaks and the path of the link.
- When a change rewrites a rule without intending to change its meaning, a
  review SHOULD compare the old and new wording for conditions, scope, subject,
  keyword, pronoun referents, and terms that other documents use for the rule.

#### Convention precedence

This convention defines which rule prevails when two rules conflict.

##### Rules

- A rule explicitly scoped to a document type or context MUST take precedence
  over a more general rule wherever they conflict, only within that scope.
- Non-conflicting general rules MUST remain in effect.
- Such an override MUST be defined in a convention document, and MAY leave
  unchanged the document that defines the general rule.
- A rule that prevails over another MUST cite the convention of the rule it
  overrides.

### Convention layout

```yaml
scope:
  include: ["convention"]
```

This section defines the order and format of the content of a convention.

````md
### <Title>

<Scope block, optional>

<Summary, optional>

```md
<Template, optional>
```

#### Terms (optional)

- **<Term>**: <meaning>.

#### Rules (optional)

- <Rule>.

#### Examples (optional)

- <Example>.

#### Best practices (optional)

- <Guidance at the Recommended level>.
````

#### Rules

- Each convention MUST follow the template above, in the same order and format,
  except for the absolute level of its headings, and MAY omit the parts marked
  as optional.
- The heading levels of the template above MUST be read as relative: the
  convention heading MUST take the level of the convention in its document, and
  Terms, Rules, Examples, and Best practices MUST be headings one level below
  the convention heading.
- A convention within another MUST come after all the parts of the one that
  contains it and MUST follow the same template.
- A group of rules with its own name MUST be a convention, not a label.
- Each item of Rules MUST use only normative keywords of the Required and
  Optional levels, as defined in
  [Doc normative language](#doc-normative-language).
- Each item of Best practices MUST use only normative keywords of the
  Recommended level, as defined in
  [Doc normative language](#doc-normative-language).

### Convention scope

This section defines how a convention declares the files its rules apply to.

```yaml
scope:
  include: ["<type | glob>", ...] # optional
  exclude: ["<type | glob>", ...] # optional
```

#### Terms

- **Convention scope**: the files that the rules of a convention apply to.
- **Scope block**: the template above, as a YAML code block right below a
  convention heading.

#### Rules

- A scope block MUST be valid YAML, with the `scope` key and, under it, only
  `include` and `exclude`, each with a list of quoted items.
- A scope block MUST have `include`, `exclude`, or both.
- Each item of `include` and `exclude` of a scope block MUST be read as defined
  in [Doc metadata](#doc-metadata).
- A file pattern MUST be read relative to the repository root, where `*` matches
  any characters except `/` and `**` matches any number of directories,
  including none.
- The convention scope of a convention MUST be that of the nearest convention
  that contains it, narrowed to what matches `include`, when present, and
  without what matches `exclude`.
- The scope block of a convention that no convention with a scope contains MUST
  have `include`.
- A convention without a scope block MUST take the convention scope of the
  nearest convention that contains it.
- The rules of a convention MUST be applied only to the files in its convention
  scope, and that scope MUST NOT be derived from any other source, such as
  `.gitignore`.
- When neither a convention nor a convention that contains it has a scope block,
  its rules MUST be applied as their text scopes them.

#### Best practices

- To check the convention scope of a convention, a review SHOULD start from the
  outermost convention with a scope block and apply each scope block down to the
  convention, in order.

### Convention language

This convention defines how a convention document words its rules.

#### Rules

- In a convention document, each rule MUST be a list item, and each sentence of
  a list item MUST use a normative keyword. A reason or an example MAY qualify
  that sentence.
- Notes, templates, items of Terms and of Examples, and lists that only
  enumerate values, sources, or links, such as a Related section, MAY contain
  sentences without a normative keyword.
- In a convention document, prose outside list items MAY introduce a convention,
  define a term, or give an example, and MUST NOT state a rule.
- In a convention document, a definition MUST be written as `**Term**:
  meaning.`, before the rules that use the term.

#### Best practices

- To check items, a review SHOULD read each one sentence by sentence and report
  each sentence without a normative keyword, except where a rule exempts it.
- To check prose, a review SHOULD report each paragraph outside list items that
  states a rule.
- To check a definition, a review SHOULD report it when a rule uses the term
  before it.

## General Conventions

### Std-first

This convention defines how code chooses its dependencies.

#### Rules

- Code MAY depend on official platform bindings, foundational runtimes and
  packages, and packages from big-tech companies, the Linux Foundation, and
  similar organizations.

#### Examples

- Terminal handling uses `std::io::IsTerminal` and minimal syscalls, not an
  external terminal library.

#### Best practices

- Code SHOULD prefer the standard library over external libraries, except the
  official platform bindings, foundational runtimes and packages, and packages
  from big-tech companies, the Linux Foundation, and similar organizations that
  Rules permits, and SHOULD keep dependencies few and consolidated.
- Code SHOULD NOT add a convenience wrapper on top of another library, or a
  wrapper of a wrapper.

### Lib-first

This convention defines where the logic of a program lives.

#### Rules

- Real logic MUST live in the lib, and the binary MUST only consume it, so that
  the logic stays testable and reusable outside the CLI, such as by another
  binary or another consumer.

### Multi-platform

This convention defines how a domain supports more than one platform.

#### Rules

- Every domain MUST be designed for Linux, Mac, and Windows from the start, not
  added later.
- Platform-specific code MUST be isolated, and shared logic MUST stay separate
  from platform-specific logic.

#### Examples

- Platform-specific code isolated in files such as `_macos.rs`, `_linux.rs`, and
  `_windows.rs`.

### Declared tool requirements

This convention defines where build and tool requirements are declared.

#### Rules

- Build and tool requirements MUST be declared in dedicated, versioned files,
  not hardcoded in scripts or docs.

#### Examples

- [`.msvc.json`](../.msvc.json) declares what is needed to build on Windows: the
  VC++ Tools components and the Windows SDK version.

### Ignored files

This convention defines which files the repository and the Docker build context
ignore.

#### Rules

- [`.gitignore`](../.gitignore) MUST deny everything by default and allowlist
  only what the repository needs, as its header comment explains.
- [`.dockerignore`](../.dockerignore) MUST be a symlink to
  [`.gitignore`](../.gitignore), so that the same allowlist keeps leftover files
  out of the Docker build context.

### CI

This convention defines how CI runs.

#### Rules

- Everything that runs in CI, such as tests, builds, and releases, MUST be
  reproducible locally.
- CI builds MUST run in Docker Bake for every platform except Mac, which Docker
  cannot build for. Local builds MAY run outside Docker Bake.

### Agents

This convention defines which agent configuration the repository commits.

#### Rules

- Configuration exclusive to a single agent tool, such as Claude-only or
  Codex-only configuration, MUST NOT be committed.
- Configuration portable across agents MAY be committed.
- A persona MUST be committed in a vendor-neutral form, and only its
  tool-specific glue MUST stay in the developer's own environment.

#### Best practices

- Configuration portable across agents SHOULD be committed when it encodes a
  repository rule, such as skills and agent personas bound to the standards of
  this repository, like a Rust reviewer that enforces
  [CODE_CONVENTIONS.md](./CODE_CONVENTIONS.md).

### Naming

#### Platform names

This convention defines how platform names are written.

##### Rules

- In prose, docs, and diagrams, platform names MUST be written as `Linux`,
  `Mac`, and `Windows`, and the `macOS` styling MUST NOT be used.
- In code, each language MAY have its own convention for platform names, and
  that convention MUST take precedence over the previous rule of Platform names,
  only within the code of that language.

## Related

- [README.md](../README.md): What Kraf is, its vision, and the index of the
  repository's documents.
- [REPOSITORY_CONVENTIONS.md](./REPOSITORY_CONVENTIONS.md): Repository-wide
  conventions: normative language, document graph, links, and language neutrality.
- [ARCHITECTURE.md](./ARCHITECTURE.md): Cross-domain map of Kraf's system
  context, modules, and how they depend on each other. Needed when the task
  touches the structure of the system, such as modules, ports and adapters, or
  the dependencies between them.
- [CODE_CONVENTIONS.md](./CODE_CONVENTIONS.md): Non-negotiable code rules for
  the Kraf codebase — rules you amend, not break. Needed when the task creates,
  edits, or reviews code or tests.
- [SPEC_CONVENTIONS.md](./SPEC_CONVENTIONS.md): Conventions for documenting
  feature intent, requirements, and acceptance scenarios. Needed when the task
  creates, edits, or reviews a module README, a spec, an intent, an acceptance,
  or an ADR.
