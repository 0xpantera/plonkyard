# AGENTS.md

This repository is an engineer's notebook for learning Halo2 and PLONKish proof
systems. Keep changes small, explicit, and easy to inspect.

## Working Style

- Do not add Halo2 circuits unless the task explicitly asks for one.
- Prefer raw Halo2 exercises before introducing Chips or shared abstractions.
- Add dependencies only when a concrete exercise needs them.
- Keep each exercise self-contained enough to read in one sitting.
- Avoid framework-style architecture until repeated examples justify it.

## Verification

Before handing off code changes, run the narrowest relevant checks. For general
workspace edits, use:

```sh
cargo check --workspace
cargo test --workspace
```

Use `just` recipes when available.

