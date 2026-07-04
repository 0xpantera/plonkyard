# RareSkills Homework 03

## Status

- Problem 1: planned.
- Problem 2: planned.
- Explicit bit witnesses variant: planned.
- Internally computed bit witnesses variant: planned.

## Problems

1. Implement Fibonacci with one advice column `a`, using the recurrence
   `a[i + 2] = a[i + 1] + a[i]`. The selector must not be enabled on every row.
   Challenge: enforce `a[0] = a[1] = 1` using either gate constraints or fixed
   column/copy constraints.
2. Range check `v` in `0..16` without lookups using bit decomposition:
   `v = b0 + 2b1 + 4b2 + 8b3`, and each `bi` is a bit. Implement it once with
   all values explicitly provided and once where only `v` is provided and the
   witness bits are computed internally.

## Concepts

- rotations
- selectors that stop before the final rows
- boolean constraints
- bit decomposition
- range checks without lookups
- computed witnesses

## Related Tiny Exercises

- `crates/06-fib-one-column`
- `crates/07-bool`
- `crates/08-bit-decomp`
- `crates/09-range-0-16`

## Plan

Keep Fibonacci separate from range checking. For the range check, first write
the fully explicit witness version, then add the internally computed witness
version after the constraints are clear.

## Notes

The selector placement in Fibonacci is part of the exercise. Enabling the
recurrence gate on every row would incorrectly query beyond the intended trace.

