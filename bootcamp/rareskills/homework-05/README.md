# RareSkills Homework 05

## Status

- Problem 1: planned, but not this weekend.
- Problem 2: planned, but not this weekend.
- Problem 3: planned, but not this weekend.
- Problem 4: planned, but not this weekend.

## Problems

1. Build an arithmetic chip for addition and multiplication with shared
   circuit-level columns `a`, `b`, and `c`.
2. Build an `IsZero` chip with an auxiliary input.
3. Use the arithmetic chip to prove knowledge of `a` such that `a^5 + a = b`,
   where `b` is public.
4. Build a 32-bit XOR chip, including range constraints.

## Concepts

- Chips
- reusable circuit configuration
- auxiliary witnesses
- public outputs
- polynomial composition
- 32-bit bitwise constraints
- range constraints

## Related Tiny Exercises

- `crates/12-arithmetic-chip`
- `crates/13-is-zero-chip`
- `crates/14-polynomial-a5-plus-a`
- `crates/15-xor-32`

## Plan

Defer this homework until the raw custom gates, selectors, equality
constraints, constants, and range checks feel mechanical. The chip exercises
should be refactors that clarify repeated patterns, not the first exposure to
the constraints.

## Notes

This is where abstractions begin to earn their keep. Keep each chip small and
compare it back to the raw circuits it replaces.

