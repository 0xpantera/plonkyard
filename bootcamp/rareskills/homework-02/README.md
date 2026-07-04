# RareSkills Homework 02

## Status

- `01-mul`: completed separately as the multiplication warmup.
- Homework 2 Problem 1: planned.
- Homework 2 Problem 2: planned.
- Homework 2 Problem 3: planned.
- Instance-column challenge: planned.

## Problems

1. Write a custom gate for `a + b = c` and enforce it in the first five rows.
2. Write custom gates for `a + b = c` and `a * b = c` over eight rows, using
   addition on even rows and multiplication on odd rows.
3. Write a high-degree gate `a * b * c * d * e * f * g = h` for `K` rows, where
   `K` is a compile-time circuit parameter.
4. Challenge: change `h` to an instance column.

## Concepts

- advice columns
- selectors
- custom gates
- row-by-row assignment
- compile-time circuit parameters
- instance columns and public inputs

## Related Tiny Exercises

- `crates/02-add-five-rows`
- `crates/03-add-mul-selectors`
- `crates/04-high-degree-gate`
- `crates/05-instance-column`

## Plan

Start with one direct circuit per problem. Avoid helper abstractions until the
same assignment or selector pattern repeats enough to become distracting.

## Notes

Problem 2 should make selector placement visible: the point is not just having
two gates, but understanding which rows each gate actually constrains.

