//! Minimal FFI facade for AgenticReality.

/// Crate version exposed for foreign runtimes.
pub fn agentic_reality_ffi_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
