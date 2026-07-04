# Learning Path

`plonkyard` grows in stages. Each stage should leave behind small, runnable
examples rather than broad abstractions.

## Stage 1: Small Raw Halo2 Circuits

Start with tiny circuits written directly against the Halo2 APIs. The first goal
is to understand advice columns, fixed columns, selectors, regions, witnesses,
and simple gates without hiding them behind reusable components.

Current tiny-exercise path:

| Exercise | Status | RareSkills anchor |
|---|---|---|
| `01-mul` | Done | Pre-homework multiplication warmup |
| `02-add-five-rows` | Planned | Homework 2 Problem 1 |
| `03-add-mul-selectors` | Planned | Homework 2 Problem 2 |
| `04-high-degree-gate` | Planned | Homework 2 Problem 3 |
| `05-instance-column` | Planned | Homework 2 challenge |
| `06-fib-one-column` | Planned | Homework 3 Problem 1 |
| `07-bool` | Planned | Homework 3 Problem 2 subconstraint |
| `08-bit-decomp` | Planned | Homework 3 Problem 2 |
| `09-range-0-16` | Planned | Homework 3 Problem 2 |
| `10-fib-constants` | Planned | Homework 4 Problem 1 |
| `11-fib-public-output` | Planned | Homework 4 challenge |
| `12-arithmetic-chip` | Planned | Homework 5 Problem 1 |
| `13-is-zero-chip` | Planned | Homework 5 Problem 2 |
| `14-polynomial-a5-plus-a` | Planned | Homework 5 Problem 3 |
| `15-xor-32` | Planned | Homework 5 Problem 4 |

## Stage 2: Refactor into Chips

After several raw circuits exist, refactor repeated patterns into Chips. This
stage should make the abstraction cost visible by comparing the raw exercise to
the Chip-based version. Homework 5 is the first planned chip stage; do not rush
there before the raw constraints feel routine.

## Stage 3: Lookups

Introduce lookup tables after equality constraints, booleans, range checks, and
small custom gates are familiar. Treat lookups as a separate learning topic so
their protocol role and Halo2 ergonomics can be studied clearly.

## Stage 4: Real Gadgets

Use the smaller lessons to build real gadgets such as Poseidon and Merkle path
verification. These should still be written as learning exercises first, with
production-style polish added only when it clarifies the code.
