# RareSkills Homework 04

## Status

- Problem 1: planned.
- Problem 2 challenge: planned for later, not part of the immediate weekend
  plan.

## Problems

1. Implement Fibonacci using one advice column, enforcing the first two numbers
   with `constrain_equal` against a fixed cell and with `constrain_constant`
   after `enable_constant`.
2. Challenge: use a public input target index for Fibonacci, fixed max rows,
   padding after the target index, and analyze which constraint is missing.

## Concepts

- fixed columns
- `constrain_equal`
- `enable_constant`
- `constrain_constant`
- public inputs
- padding traces
- missing-constraint analysis

## Related Tiny Exercises

- `crates/10-fib-constants`
- `crates/11-fib-public-output`

## Plan

Do the constants version first. Treat the public target-index challenge as an
analysis-heavy exercise, not just a coding task.

## Notes

The challenge is useful because it asks what the circuit does not prove. That
kind of missing-constraint audit should stay explicit in the exercise notes.

