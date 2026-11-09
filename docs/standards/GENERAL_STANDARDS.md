---
description: General standards for this repository, document layout and metadata, normative language, the document graph, links, navigation, and language neutrality.
tags: [standards, repository-standards, document-layout, document-metadata, normative-language, document-graph, links, navigation, language-neutrality]
type: ["standard"]
scope: ["**/*", "!./.github/*", "!./AGENTS.md", "!./README.md", "!./INDEX.md"]
---

# General Standards

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> See [GENERAL_STANDARDS.md](#normative-language).

> [!IMPORTANT]
> This is a standard document. It works like a constitution. Its rules MUST NOT
> be bypassed by ad-hoc exceptions. See
> [DOC_STANDARDS.md](./DOC_STANDARDS.md#standard-documents).

## Overview

This document defines general-wide standards for repository, document layout and
metadata, normative language, the document graph and links, and language
neutrality.

## Document Structure

This standard defines the layout and metadata of general documents.

### Document Layout

This standard defines the order of the parts of a general document.

```md
---
<metadata yaml>
---

# <title>

<notes, if any>

## Overview

<overview text>

## <specific section title>

<section content>

## <other specific section titles, if any>

<section content>

## Links

<links>
```

#### Rules

- Every document MUST follow the order of the template above.
- A document MAY omit other notes when none apply.

### Document Metadata

```yaml
scope: ["**/*.md"]
```

This standard defines the metadata that every document declares at its top, so
that an agent can index it and decide whether the task needs it.

#### Document Metadata - Template

```md
---
description: <one line description>
tags: [<tag>, ...]
type: [<type>, ...]
scope: [<type | glob>, ...]
---
```

#### Terms

- **`type`**: what the document is.
- **`scope`**: the files whose tasks need the document, given as types or file
  patterns.

#### Rules

- Every document MUST start with the metadata of the
  [template](#document-metadata---template) above, with every field that the
  template does not mark as optional.
- The metadata MUST be a valid YAML block.
- `type` MUST list one or more types that state what the document is.
- Each item of `scope` MUST be a type written as `type:<name>`, or a file
  pattern.
- An item written as `type:<name>` MUST match every document whose `type` field
  contains `<name>`.
- An item that is a file pattern MUST be read as defined in
  [File patterns](#file-patterns).
- A file that has its own schema (e.g. `SKILL.md` or `AGENTS.md`) MUST be
  outside this standard, even when it is a standard document.

### File Patterns

This standard defines how a file pattern in `scope` selects files.

#### Rules

- A file pattern MUST be read relative to the repository root.
- A leading `./` MUST denote the repository root and MUST NOT change the meaning
  of the pattern.
- `*` MUST match any characters except `/`.
- `**` MUST match any number of directories, including none.
- A file pattern that starts with `!` MUST exclude the files it matches from the
  files matched by the other items of the same `scope`.

## Language Standards

This standard defines how documents express normative requirements and remain
independent of implementation languages.

### Normative Language

This standard defines how any document words a requirement, with the key words
of [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and
[RFC 8174](https://www.rfc-editor.org/rfc/rfc8174). A normative keyword gives a
sentence its force; it does not make the document a standard document.

Normative keywords express three levels of force:

| Level       | Use when the sentence...                      | Normative keywords                               |
| ----------- | --------------------------------------------- | ------------------------------------------------ |
| Required    | states a requirement or prohibition           | MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT       |
| Recommended | recommends or discourages an action           | SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED |
| Optional    | grants permission or makes an action optional | MAY, OPTIONAL                                    |

The standard note and its template provide a consistent reference for
interpreting normative requirements.

#### Normative Language - Note Template

```md
> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
> See
> [GENERAL_STANDARDS.md](<relative-link-to-docs>/GENERAL_STANDARDS.md#normative-language).
```

#### Terms

- **Standard document**: a document whose purpose is to define rules and that
  works like a constitution. See
  [Standard documents](./DOC_STANDARDS.md#standard-documents).
- **Normative keyword**: one of the key words MUST, MUST NOT, REQUIRED, SHALL,
  SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED, MAY, and
  OPTIONAL, in uppercase, with the meaning that
  [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and
  [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174) give them. The
  [note template](#normative-language---note-template) lists the same key words.
- **Explicit instruction**: a sentence that expresses normative force with a
  normative keyword (e.g. "Reviews MUST report each deviation with its
  source.").
- **Implicit instruction**: a sentence that expresses normative force without a
  normative keyword (e.g. "Read the standard before you change the code.").
- **Descriptive sentence**: a sentence that states a fact or a behavior and
  expresses no normative force (e.g. "The repository is organized by feature.").
- **Unclassifiable sentence**: a sentence that does not make clear whether it is
  an instruction or a description (e.g. "The agent reads the file before
  committing.").
- **Example**: an illustration of an idea that is explicitly marked as one. It
  is marked by any of these forms: a part of a sentence in parentheses that
  begins with `e.g.`; an item or paragraph under a heading titled `Example` or
  `Examples`; or a section titled `Example` or `Examples`. Other wording, such
  as "such as" or "for example", does not mark an example.

#### Rules

- Every document MUST carry the note of the template above, exactly as written.
- A document MAY carry the note without containing normative requirements or
  using every normative keyword, and a purely descriptive document MUST carry it
  too.
- A family of documents MAY define an equivalent note that lists only the
  normative keywords it uses, if it cites RFC 2119 and RFC 8174 near the top.
- A document carrying the note MUST express every normative requirement,
  permission, recommendation, or prohibition with normative keywords, and MUST
  NOT use alternative words or lowercase variants for normative meaning.
- An example MAY use normative keywords of any level, because it illustrates an
  idea and its text is not normative.
- When a document intends the reader to read or consult something, it MUST state
  that intent with a normative keyword directly before the verb (e.g. `MUST read
  [link](…)`, `SHOULD see [link](…)`, or `MAY read [link](…)`). A link that only
  points to an existing document, without that intent (e.g. `see [link](…)`), is
  written without a normative keyword.
- A link without a normative keyword before its verb MUST be treated as a plain
  reference, with no normative force.
- A document that is not a standard document MAY use normative keywords to fix
  the behavior of the agent that reads it (e.g. to use sudo). Using normative
  keywords in such a document does not make it a standard document. Such a
  direction MUST NOT be treated as a rule of this repository, and MUST NOT be
  reported as a violation of the rules of this repository, but MAY be reported
  as a warning, an info, or an alert, when needed or when requested.

### Language Neutrality

This standard defines how documents stay independent of the implementation
language.

#### Rules

- The normative text of a document MUST NOT use language-specific or
  technology-specific terms, file names, and extensions (e.g. `trait`,
  `lib.rs`), unless the document is about a specific language or technology.
- That text MUST refer to roles instead, unless the document is about a specific
  language or technology: the `lib`, `cli`, and `mcp` entrypoints without an
  extension (e.g. "interface" for a trait).
- Within a document's scope, terms for languages, platforms, technologies,
  tools, and files MAY be used when they define the subject of that document. This
  permission MUST override [Language Neutrality](#language-neutrality) only
  within that scope.
- An example MAY use language-specific terms, code symbols, file names, and
  extensions to illustrate a language-neutral rule.
- Data and configuration formats, such as YAML and JSON, MAY be named when a
  rule depends on them.

## Link Rules and Link Navigation (Document Graph)

This standard defines how links are written and how they connect documents into
a navigable graph.

### Link Rules

This standard defines how links are written, so that humans, agents, platforms,
and IDEs can follow them.

#### Rules

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

### Link Navigation (Document Graph)

This standard defines how documents link to each other: a tree based on folders
makes every document reachable, and other links stay free on top of it. How to
write each link is defined in [Link Rules](#link-rules).

#### Terms

- **Doc graph**: the documents and the links between them.
- **Index**: the entry document of a folder, which lists the documents in it.
- **Root**: the index of the repository root.
- **Nearest index**: for a document that is not an index, the index of its own
  folder or, when that folder has none, of the nearest ancestor folder that has
  one. For an index, the index of the nearest ancestor folder that has one.
- **Tree-linked**: a document whose Links section links to its nearest index and
  whose nearest index links to it in its own Links section.

#### Rules

- The index of a folder other than the repository root MUST be its `README.md`
  file, because code hosting platforms display that file by default as the entry
  of the folder.
- The index of the repository root MUST be its `INDEX.md` file. No other file
  MUST be named `INDEX.md`.
- A folder without a `README.md` file MUST use the index of its nearest ancestor
  folder.
- The repository root MUST have an index.
- Every document MUST have a Links section.
- Every document other than the root MUST be tree-linked, even when those links
  already appear in the content, so that the reader can reach it from the root
  and go back.
- The links of a Links section MUST point to documents, not to sections of other
  documents.
- Each link of a Links section MUST be followed by the `description` from the
  metadata of its target, or its equivalent field, with the same text.
- A Links section MAY link to other documents. A link that appears in the
  content MAY be left out of it.
- The closing navigation links of a document MUST be in its Links section, not
  in a closing sentence.
- A set of documents MAY define its own graph of links. When it does, the graph
  MUST be documented in the index of the set and MUST link to the nearest index,
  so that the reader can leave the set.

## Links

- [INDEX.md](../../INDEX.md): Index of the repository's documents, modules and other files.
- [README.md](./README.md): Index of the standards that govern this repository.
- [DOC_STANDARDS.md](./DOC_STANDARDS.md): Standards for document layout,
  metadata, precedence, scope, and normative language.
