---
name: spec-author
description: Create or update an intent, specifications, acceptances, and ADRs from an approved issue. Use after the user approves an issue produced by spec-intake.
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174, as defined in
> [GENERAL_CONVENTIONS.md](../../../docs/GENERAL_CONVENTIONS.md#normative-language).
> This document MUST NOT use alternative words or lowercase variants as
> normative keywords.

---

> [!IMPORTANT]
> This is a convention document, as defined in
> [GENERAL_CONVENTIONS.md](../../../docs/GENERAL_CONVENTIONS.md#convention-documents):
> its rules are mandatory and non-negotiable. Treat them like a constitution: a
> rule changes only through a pull request that edits this document and MUST NOT
> be bypassed by an ad-hoc exception in code.

# Spec author

This skill turns an approved issue into documents: an intent, specifications,
acceptances, and ADRs. The issue comes from
[spec-intake](../spec-intake/SKILL.md). The skill does not change code or tests.

## Rules

- The input is an issue that the user states is approved. The agent MUST NOT
  author from a conversation or from an issue the user has not approved. It MUST
  send the user back to [spec-intake](../spec-intake/SKILL.md) instead.
- The agent MUST author only content that the issue states. It MUST NOT add a
  principle, a requirement, a scenario, or a decision that the issue does not
  contain.
- When the issue lacks content that a document needs, or lists an open question
  that the document depends on, the agent MUST stop, name the gap, and ask the
  user. It MUST NOT author what depends on the gap.
- The layout, identifiers, statuses, notation, and links of every document type
  are those of [SPEC_CONVENTIONS.md](../../../docs/SPEC_CONVENTIONS.md). This
  skill MUST NOT restate them, and the agent MUST follow them.

## Process

### 1. Read

- The agent MUST read the whole issue, including its comments.
- The agent MUST read [SPEC_CONVENTIONS.md](../../../docs/SPEC_CONVENTIONS.md)
  and, by following links, the documents listed under Affected documents in the
  issue.

### 2. Intent

- When the issue states that the change needs a new intent or an update to the
  current one, the agent MUST derive it from the sections Problem, Intent, and
  Boundaries of the issue.
- The agent MUST write a principle in an intent only when the issue states it.

### 3. Specifications

- For each requirement the issue drafts, the agent MUST write a requirement with
  the next free identifier of its area, or change the requirement the issue
  names.
- A new requirement MUST have Doc status `Draft` and Implementation status
  `Not Implemented`. A changed requirement MUST have Doc status `Draft` and
  Implementation status `Unknown`, unless the issue states its status.
- When the issue changes or removes an existing requirement, the agent MUST
  compare the scope of the old and the new wording as the
  [Review](../../../docs/SPEC_CONVENTIONS.md#review) section defines, and MUST
  report each case that only one wording covers.

### 4. Acceptances

- For each scenario the issue sketches, the agent MUST write an acceptance
  scenario with Doc status `Draft` and Verification status `Unknown`.
- Every requirement the agent writes or changes MUST have at least one
  acceptance scenario. When the issue sketches none for a requirement, the agent
  MUST stop and ask the user.

### 5. ADRs

- For each architectural decision under Decisions in the issue, the agent MUST
  write an ADR with Doc status `Proposed`.
- The agent MUST NOT edit the content of an ADR whose Doc status is `Accepted`.
  A decision that replaces it MUST be a new ADR, as
  [SPEC_CONVENTIONS.md](../../../docs/SPEC_CONVENTIONS.md#adr--adr-md)
  defines.

### 6. Graph

The agent MUST keep the docs graph complete, as
[Docs graph](../../../docs/SPEC_CONVENTIONS.md#docs-graph) defines: each new
document is linked from the document that introduces it and links back to it.

### 7. Verify

- The agent MUST verify that every document it wrote follows the
  normative-language rules, as
  [AGENTS.md](../../../AGENTS.md#normative-language) requires.
- The agent MUST verify that identifiers are unique and that every link resolves
  in both directions.
- Once it finishes writing, the agent MUST run the declared tools, as
  [Writing content](../../../AGENTS.md#writing-content) defines.

### 8. Report

The agent MUST report to the user: the documents it created or changed; the
identifiers it created or changed; each gap it stopped on; and each case from
the scope comparison of step 3. The agent MUST include the issue reference in
the pull request description it writes, if the user asks for a pull request.

## Related

- [README.md](../../../README.md): what Kraf is.
- [AGENTS.md](../../../AGENTS.md): entry point for agent instructions.
- [SPEC_CONVENTIONS.md](../../../docs/SPEC_CONVENTIONS.md): layout, identifiers,
  intent, specification, acceptance, ADR.
- [spec-intake](../spec-intake/SKILL.md): interviews the user and publishes the
  issue that this skill consumes.
