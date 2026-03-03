//! Phase 07: Security tests.

use agentic_reality::security::*;

#[test]
fn test_auth_manager_no_token() {
    let mut mgr = AuthManager::new();
    assert!(!mgr.is_auth_required());
    let session = mgr.authenticate("anything").unwrap();
    assert_eq!(session, "anonymous");
}

#[test]
fn test_permissions_default() {
    let perms = Permissions::default();
    assert!(perms.deployment_read);
    assert!(perms.deployment_write);
    assert!(perms.environment_read);
    assert!(perms.resource_read);
    assert!(perms.topology_read);
    assert!(perms.temporal_read);
    assert!(perms.stakes_read);
    assert!(!perms.admin);
}

#[test]
fn test_audit_logger_new() {
    let logger = AuditLogger::new();
    // Should not panic even without env var
    logger.log_operation("test", "session-1", true);
}

#[test]
fn test_audit_logger_writes() {
    let dir = tempfile::tempdir().unwrap();
    let log_path = dir.path().join("audit.log");
    std::env::set_var("AGENTIC_AUDIT_LOG", log_path.to_str().unwrap());
    let logger = AuditLogger::new();
    logger.log_operation("test-op", "session-test", true);
    std::env::remove_var("AGENTIC_AUDIT_LOG");
    let content = std::fs::read_to_string(&log_path).unwrap();
    assert!(content.contains("test-op"));
    assert!(content.contains("session-test"));
}

#[test]
fn test_session_binding_fields() {
    let binding = SessionBinding {
        session_id: "s-1".into(),
        created_at: 1000,
        last_activity: 2000,
        permissions: Permissions::default(),
    };
    assert_eq!(binding.session_id, "s-1");
    assert_eq!(binding.created_at, 1000);
}
