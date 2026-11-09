---
description: Standards for document layout, metadata, precedence, scope, and normative language.
tags: [standards, documentation, document-layout, document-metadata, precedence, scope, normative-language]
type: ["standard"]
scope: ["type:standard"]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> See [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#normative-language).

> [!IMPORTANT]
> This is a standard document. It works like a constitution. Its rules are
> mandatory and non-negotiable. A rule MUST change only through a pull request
> that edits the standard document defining it and MUST NOT be bypassed by an
> ad-hoc exception. See [Standard documents](#standard-documents).

## Overview

This document defines standards for documents: their layout and metadata, the
precedence and scope of rules, and the language used to express them. It
applies the repository-wide rules in
[GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md) and does not replace them.

[SPEC_STANDARDS.md](./SPEC_STANDARDS.md) defines the more
specific standards for feature intent, requirements, acceptance scenarios, and
ADRs.

## Document layout

This section defines the order of the parts of a general document.

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

This section defines the metadata that every document declares so that an
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
- Each item of `scope` MUST be a type written as `type:<name>`, or a file
  pattern.
- An item of `scope` written as `type:<name>` MUST match every document whose
  `type` field contains `<name>`.
- A file-pattern item of `scope` MUST be read as defined by
  [File patterns](./GENERAL_STANDARDS.md#file-patterns).

## Standard documents

This section defines what a standard document is, how its rules bind and
change, and which rule prevails in a conflict.

#### Terms

- **Standard document**: a document whose purpose is to define rules and that
  works like a constitution.

#### Rules

- A standard document MUST carry the standard note near the top, after the
  normative-language note, separated from it only by a blank line.
- The rules of a standard document MUST be followed by the codebase and agents
  within that document's scope.
- Ad-hoc exceptions MUST NOT be introduced.
- A rule MUST change only through a pull request that edits the standard
  document defining it.
- A pull request that changes a standard document MUST bring into compliance the
  content that the change makes noncompliant, unless its description lists what
  it leaves for a later pull request.
- Rules MUST be defined only in standard documents. A document that is not a
  standard document MUST NOT introduce rules and MAY only reference or apply
  existing ones.

### Precedence

This section defines which rule prevails when two rules conflict.

#### Rules

- A rule explicitly scoped to a document type or context MUST take precedence
  over a more general rule wherever they conflict, only within that scope.
- Non-conflicting general rules MUST remain in effect.
- An override MUST be defined in a standard document and MUST cite the
  rule it overrides.
- [SPEC_STANDARDS.md](./SPEC_STANDARDS.md) MUST be treated as the more specific
  standard for specification document types. It MAY override this document only
  within that scope, and only when it states the override explicitly.
  Non-conflicting rules in this document MUST remain in effect.

## Rule language

This section defines how a standard document words its rules.

#### Rules

- In a standard document, each rule MUST be a list item, and each sentence of a
  list item MUST use a normative keyword. A reason or an example MAY qualify
  that sentence.
- Notes, templates, items of Terms and Examples, and lists that only enumerate
  values, sources, or links MAY contain sentences without a normative keyword.
- Prose outside list items MAY introduce a section, define a term, or give an
  example, and MUST NOT state a rule.
- A definition MUST be written as `**Term**: meaning.`, before the rules that
  use the term.

## Links

- [README.md](./README.md): Index of the standards that govern this repository.
- [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md): General standards for this
  repository, document layout and metadata, normative language, the document
  graph, links, navigation, and language neutrality.
- [SPEC_STANDARDS.md](./SPEC_STANDARDS.md): Standards for documenting feature
  intent, requirements, and acceptance scenarios.
