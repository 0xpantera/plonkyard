# plonkyard

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
2. Refactor repeated patterns into Chips once the raw version is understood.
3. Study lookups as their own topic instead of mixing them into early examples.
4. Build real gadgets after the smaller pieces feel mechanical.

In parallel, the notes will track the zkSecurity PLONK tutorial vocabulary and
connect it back to what the Halo2 code is doing.

## Planned Exercises

These are planned exercises, not implemented features.

| # | Exercise |
|---|---|
| 1 | Multiplication |
| 2 | Equality / Copy Constraints |
| 3 | Boolean |
| 4 | Bit Decomposition |
| 5 | Conditional Select |
| 6 | IsZero |
| 7 | Range Check |
| 8 | Fibonacci |
| 9 | Lookups |
| 10 | Poseidon |
| 11 | Merkle Path |

## Workspace

```text
crates/
  common/
  halo2-exercises/
docs/
notes/
```

`halo2-exercises` is where the small exercises will live. `common` is reserved
for helpers that genuinely become shared across multiple exercises.

## Commands

```sh
just check
just test
just fmt
just clippy
```

