//! Phase 09: FFI and misc tests.

#[test]
fn test_ffi_version() {
    let version = agentic_reality::ffi::agentic_reality_ffi_version();
    assert_eq!(version, "0.1.0");
}

#[test]
fn test_now_micros() {
    let t1 = agentic_reality::types::now_micros();
    let t2 = agentic_reality::types::now_micros();
    assert!(t2 >= t1);
}

#[test]
fn test_areal_magic() {
    assert_eq!(&agentic_reality::types::AREAL_MAGIC, b"REAL");
}

#[test]
fn test_footer_magic() {
    assert_eq!(&agentic_reality::types::FOOTER_MAGIC, b"REALEND\0");
}

#[test]
fn test_header_size() {
    assert_eq!(agentic_reality::types::HEADER_SIZE, 256);
}

#[test]
fn test_footer_size() {
    assert_eq!(agentic_reality::types::FOOTER_SIZE, 64);
}

#[test]
fn test_format_version() {
    assert_eq!(agentic_reality::types::FORMAT_VERSION, 1);
}

#[test]
fn test_engine_default() {
    let engine = agentic_reality::engine::RealityEngine::default();
    assert!(!engine.is_initialized());
    assert!(!engine.is_dirty());
}
