# plonkyard

[![CI](https://github.com/0xpantera/plonkyard/actions/workflows/ci.yml/badge.svg)](https://github.com/0xpantera/plonkyard/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

`plonkyard` is a Rust workspace for learning Halo2 and PLONKish proof systems
through many tiny, self-contained exercises.

The point is not to build a reusable framework. The point is to keep a working
engineer's notebook: small circuits, direct code, clear tests, and notes that
connect implementation details back to the underlying protocol.

## Motivation

Halo2 is easier to learn when each concept can be isolated and run. The Halo2
Book is useful reference material, but it can feel abstract before the mechanics
are familiar. RareSkills-style exercises are a better fit for the first pass:
write one small circuit, inspect the constraints, test it, and only then move
on.

This repository is meant to support that workflow over time. It should be able
to grow into dozens of exercises without forcing early abstractions.

Chips, gadgets, helper crates, and reusable abstractions are intentionally
delayed. They belong here only after the raw constraints feel mechanical enough
that the abstraction teaches something instead of hiding the thing being learned.

## Philosophy

Many tiny circuits:

- prefer one concept per exercise
- keep raw Halo2 code visible before introducing Chips
- add helpers only after repeated friction appears
- favor readable tests over clever abstractions
- document the protocol idea next to the engineering lesson

## Learning Roadmap

The initial path is staged:

1. Write small raw Halo2 circuits with minimal abstraction.
2. Work through the RareSkills-style homework path: custom gates, selectors,
   equality, constants, public inputs, and bit decomposition.
3. Refactor repeated patterns into Chips once the raw version is understood.
4. Study lookups as their own topic instead of mixing them into early examples.
5. Build real gadgets after the smaller pieces feel mechanical.

In parallel, the notes will track the zkSecurity PLONK tutorial vocabulary and
connect it back to what the Halo2 code is doing.

## Planned Exercises

Most exercise crates start as placeholders. A checked item means there is at
least one small circuit and test for that topic.

- [x] 01. Multiplication
- [x] 02. Add Five Rows
- [x] 03. Add/Mul Selectors
- [x] 04. High-Degree Gate
- [ ] 05. Instance Column
- [ ] 06. Fibonacci, One Advice Column
- [ ] 07. Boolean
- [ ] 08. Bit Decomposition
- [ ] 09. Range 0..16
- [ ] 10. Fibonacci Constants
- [ ] 11. Fibonacci Public Output
- [ ] 12. Arithmetic Chip
- [ ] 13. IsZero Chip
- [ ] 14. Polynomial `a^5 + a = b`
- [ ] 15. XOR-32

## Workspace

```text
crates/
  01-mul/
  02-add-five-rows/
  03-add-mul-selectors/
  04-high-degree-gate/
  05-instance-column/
  06-fib-one-column/
  07-bool/
  08-bit-decomp/
  09-range-0-16/
  10-fib-constants/
  11-fib-public-output/
  12-arithmetic-chip/
  13-is-zero-chip/
  14-polynomial-a5-plus-a/
  15-xor-32/
  common/
bootcamp/
  rareskills/
docs/
notes/
```

Each exercise gets its own crate so examples can stay small, local, and easy to
run in isolation. `common` is reserved for helpers that genuinely become shared
across multiple exercises.

## Commands

```sh
just check
just test
just fmt
just clippy
```

## References

- [Halo2 Book](https://zcash.github.io/halo2/)
- [zkSecurity PLONK tutorial](https://plonk.zksecurity.xyz/1_Getting_started/1_Introduction.html)
- [RareSkills ZK Book](https://www.rareskills.io/zk-book)
