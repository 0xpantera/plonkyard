//! Tiny Halo2 exercise modules will live here.
//!
//! No circuits are implemented yet. The first pass of this repository is about
//! establishing a clean notebook-like workspace.

pub mod multiplication;

/// Minimal placeholder used to verify that the exercise crate compiles.
pub fn placeholder() -> &'static str {
    plonkyard_common::workspace_name()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_points_to_workspace() {
        assert_eq!(placeholder(), "plonkyard");
    }
}
