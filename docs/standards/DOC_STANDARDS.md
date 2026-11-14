---
description: Standards for standard documents: what they are, their layout, precedence between rules, and how they word their rules.
tags: [standards, documentation, standard-documents, precedence, rule-language]
type: ["standard"]
scope: ["type:standard", "!type:root"]
language: en
---

# Document Standards

> [!NOTE]
> The keywords follow RFC 2119 and RFC 8174 across three levels of force:
>
> - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
> - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
> - **Optional:** MAY, OPTIONAL
>
> See [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#normative-language).

</br>

> [!IMPORTANT]
> This is a standard document:
>
> - Works like a constitution
> - Rules and Instructions MUST be mandatory and normative
> - Rules and Instructions MUST NOT be ignored
>
> See [DOC_STANDARDS.md](./DOC_STANDARDS.md#standard-documents).

## Overview

This document defines standards for standard documents: what they are, how their
rules bind and change, which rule prevails in a conflict, and how they word
their rules. The layout and metadata of documents follow
[GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md), which this document applies and
does not replace.

[SPEC_STANDARDS.md](./SPEC_STANDARDS.md) defines the more specific standards for
feature intent, requirements, acceptance scenarios, and ADRs.

## Standard documents

This section defines how the rules of a
[standard document](./GENERAL_STANDARDS.md#normative-language) bind and change,
and which rule prevails in a conflict.

#### Rules

- The rules of a standard document MUST be followed by the codebase and agents
  within that document's scope.
- A rule MUST change only through a pull request that edits the standard
  document defining it.
- A pull request that changes a standard document MUST bring into compliance the
  content that the change makes noncompliant, unless its description lists what
  it leaves for a later pull request.
- Rules MUST be defined only in standard documents.

### Standard document layout

This section defines how a standard document groups its terms and rules.

#### Rules

- A section of a standard document, at any heading level, that defines rules
  MUST group them under a heading titled `Rules`.
- A section of a standard document, at any heading level, that defines terms
  MUST group them under a heading titled `Terms`, before its `Rules` heading.

### Precedence

This section defines which rule prevails when two rules conflict.

#### Rules

- A rule MAY override another rule only within its own scope, and only when it
  is defined in a standard document and cites the rule it overrides.
- Two rules that conflict without such an override MUST be reported as a defect.

## Rule language

This section defines how a standard document words its rules.

#### Terms

- **Descriptive sentence**: a sentence that states a fact or a behavior and
  expresses no normative force (e.g. "The repository is organized by feature.").
- **Unclassifiable sentence**: a sentence that does not make clear whether it is
  an instruction or a description (e.g. "The agent reads the file before
  committing.").

#### Rules

- In a standard document, each rule MUST be a list item, and each sentence of a
  list item MUST use a normative keyword. A reason or an example MAY qualify
  that sentence.
- Notes, templates, items of Terms and Examples, and lists that only enumerate
  values, sources, or links MAY contain sentences without a normative keyword.
- Prose outside list items MAY introduce a section or give an example, and MUST
  NOT state a rule.
- A definition MUST be written as `**Term**: meaning.`, before the rules that
  use the term.
- An unclassifiable sentence in a standard document MUST be rewritten as a rule
  with a normative keyword or as a descriptive sentence.

## Links

- [README.md](./README.md): Index of the standards that govern this repository,
  to help people and agents navigate it on their own through the document graph.
- [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md): General standards for this
  repository, document layout and metadata, normative language, the document
  graph, links, navigation, and language neutrality.
- [SPEC_STANDARDS.md](./SPEC_STANDARDS.md): Standards for documenting feature
  intent, requirements, and acceptance scenarios.
