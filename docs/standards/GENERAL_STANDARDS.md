---
description: General standards for this repository, document layout and metadata, normative language, the document graph, links, navigation, and language neutrality.
tags: [standards, repository-standards, document-layout, document-metadata, normative-language, document-graph, links, navigation, language-neutrality]
type: ["standard"]
scope: ["**/*", "type:standard", "!./.github/**/*"]
language: en
---

# General Standards

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

This document defines general repository-wide standards for document layout and
metadata, normative language, the document graph and links, and language
neutrality.

### Terms

- **Document**: any Markdown (`.md`) file in the repository.

## Document Structure

### Templates

#### Terms

- **Part**: a line, a heading, or a field of a template.

#### Rules

- A document that a rule requires to follow a template MUST contain each part of
  the template, in its order. Text outside `<...>` MUST be kept as written,
  except line wrapping. Each `<...>` placeholder MUST be replaced with content.
  A placeholder marked OPTIONAL MAY be omitted, unless another rule requires its
  content. A part not in the template MUST NOT be added, unless the template
  shows `...` in its place. Inside a part, `...` repeats the item before it,
  zero or more times.

### Document Layout

```yaml
scope: ["**/*.md", "!type:root"]
```

```md
---
<metadata yaml>
---

# <title>

<normative-language note, OPTIONAL>

<standard note, OPTIONAL>

<other notes, OPTIONAL>

## Overview

<overview text>

<specific section>

<other specific sections, OPTIONAL>

## Links

<links>
```

#### Rules

- Every document MUST follow the [template](#document-layout) above.
- Each note after the first MUST be preceded by a `</br>` line, with a blank
  line before and after it.

### Document Metadata

```yaml
scope: ["**/*.md"]
```

#### Document Metadata - Template

```md
---
description: <one line description>
tags: [<tag>, ...]
type: [<type>, ...]
scope: [<type:name | glob>, ...]
language: <language-code>
---
```

#### Terms

- **`type`**: one or more values that state what the document is.
- **`scope`**: the files that the document applies to, given as types or file
  patterns.
- **Section metadata**: a YAML block placed directly below a section heading
  that declares metadata for that section only.
- **Code file**: a file that is inferred, from its extension, name, or content,
  to hold program code, or the configuration of a tool used for coding, such as
  an editor, version control, a linter, a formatter, a package manager, a test
  tool, a container, or CI (e.g. `lib.rs`, `Cargo.toml`, `Dockerfile`, or
  `.gitignore`).

#### Rules

- Every document MUST start with metadata that follows the
  [template](#document-metadata---template) above.
- The metadata MUST be a valid YAML block.
- Each item of `scope` MUST be a type written as `type:<name>`, or a file
  pattern.
- An item written as `type:<name>` MUST match every document whose `type` field
  lists `<name>` among its values, even when it lists other values too.
- An item written as `type:code` MUST also match every code file, even when it
  has no metadata.
- An item that is a file pattern MUST be read as defined in
  [File patterns](#file-patterns).
- An item that starts with `!`, followed by a type or a file pattern, MUST
  exclude the files it matches from the files matched by the other items of the
  same `scope`.
- Section metadata MAY be declared in any section of any document.
- A section without section metadata MUST inherit the metadata of its nearest
  ancestor section that declares section metadata or, when none does, of its
  document.
- A `scope` in section metadata MUST be read as defined for a document and MUST
  apply only to that section.

### File Patterns

#### Rules

- A file pattern MUST be read relative to the repository root.
- A leading `./` MUST denote the repository root and MUST NOT change the meaning
  of the pattern.
- `*` MUST match any characters except `/`.
- `**` MUST match any number of directories, including none.

## Language Standards

### Normative Language

This standard defines how any document words a requirement, with the keywords of
[RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and
[RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).

Normative keywords express three levels of force:

| Level       | Use when the sentence...                      | Normative keywords                               |
| ----------- | --------------------------------------------- | ------------------------------------------------ |
| Required    | states a requirement or prohibition           | MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT       |
| Recommended | recommends or discourages an action           | SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED |
| Optional    | grants permission or makes an action optional | MAY, OPTIONAL                                    |

#### Terms

- **Standard document**: a document whose `type` lists `standard`. See
  [Standard documents](./DOC_STANDARDS.md#standard-documents).
- **[Normative keyword](../AGENTS.md#terms)**: defined in
  [AGENTS.md](../AGENTS.md#terms).
- **Normative-language note**: the note that states that the normative keywords
  follow RFC 2119 and RFC 8174. It follows the
  [note template](#normative-language-note---template).
- **[Explicit instruction](../AGENTS.md#terms)**: defined in
  [AGENTS.md](../AGENTS.md#terms).
- **[Implicit instruction](../AGENTS.md#terms)**: defined in
  [AGENTS.md](../AGENTS.md#terms).
- **Example**: an illustration of an idea that is explicitly marked as one. It
  is marked by any of these forms: a part of a sentence in parentheses that
  begins with `e.g.`; an item or paragraph under a heading titled `Example` or
  `Examples`; or a section titled `Example` or `Examples`. Other wording, such
  as "such as" or "for example", does not mark an example.

#### Rules

- A document carrying the [note](#normative-language-note) MUST express every
  normative requirement, permission, recommendation, or prohibition with
  normative keywords, and MUST NOT use alternative words or lowercase variants
  for normative meaning.
- An example MAY use normative keywords of any level, because it illustrates an
  idea and its text is not normative.
- A document MUST state its intent that the reader read or consult a link with a
  normative keyword directly before the verb (e.g. `MUST read [link](…)`). A
  link without one MUST be treated as a plain reference.
- A document that is not a standard document MAY use normative keywords to
  direct the agent that reads it (e.g. to use sudo). Such a direction MUST NOT
  be treated as a rule of this repository, nor reported as a violation of its
  rules, and MAY be reported as a warning, an info, or an alert.

### Normative-Language Note

```yaml
scope: ["**/*.md", "!type:root", "!type:index"]
```

#### Normative-Language Note - Template

```md
> [!NOTE]
> The keywords follow RFC 2119 and RFC 8174 across three levels of force:
>
> - **Required:** MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT
> - **Recommended:** SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED
> - **Optional:** MAY, OPTIONAL
>
> See [GENERAL_STANDARDS.md](./GENERAL_STANDARDS.md#normative-language).
```

#### Rules

- Every document matched by the `scope` of this section MUST carry the note,
  following the [template](#normative-language-note---template) above, as the
  first of the notes of the [layout](#document-layout), even when it uses no
  normative keyword.

### Standard Note

```yaml
scope: ["type:standard", "!type:root"]
```

#### Standard Note - Template

```md
> [!IMPORTANT]
> This is a standard document:
>
> - Works like a constitution
> - Rules and Instructions MUST be mandatory and normative
> - Rules and Instructions MUST NOT be ignored
>
> See [DOC_STANDARDS.md](./DOC_STANDARDS.md#standard-documents).
```

#### Terms

- **Standard note**: the note that marks a document as a standard document. It
  follows the [template](#standard-note---template) above.

#### Rules

- Every standard document MUST carry the note, following the
  [template](#standard-note---template) above, as the next note after the
  [normative-language note](#normative-language-note).

### Language Neutrality

#### Rules

- The normative text of a document MUST NOT use language-specific or
  technology-specific terms, file names, or extensions (e.g. `trait`, `lib.rs`),
  and MUST refer to roles instead (e.g. "interface" for a trait), unless the
  rule is about that language or technology.
- Data and configuration formats, such as YAML and JSON, MAY be named when a
  rule depends on them.
- In prose, docs, and diagrams, platform names MUST be written as `Linux`,
  `Mac`, and `Windows`, and the `macOS` styling MUST NOT be used.
- In code, each language MAY have its own standard for platform names, and that
  standard MUST take precedence over the preceding rule, only within the code of
  that language.

## Document Graph

### Link Rules

#### Rules

- Every reference to a specific file, or to a specific section of a document,
  including a section of the same document, MUST be a navigable link, not a bare
  name. A generic reference (e.g. "a module's `README.md`") MAY be written
  without a link.
- A reference to a specific part of a document that is not a section (e.g. "the
  template above", a table, or a list) is a reference to the section that
  contains it, and MUST be a link to that section.
- The name of a single thing that a specific section defines (e.g. "the
  interpretation clause", "the checklist", or "the standard note") is a
  reference to that section, and MUST be a link to it. A term that names any
  instance of what a section defines (e.g. "a finding" or "a review") MAY be
  written without a link.
- A file name that a standard defines, requires, or uses to identify a role
  (e.g. `README.md` as the name of the index of a folder, or the `AGENTS.md` and
  `README.md` files of the repository root, to which a rule applies) is a
  definition, not a reference to a specific file, and MUST be written without a
  link.
- A link MUST be written in the form `[text](target)`, where `target` is the
  path relative to the document that contains it, for a file in the repository,
  or the URL, for an HTTP link and `text` SHOULD be title of section or
  document.
- A link to an HTTP URL MUST point to the page's form for human viewing, not its
  raw-text form.
- A link to an HTTP URL MUST use a fixed URL, whose content does not change
  (e.g. a URL with a revision, a commit, a hash, or a date, or the permalink of
  an article), when the site provides one. When it does not, the link MUST use
  the URL of a copy of the page kept by a public, trusted web archiving service
  that keeps permanent copies.

### Link Navigation

#### Terms

- **Doc graph**: the documents and the links between them.
- **Index**: the entry document of a folder, which lists the documents in it.
- **Root index**: the index of the repository root.
- **Nearest index**: for a document that is not an index, the index of its own
  folder or, when that folder has none, of the nearest ancestor folder that has
  one. For an index, the index of the nearest ancestor folder that has one.
- **Tree-linked**: a document whose Links section links to its nearest index and
  whose nearest index links to it in its own Links section.

#### Rules

- The index of a folder, including the repository root, MUST be its `README.md`
  file, because code hosting platforms display that file by default as the entry
  of the folder.
- Every index MUST list `readme` and `index` in `type`.
- The repository root MUST have an index.
- Every document MUST have a Links section.
- Every document other than the root index MUST be tree-linked, even when those
  links already appear in the content, so that the reader can reach it from the
  root and go back.
- The links of a Links section MUST point to documents, not to sections of other
  documents.
- Each link of a Links section MUST be followed by the `description` from the
  metadata of its target, or its equivalent field, with the same text.
- A Links section MAY link to other documents. A link that appears in the
  content MAY be left out of Doc Graph
- The closing navigation links of a document MUST be in its Links section, not
  in a closing sentence.

## Links

- [README.md](../../README.md): What Kraf is and its vision, and the index of
  the repository's documents, modules and other files.
- [STANDARDS.md](./README.md): Index of the standards that govern this
  repository, to help people and agents navigate it on their own through the
  document graph.
- [DOC_STANDARDS.md](./DOC_STANDARDS.md): Standards for standard documents: what
  they are, their layout, precedence between rules, and how they word their
  rules.
