//! Small shared helpers for exercises.
//!
//! This crate is intentionally sparse at the start. Add helpers here only when
//! multiple exercises actually need them.

/// Returns the repository name used in examples and documentation.
pub fn workspace_name() -> &'static str {
    "plonkyard"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_name_is_stable() {
        assert_eq!(workspace_name(), "plonkyard");
    }
}
