---
title: Example intent
summary: Illustrative intent statement used to demonstrate the specification format.
tags: [specification, example, intent]
---

## Problem

Configuration read directly from process state is difficult to review,
reproduce, or audit.

## Intent

The feature must make process configuration explicit rather than inheriting
undeclared host state.

## Boundaries

This example does not define how configuration is stored, transmitted, or
loaded — only that unknown keys are rejected.

## Documentation

- [Readme](./README.md): module overview.
- [Example area specification](./example/example.spec.md): example area
  requirements and acceptance scenarios.
