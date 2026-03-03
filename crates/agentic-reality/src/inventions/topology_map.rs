//! Invention implementation.

/// Invention marker — implementation details are in the types and engine modules.
pub struct Invention;

impl Invention {
    pub fn name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }
}
