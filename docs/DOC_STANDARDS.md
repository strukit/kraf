---
title: DOC CONVENTIONS
description: "Conventions for convention-section layout, precedence, scope, and language."
tags: [conventions, documentation, convention-layout, precedence, scope, language]
type: ["convention"]
include: ["**/*.md"]
exclude: [".github/**"]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords. See
> [REPOSITORY_CONVENTIONS.md](./REPOSITORY_CONVENTIONS.md#normative-language).

> [!IMPORTANT]
> This is a convention document: its rules are mandatory and non-negotiable.
> Treat them like a constitution: a rule changes only through a pull request
> that edits this document and MUST NOT be bypassed by an ad-hoc exception in
> code. See [DOC_CONVENTIONS.md](./DOC_CONVENTIONS.md#convention-documents).

## Overview

This document defines conventions for the structure, precedence, scope, and
wording of convention documents. For document layout and metadata and
repository-wide conventions such as normative language, the document graph,
links, and language neutrality, see
[REPOSITORY_CONVENTIONS.md](./REPOSITORY_CONVENTIONS.md).

## Rule Conventions

### Convention section layout

```yaml
scope:
  include: ["convention"]
```

This convention defines the order and format of the content of a convention.

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
  [Normative Language](./REPOSITORY_CONVENTIONS.md#normative-language).
- Each item of Best practices MUST use only normative keywords of the
  Recommended level, as defined in
  [Normative Language](./REPOSITORY_CONVENTIONS.md#normative-language).

### Convention documents

This convention defines what a convention document is, how its rules bind and
change, and which rule prevails in a conflict.

```md
> [!IMPORTANT]
> This is a convention document: its rules are mandatory and non-negotiable.
> Treat them like a constitution: a rule changes only through a pull request
> that edits this document and MUST NOT be bypassed by an ad-hoc exception in
> code. See
> [DOC_CONVENTIONS.md](<relative-link-to-docs>/DOC_CONVENTIONS.md#convention-documents).
```

#### Terms

- **Convention document**: a document whose purpose is to define rules and that
  works like a constitution.
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
  in
  [Document Metadata](./REPOSITORY_CONVENTIONS.md#document-metadata).
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
  enumerate values, sources, or links, such as a Links section, MAY
  contain sentences without a normative keyword.
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

## Links

- [README.md](../README.md): What Kraf is, its vision, and the index of the
  repository's documents.
- [REPOSITORY_CONVENTIONS.md](./REPOSITORY_CONVENTIONS.md): Repository
  conventions for document layout and metadata, normative language, the
  document graph, links, navigation, and implementation-language neutrality.
