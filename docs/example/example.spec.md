---
title: Example area - SPEC
summary: Illustrative specification used to demonstrate the requirement format.
tags: [example, specification, spec]
---

> [!IMPORTANT]
> Requirements in this specification use EARS notation. **SHALL** and **SHALL
> NOT** define mandatory behavior; **SHOULD** and **SHOULD NOT** define expected
> behavior that requires an explicit justification to deviate from; **MAY**
> defines optional behavior.
>
> These terms follow RFC 2119 and RFC 8174.

## Requirements

### EXAMPLE_AREA-REQ-0001

Name: Explicit configuration

**Links:**

- [Intent](../example-intent.md)
- [EXAMPLE_AREA-ACC-0001](./example.acceptance.md#example_area-acc-0001)

**Status**:

- Doc status: Draft
- Implementation status: Not implemented

```text
WHEN: Configuration provides an unknown key,

THE SOFTWARE SHALL: reject the configuration.
```
