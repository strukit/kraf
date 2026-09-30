---
title: Example intent
summary: Illustrative intent statement used to demonstrate the specification format.
tags: [specification, example, intent]
---

> [!IMPORTANT]
> The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT,
> RECOMMENDED, MAY, and OPTIONAL follow RFC 2119 and RFC 8174.
>
> This document MUST use these terms, in uppercase, when expressing normative
> requirements, permissions, recommendations, or prohibitions. It MUST NOT use
> alternative words or lowercase variants as normative keywords.

## Problem

Configuration read directly from process state is difficult to review,
reproduce, or audit.

## Intent

The feature is intended to make process configuration explicit rather than
inheriting undeclared host state.

## Boundaries

This example does not define how configuration is stored, transmitted, or
loaded — only that unknown keys are rejected.

## Documentation

- [Readme](./README.md): documentation index and example overview.
- [Specification conventions](./SPEC_CONVENTIONS.md): documentation conventions
  and illustrative examples.
- [Example area specification](./example/example.spec.md): example area
  requirements and acceptance scenarios.
