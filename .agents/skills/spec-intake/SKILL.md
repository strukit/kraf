---
name: spec-intake
description: Interview the user about a change request, settle every open decision against the docs graph, and publish the outcome as an issue. Use before creating or changing an intent, a specification, an acceptance, or an ADR.
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

# Spec intake

This skill turns a change request into an approved issue. It interviews the
user, settles every open decision against the docs graph, and publishes the
outcome. It does not create or edit an intent, a specification, an acceptance,
an ADR, or code: [spec-author](../spec-author/SKILL.md) does that after the user
approves the issue.

The skill applies to a request that adds or changes behavior, or that opens a
new module or feature. It does not apply to a meaning-preserving change, as
defined in [AGENTS.md](../../../AGENTS.md#classification).

## Rules

- The interview is mandatory work of this skill. A question that the agent
  raises under this skill cites this skill as its source, as
  [AGENTS.md](../../../AGENTS.md#rules-and-recommendations) requires of a raised
  concern.
- The agent MUST find facts itself and MUST ask the user only for decisions. A
  fact is anything that the docs graph, the code, or an available tool answers.
- The agent MUST NOT assume the answer to a decision.
- Each round of questions MUST contain every question whose prerequisites are
  settled, and MUST NOT contain a question that depends on an answer still open
  in the same round.
- Each question MUST carry the agent's suggested answer, worded so that a plain
  "yes" accepts it.
- The agent MUST wait for the user's answers before the next round and before
  the next step. When the user hands a decision back to the agent, the agent
  MUST state a recommendation and obtain explicit confirmation.

Format of a question:

```text
Q1 - <title>: <question, with the choices when there are any>
Suggested: <the agent's answer>
```

## Process

### 1. Discovery

- The agent MUST restate the request in one short paragraph.
- When the problem, the people affected, or the expected result is missing, the
  agent MUST ask for it in the first round.
- The agent MUST obtain the user's confirmation of the restatement before it
  explores.

### 2. Exploration

- The agent MUST navigate by links, as
  [Navigation](../../../AGENTS.md#navigation) defines: from the root
  [README.md](../../../README.md) to the module README, then to the intent, the
  specifications, the acceptances, and the ADRs that the request needs.
- The agent MUST establish, and list by identifier: the module and area that the
  request concerns; the requirements it touches, extends, or contradicts; the
  acceptance scenarios that verify them; the ADRs that constrain it; and the
  implementation status of those requirements.
- The agent MAY read code to establish a fact such as an implementation status.
- When the agent delegates exploration, the delegation rules of
  [AGENTS.md](../../../AGENTS.md#delegation) apply, including that the prompt
  names the revision and does not restate rules.
- The agent MUST present the findings to the user as a list of identifiers, each
  with one line.

### 3. Questions

The agent works the decisions of the request as a tree: every decision branches
into the decisions that depend on it. It asks in rounds, as the Rules define,
until no question remains.

The agent MUST cover each of these topics that the request touches:

- Scope and boundaries: what the change owns and what it leaves out.
- Triggers and preconditions: which pattern of EARS notation each behavior
  uses, as [SPEC_CONVENTIONS.md](../../../docs/SPEC_CONVENTIONS.md#specification-specmd)
  defines.
- Unwanted behavior: faults, failures, and errors.
- Conflicts: any existing requirement that the change contradicts or replaces.
- Platforms: any difference between Linux, Mac, and Windows.
- Verification: what an observer sees when the behavior holds.
- Intent: whether the change needs a new intent, an update to the current one,
  or neither.

The step ends when the user confirms that the shared understanding is complete.

### 4. Options

When a decision from step 3 depends on an architectural choice that no ADR
settles, the agent MUST present two or three approaches with their trade-offs
and its recommendation, and MUST ask the user to choose. Otherwise the agent
skips this step.

### 5. Issue

The agent MUST compose the issue with these sections, in this order:

- Problem, Intent, and Boundaries: the material of an intent, following its
  layout in [SPEC_CONVENTIONS.md](../../../docs/SPEC_CONVENTIONS.md#intent--intentmd).
  The section Intent MUST state whether the change needs a new intent, an update
  to the current one, or neither.
- Requirements: draft requirements in EARS notation. Each one MUST name the
  existing identifier it changes, or state that it is new.
- Acceptance: draft scenarios in Given, When, Then form, each naming the
  requirement it verifies.
- Decisions: the settled answers and the chosen approach.
- Affected documents: links to the documents the change touches.
- Open questions: empty, unless the user deferred a decision. A deferred
  decision MUST be named here.

Publishing reaches outside the working root and is hard to reverse, so the agent
MUST show the composed issue and MUST obtain the user's confirmation before it
publishes. It MUST publish to the GitHub Issues of this repository, through an
available tool, as
[Choosing a tool](../../../AGENTS.md#choosing-a-tool) defines. When no such tool
is available, the agent MUST inform the user and present the issue in its reply.

### 6. Hand-off

- The agent MUST give the user the reference of the published issue.
- The agent MUST NOT start [spec-author](../spec-author/SKILL.md) until the user
  states that the issue is approved.

## Related

- [README.md](../../../README.md): what Kraf is.
- [AGENTS.md](../../../AGENTS.md): entry point for agent instructions.
- [SPEC_CONVENTIONS.md](../../../docs/SPEC_CONVENTIONS.md): layout, identifiers,
  intent, specification, acceptance, ADR. Needed to draft the sections
  Requirements and Acceptance.
- [spec-author](../spec-author/SKILL.md): authors the documents from the
  approved issue.
