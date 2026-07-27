# RareSkills Homework 06

Source: [Homework 6](https://app.notion.com/p/Homework-6-39309cb3e96280588089d99d0ac89b95)

## Status

- Problem 1: planned.
- Problem 2: planned.
- Chunked-addition challenge: planned.

## Problems

1. Use a static lookup table to compute the XOR of two 8-bit values. Explore
   16-bit and 32-bit variants and identify when the table size becomes
   impractical.
2. Add two 16-bit values using a static range table and an advice carry bit so
   overflow is constrained rather than ignored.
3. Challenge: add 32-bit values by decomposing them into smaller 8-bit or
   16-bit chunks and propagating carries between chunks.

## Concepts

- static lookup tables
- tuple lookups
- table-size tradeoffs
- overflow and carry witnesses
- limb decomposition
- carry propagation

## Related Tiny Exercises

- `crates/16-lookup-xor-8`
- `crates/17-lookup-add-carry-16`
- `crates/18-lookup-add-chunks-32`

## Plan

Implement the 8-bit XOR table first and measure how its row count scales before
attempting wider variants. Then isolate carry-aware 16-bit addition before
composing multiple limbs for the 32-bit challenge.

## Notes

Both homework problems use static lookup tables. Keep the table-generation code
visible and document the number of rows required at each bit width; the scaling
limit is part of the lesson, not merely an implementation detail.
