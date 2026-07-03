# Learning Path

`plonkyard` grows in stages. Each stage should leave behind small, runnable
examples rather than broad abstractions.

## Stage 1: Small Raw Halo2 Circuits

Start with tiny circuits written directly against the Halo2 APIs. The first goal
is to understand advice columns, fixed columns, selectors, regions, witnesses,
and simple gates without hiding them behind reusable components.

## Stage 2: Refactor into Chips

After several raw circuits exist, refactor repeated patterns into Chips. This
stage should make the abstraction cost visible by comparing the raw exercise to
the Chip-based version.

## Stage 3: Lookups

Introduce lookup tables after equality constraints, booleans, range checks, and
small custom gates are familiar. Treat lookups as a separate learning topic so
their protocol role and Halo2 ergonomics can be studied clearly.

## Stage 4: Real Gadgets

Use the smaller lessons to build real gadgets such as Poseidon and Merkle path
verification. These should still be written as learning exercises first, with
production-style polish added only when it clarifies the code.

