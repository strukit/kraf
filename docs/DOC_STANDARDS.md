---
description: Conventions for document layout, metadata, precedence, scope, and normative language.
tags: [conventions, documentation, document-layout, document-metadata, precedence, scope, normative-language]
type: ["convention"]
scope: ["**/*.md"]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords. See
> [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#normative-language).

> [!IMPORTANT]
> This is a convention document: its rules are mandatory and non-negotiable.
> Treat them like a constitution. A rule MUST change only through a pull request
> that edits the convention document defining it and MUST NOT be bypassed by an
> ad-hoc exception. See
> [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#convention-documents).

## Overview

This document defines conventions for documents: their layout and metadata, the
precedence and scope of conventions, and the language used to express them. It
applies the repository-wide rules in
[GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md) and does not replace them.

[SPEC_CONVENTIONS.md](./SPEC_CONVENTIONS.md) defines the more specific
conventions for feature intent, requirements, acceptance scenarios, and ADRs.

## Document layout

This convention defines the order of the parts of a general document.

```md
---
<metadata yaml>
---

# Title

<notes, if any>

## Overview

<overview text>

## <Document-specific section title>

<section content>

## <Other document-specific section titles, if any>

<section content>

## Links

<links>
```

#### Rules

- Every document MUST follow the order of the template above.
- A document MAY omit notes when none apply.
- Every document MUST have a `Links` section, as required by
  [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#link-navigation-document-graph).

## Document metadata

This convention defines the metadata that every document declares so that an
agent can index it and decide whether a task needs it.

#### Terms

- **`type`**: what the document is.
- **`scope`**: the files whose tasks need the document, given as types or file
  patterns.

#### Rules

- Every document MUST start with the metadata of the template in
  [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#document-metadata---template),
  with every required field present.
- The metadata MUST be a valid YAML block.
- `type` MUST list one or more types that state what the document is.
- Each item of `scope` MUST be a type or a file pattern.
- An item of `scope` that is a type MUST match every document whose `type`
  contains that type.
- A file-pattern item of `scope` MUST be read as defined by
  [Convention scope](#convention-scope).

## Convention documents

This convention defines what a convention document is, how its rules bind and
change, and which rule prevails in a conflict.

#### Terms

- **Convention document**: a document whose purpose is to define rules and that
  works like a constitution.
- **Convention**: a section of a convention document that groups related rules,
  with its own scope, terms, and rules.

#### Rules

- A convention document MUST carry the convention note near the top, after the
  normative-language note, separated from it only by a blank line.
- The rules of a convention document MUST be followed by the codebase and agents
  within that document's scope.
- Ad-hoc exceptions MUST NOT be introduced.
- A rule MUST change only through a pull request that edits the convention
  document defining it.
- A pull request that changes a convention document MUST bring into compliance
  the content that the change makes noncompliant, unless its description lists
  what it leaves for a later pull request.
- Rules MUST be defined only in convention documents. A document that is not a
  convention document MUST NOT introduce rules and MAY only reference or apply
  existing ones.

### Convention precedence

This convention defines which rule prevails when two rules conflict.

#### Rules

- A rule explicitly scoped to a document type or context MUST take precedence
  over a more general rule wherever they conflict, only within that scope.
- Non-conflicting general rules MUST remain in effect.
- An override MUST be defined in a convention document and MUST cite the
  convention of the rule it overrides.
- [SPEC_CONVENTIONS.md](./SPEC_CONVENTIONS.md) MUST be treated as the more
  specific convention for specification document types and MAY override this
  document only within that scope; non-conflicting rules in this document MUST
  remain in effect.

## Convention scope

This convention defines how a convention declares the files its rules apply to.

```yaml
scope:
  include: ["<type | glob>", ...] # optional
  exclude: ["<type | glob>", ...] # optional
```

#### Terms

- **Convention scope**: the files that the rules of a convention apply to.
- **Scope block**: the template above as a YAML code block immediately below a
  convention heading.

#### Rules

- A scope block MUST be valid YAML with the `scope` key and, under it, only
  `include` and `exclude`, each with a list of quoted items.
- A scope block MUST have `include`, `exclude`, or both.
- Each item of `include` and `exclude` MUST be read as defined in
  [Document metadata](./GENERAL_STANDARDS.md#document-metadata).
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
- The rules of a convention MUST be applied only to files in its convention
  scope, and that scope MUST NOT be derived from another source, such as
  `.gitignore`.
- When neither a convention nor a convention that contains it has a scope block,
  its rules MUST be applied as their text scopes them.

## Convention language

This convention defines how a convention document words its rules.

#### Rules

- In a convention document, each rule MUST be a list item, and each sentence of
  a list item MUST use a normative keyword. A reason or an example MAY qualify
  that sentence.
- Notes, templates, items of Terms and Examples, and lists that only enumerate
  values, sources, or links MAY contain sentences without a normative keyword.
- Prose outside list items MAY introduce a convention, define a term, or give an
  example, and MUST NOT state a rule.
- A definition MUST be written as `**Term**: meaning.`, before the rules that
  use the term.

## Links

- [README.md](../README.md): What Kraf is, its vision, and the index of the
  repository's documents.
- [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md): General standards for
  repository, document layout and metadata, normative language, the document
  graph, links, and language neutrality.
- [SPEC_CONVENTIONS.md](./SPEC_CONVENTIONS.md): Conventions for documenting
  feature intent, requirements, and acceptance scenarios.
