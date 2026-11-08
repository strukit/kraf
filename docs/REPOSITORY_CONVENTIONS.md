---
title: REPOSITORY CONVENTIONS
description: "Repository structure, normative language, document graph, and language neutrality rules."
tags: [conventions, repository, documentation]
type: ["convention"]
include: ["**/*"]
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
> code. See
> [REPOSITORY_CONVENTIONS.md](./REPOSITORY_CONVENTIONS.md#convention-documents).

## Overview

This document defines conventions that apply across the repository: how to write
normative requirements with RFC 2119 keywords, how documents form a navigable
graph, how to keep documents independent of implementation languages, and how the
workspace is organized.

## Normative language

```yaml
scope:
  exclude: ["README.md"]
```

This convention defines how any document words a requirement, with the key words
of [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and
[RFC 8174](https://www.rfc-editor.org/rfc/rfc8174). A normative keyword gives a
sentence its force; it does not make the document a convention document.

```md
> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords. See
> [REPOSITORY_CONVENTIONS.md](<relative-link-to-docs>/REPOSITORY_CONVENTIONS.md#normative-language).
```

### Terms

- **Normative keyword**: one of the key words that the note of the template
  above lists, in uppercase.
- **Level**: the force of a normative keyword, with the values of the table
  below. The keyword alone gives the level of the sentence that uses it; no part
  of a document declares a level.

| Level       | Normative keywords                               |
| ----------- | ------------------------------------------------ |
| Required    | MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT       |
| Recommended | SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED |
| Optional    | MAY, OPTIONAL                                    |

### Rules

- Every document MUST carry the note of the template above near the top.
- A document MAY carry the note without containing normative requirements or
  using every normative keyword, and a purely descriptive document MUST carry it
  too.
- A family of documents MAY define an equivalent note that lists only the
  normative keywords it uses, if it cites RFC 2119 and RFC 8174 near the top.
- A document carrying the note MUST express every normative requirement,
  permission, recommendation, or prohibition with normative keywords, and MUST
  NOT use alternative words or lowercase variants for normative meaning.

### Best practices

- To check the note, a review SHOULD compare it with the template, ignoring line
  breaks and the path of the link.

## Doc graph

This convention defines how documents link to each other: a tree based on
folders makes every document reachable, and other links stay free on top of it.
How to write each link is defined in [Doc links](#doc-links).

### Terms

- **Doc graph**: the documents and the links between them.
- **Index**: the entry document of a folder, which lists the documents in it.
- **Root**: the index of the repository root.
- **Parent**: for a document that is not an index, the index of its own folder
  or, when that folder has none, of the nearest ancestor folder that has one.
  For an index, the index of the nearest ancestor folder that has one.
- **Tree-linked**: a document whose Related section links to its parent and
  whose parent links to it in its own Related section.

### Rules

- The index of a folder MUST be its `README.md` file, because code hosting
  platforms display that file by default as the entry of the folder.
- The repository root MUST have an index.
- Every document MUST have a Related section.
- Every document other than the root MUST be tree-linked, even when those links
  already appear in the content, so that the reader can reach it from the root
  and go back.
- The links of a Related section MUST point to documents, not to sections of
  other documents.
- Each link of a Related section MUST be followed by the `description` from the
  metadata of its target, or its equivalent field, with the same text.
- A Related section MAY link to other documents. A link that appears in the
  content MAY be left out of it.
- The closing navigation links of a document MUST be in its Related section, not
  in a closing sentence.

### Best practices

- To check the graph, a review SHOULD confirm, for each document that the
  change:
  - adds: that it is tree-linked;
  - removes: that no document links to it;
  - moves: the checks of "adds" at the new path and of "removes" at the old
    path, and that the links of the document itself still resolve.

  It SHOULD report each failure.

## Doc links

This convention defines how links are written, so that humans, agents,
platforms, and IDEs can follow them.

### Rules

- Every reference to a specific file MUST be a navigable link, not a bare name.
  A generic reference, such as "a module's `README.md`", MAY be written without
  a link.
- A link MUST be written in the form `[text](target)`.
- A link to a file in the repository MUST use the path relative to the document
  that contains it.
- A link to an HTTP URL MUST point to the page's form for human viewing, not its
  raw-text form.
- A link to an HTTP URL MUST use a fixed URL, whose content does not change (for
  example, a URL with a revision, a commit, a hash, or a date, or the permalink
  of an article), when the site provides one. When it does not, the link MUST
  use the URL of a copy of the page kept by a public, trusted web archiving
  service that keeps permanent copies.

## Doc language neutrality

This convention defines how documents stay independent of the implementation
language.

### Rules

- The normative text of a document MUST NOT use language-specific terms, such as
  "trait", or language-specific file names and extensions, such as `lib.rs`,
  unless the document is about a specific language.
- That text MUST refer to roles instead, unless the document is about a specific
  language: "interface" for a trait, and the `lib`, `cli`, and `mcp` entrypoints
  without an extension.
- Clearly identified examples MAY use language-specific terms, code symbols,
  file names, and extensions to illustrate a language-neutral rule.
- An example MUST NOT introduce a language-specific requirement.
- Data and configuration formats, such as YAML and JSON, MAY be named when a
  rule depends on them.

## Related

- [README.md](../README.md): What Kraf is, its vision, and the index of the
  repository's documents.
- [GENERAL_CONVENTIONS.md](./GENERAL_CONVENTIONS.md): Convention document format,
  rule language, and precedence rules.
