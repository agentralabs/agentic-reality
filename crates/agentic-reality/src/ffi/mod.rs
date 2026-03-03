//! FFI bindings for AgenticReality.

/// Version of the FFI interface.
pub fn agentic_reality_ffi_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
