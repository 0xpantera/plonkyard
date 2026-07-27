# RareSkills Homework 07

Source: [Homework 7](https://app.notion.com/p/Homework-7-3a009cb3e96280d4ab72ca7a438cee3b)

## Status

- Problem 1: planned.
- Generic-automaton challenge: blocked on backend/API choice.

## Problems

1. Implement a fixed finite automaton as a static lookup table of
   `(current_state, symbol, next_state)` tuples. Prove knowledge of an accepted
   word and pad shorter words to a fixed maximum length with a no-op symbol.
2. Challenge: make the transition function and accepting states public,
   variable inputs. Copy them from instance columns into advice columns, then
   look up the private trace against those advice columns with inclusion flags.

The fixed automaton in Problem 1 recognizes binary values divisible by three:

```text
(r0, 0, r0)  (r0, 1, r1)
(r1, 0, r2)  (r1, 1, r0)
(r2, 0, r1)  (r2, 1, r2)
```

`r0` is both the initial and accepting state.

## Concepts

- finite-state-machine traces
- multi-column lookup arguments
- no-op padding
- initial and accepting-state constraints
- public transition tables
- instance-to-advice copying
- advice-to-advice lookups
- lookup inclusion flags

## Related Tiny Exercises

- `crates/19-fixed-automaton-lookup`
- `crates/20-generic-automaton-lookup-any`

## Plan

Start with the fixed divisible-by-three automaton and make every transition,
padding rule, initial state, and final state explicit. Attempt the generic
challenge only after selecting a Halo2 backend that supports advice-to-advice
lookup arguments.

## Compatibility Note

This workspace currently uses upstream `halo2_proofs 0.3.2`. That version
provides `ConstraintSystem::lookup` against `TableColumn`, but does not provide
the `lookup_any` API required by the challenge as written. Keep the generic
exercise as a placeholder until the dependency strategy is chosen explicitly.

## Notes

Do not encode `r0` as zero without first addressing inactive and padded rows.
Unassigned or zero-filled cells can otherwise become indistinguishable from the
real initial/accepting state and accidentally satisfy transition checks.
