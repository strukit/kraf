---
description: Standards for standard documents: what they are, their layout, precedence between rules, and how they word their rules.
tags: [standards, documentation, standard-documents, precedence, rule-language]
type: ["standard"]
scope: ["type:standard", "!type:root"]
---

> [!NOTE]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> See [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#normative-language).

</br>

> [!IMPORTANT]
> This is a standard document. It works like a constitution. Its rules MUST NOT
> be bypassed by ad-hoc exceptions. See
> [DOC_STANDARDS.md](#standard-documents).

## Overview

This document defines standards for standard documents: what they are, how
their rules bind and change, which rule prevails in a conflict, and how they
word their rules. The layout and metadata of documents follow
[GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md), which this document applies and
does not replace.

[SPEC_STANDARDS.md](./SPEC_STANDARDS.md) defines the more
specific standards for feature intent, requirements, acceptance scenarios, and
ADRs.

## Standard documents

This section defines what a standard document is, how its rules bind and
change, and which rule prevails in a conflict.

#### Terms

- **Standard document**: a document whose purpose is to define rules and that
  works like a constitution.

#### Rules

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

### Standard document layout

This section defines how a standard document groups its terms and rules.

#### Rules

- A section of a standard document, at any heading level, that defines rules
  MUST group them under a heading titled `Rules`.
- A section of a standard document, at any heading level, that defines terms
  MUST group them under a heading titled `Terms`, before its `Rules` heading.
- A section MAY omit the `Terms` heading when it defines no term.

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

- [README.md](./README.md): Index of the standards that govern this repository, to help people and agents navigate it on their own through the document graph.
- [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md): General standards for this
  repository, document layout and metadata, normative language, the document
  graph, links, navigation, and language neutrality.
- [SPEC_STANDARDS.md](./SPEC_STANDARDS.md): Standards for documenting feature
  intent, requirements, and acceptance scenarios.
