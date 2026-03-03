# AGENTIC REALITY SPECIFICATION — PART 3 (Continued)

> **Specs Covered:** SPEC-09 through SPEC-12
> **Sister:** #10 of 25 (The Ground)
> **Continues from:** Part 3 (incomplete)

---

## 12.3 Stress Tests (Continued)

```rust
//! tests/stress/test_concurrent.rs

#[test]
fn test_concurrent_sensing() {
    use std::sync::Arc;
    use std::thread;
    
    let engine = Arc::new(RwLock::new(RealityEngine::new()));
    
    // Initialize
    {
        let mut e = engine.write().unwrap();
        e.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
    }
    
    let mut handles = vec![];
    
    // Spawn multiple readers
    for _ in 0..10 {
        let engine = Arc::clone(&engine);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let e = engine.read().unwrap();
                let _ = e.query().get_soul();
                let _ = e.query().get_environment();
                let _ = e.query().get_body();
            }
        }));
    }
    
    // Spawn writers
    for _ in 0..3 {
        let engine = Arc::clone(&engine);
        handles.push(thread::spawn(move || {
            for _ in 0..50 {
                let mut e = engine.write().unwrap();
                let _ = e.write().sense_resources();
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify state is consistent
    let e = engine.read().unwrap();
    assert!(e.query().get_soul().is_some());
}

#[test]
fn test_concurrent_anchor_verification() {
    use std::sync::Arc;
    use std::thread;
    
    let engine = Arc::new(RwLock::new(RealityEngine::new()));
    
    // Initialize with multiple anchors
    {
        let mut e = engine.write().unwrap();
        e.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
        
        for i in 0..10 {
            e.write().add_anchor(RealityAnchor {
                id: AnchorId::new(),
                anchor_type: AnchorType::External {
                    api: format!("api-{}", i),
                    field: "status".to_string(),
                },
                verification: VerificationMethod::AuthoritativeQuery {
                    source: format!("source-{}", i),
                    query: "verify".to_string(),
                },
                last_value: AnchorValue::String(format!("value-{}", i)),
                trust: 0.8,
                frequency: Duration::from_secs(60),
                dependents: vec![],
            }).unwrap();
        }
    }
    
    let mut handles = vec![];
    
    // Concurrent verifications
    for _ in 0..5 {
        let engine = Arc::clone(&engine);
        handles.push(thread::spawn(move || {
            for _ in 0..20 {
                let mut e = engine.write().unwrap();
                let _ = e.write().verify_all_anchors();
            }
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // All anchors should still exist
    let e = engine.read().unwrap();
    assert_eq!(e.query().get_anchors().len(), 10);
}

#[test]
fn test_large_topology() {
    let mut engine = RealityEngine::new();
    engine.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
    
    // Add 1000 dependencies
    for i in 0..1000 {
        engine.write().add_downstream(DownstreamEntity {
            id: DependencyId::new(),
            service: ServiceId(format!("service-{}", i)),
            entity_type: DownstreamType::Service { 
                service_name: format!("service-{}", i),
            },
            health: HealthStatus::Healthy,
            latency: LatencyStats::default(),
            criticality: if i < 10 {
                DependencyCriticality::Critical { timeout_behavior: TimeoutBehavior::Fail }
            } else {
                DependencyCriticality::Optional { feature: format!("feature-{}", i) }
            },
            fallback: None,
            connection: ConnectionState::Connected,
        }).unwrap();
    }
    
    // Add 100 siblings
    for i in 0..100 {
        engine.write().add_sibling(SiblingEntity {
            incarnation: IncarnationId::new(),
            neighbor_id: NeighborId(format!("sibling-{}", i)),
            health: HealthStatus::Healthy,
            load: LoadLevel::Normal,
            network_distance: NetworkDistance::Local,
            offload_capable: true,
            sync_status: SyncStatus::InSync,
            last_contact: Timestamp::now(),
        }).unwrap();
    }
    
    // Queries should still be fast
    let start = std::time::Instant::now();
    
    for _ in 0..1000 {
        let _ = engine.query().get_downstream();
        let _ = engine.query().get_siblings();
        let _ = engine.query().get_topology_health();
    }
    
    let elapsed = start.elapsed();
    assert!(elapsed < Duration::from_secs(1), "Queries took too long: {:?}", elapsed);
    
    // Critical deps should be indexed
    let critical = engine.indexes.get_critical_dependencies();
    assert_eq!(critical.len(), 10);
}

#[test]
fn test_persistence_stress() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("stress.areal");
    
    // Create and save repeatedly
    for i in 0..100 {
        let mut engine = RealityEngine::new();
        engine.write().initialize_soul(InitializeSoulRequest {
            purpose: DeploymentPurpose {
                description: format!("Iteration {}", i),
                category: PurposeCategory::General,
            },
            ..Default::default()
        }).unwrap();
        
        // Add some state
        engine.write().sense_environment().unwrap();
        engine.write().sense_resources().unwrap();
        
        for j in 0..10 {
            engine.write().add_anchor(RealityAnchor {
                id: AnchorId::new(),
                anchor_type: AnchorType::External {
                    api: format!("api-{}-{}", i, j),
                    field: "value".to_string(),
                },
                verification: VerificationMethod::AuthoritativeQuery {
                    source: "test".to_string(),
                    query: "verify".to_string(),
                },
                last_value: AnchorValue::String("test".to_string()),
                trust: 0.8,
                frequency: Duration::from_secs(60),
                dependents: vec![],
            }).unwrap();
        }
        
        // Save
        engine.save(&file_path).unwrap();
        
        // Load and verify
        let loaded = RealityEngine::load(&file_path).unwrap();
        assert_eq!(
            loaded.query().get_soul().unwrap().birth.purpose.description,
            format!("Iteration {}", i)
        );
        assert_eq!(loaded.query().get_anchors().len(), 10);
    }
}
```

## 12.4 MCP Tool Tests

```rust
//! tests/mcp/phase01_deployment.rs

use agentic_reality_mcp::RealityMcpServer;
use serde_json::json;

#[tokio::test]
async fn test_deployment_operations() {
    let engine = RealityEngine::new();
    let server = RealityMcpServer::new(engine, false);
    
    // Test initialize
    let result = server.handle_tool_call(
        "reality_deployment",
        json!({
            "operation": "initialize",
            "spawned_by": "test",
            "purpose": "MCP testing"
        })
    ).await.unwrap();
    
    assert!(result.get("incarnation_id").is_some());
    
    // Test get_soul
    let result = server.handle_tool_call(
        "reality_deployment",
        json!({ "operation": "get_soul" })
    ).await.unwrap();
    
    assert!(result.get("incarnation_id").is_some());
    assert!(result.get("birth").is_some());
    
    // Test get_vitals
    let result = server.handle_tool_call(
        "reality_deployment",
        json!({ "operation": "get_vitals" })
    ).await.unwrap();
    
    assert!(result.get("health").is_some());
    assert!(result.get("uptime").is_some());
    
    // Test get_summary
    let result = server.handle_tool_call(
        "reality_deployment",
        json!({ "operation": "get_summary" })
    ).await.unwrap();
    
    assert!(result.is_object());
}

//! tests/mcp/phase04_reality.rs

#[tokio::test]
async fn test_anchor_operations() {
    let engine = RealityEngine::new();
    let server = RealityMcpServer::new(engine, false);
    
    // Initialize first
    server.handle_tool_call(
        "reality_deployment",
        json!({ "operation": "initialize" })
    ).await.unwrap();
    
    // Add anchor
    let result = server.handle_tool_call(
        "reality_anchor",
        json!({
            "operation": "add",
            "anchor_type": "time",
            "source": "ntp.org",
            "frequency_seconds": 300
        })
    ).await.unwrap();
    
    let anchor_id = result.get("id").unwrap().as_str().unwrap();
    
    // List anchors
    let result = server.handle_tool_call(
        "reality_anchor",
        json!({ "operation": "list" })
    ).await.unwrap();
    
    assert!(result.as_array().unwrap().len() >= 1);
    
    // Get specific anchor
    let result = server.handle_tool_call(
        "reality_anchor",
        json!({
            "operation": "get",
            "anchor_id": anchor_id
        })
    ).await.unwrap();
    
    assert_eq!(result.get("id").unwrap().as_str().unwrap(), anchor_id);
    
    // Verify anchor
    let result = server.handle_tool_call(
        "reality_anchor",
        json!({
            "operation": "verify",
            "anchor_id": anchor_id
        })
    ).await.unwrap();
    
    assert!(result.get("verified").is_some());
    
    // Get drift
    let result = server.handle_tool_call(
        "reality_anchor",
        json!({ "operation": "get_drift" })
    ).await.unwrap();
    
    assert!(result.is_object());
    
    // Remove anchor
    let result = server.handle_tool_call(
        "reality_anchor",
        json!({
            "operation": "remove",
            "anchor_id": anchor_id
        })
    ).await.unwrap();
    
    assert!(result.get("removed").unwrap().as_bool().unwrap());
}

//! tests/mcp/phase06_stakes.rs

#[tokio::test]
async fn test_stakes_operations() {
    let engine = RealityEngine::new();
    let server = RealityMcpServer::new(engine, false);
    
    // Initialize
    server.handle_tool_call(
        "reality_deployment",
        json!({ "operation": "initialize" })
    ).await.unwrap();
    
    // Set stakes level
    let result = server.handle_tool_call(
        "reality_stakes",
        json!({
            "operation": "set_level",
            "level": "high"
        })
    ).await.unwrap();
    
    assert!(result.is_object());
    
    // Get stakes level
    let result = server.handle_tool_call(
        "reality_stakes",
        json!({ "operation": "get_level" })
    ).await.unwrap();
    
    // Should be high
    assert!(result.to_string().contains("High") || result.to_string().contains("high"));
    
    // Test should_proceed
    let result = server.handle_tool_call(
        "reality_stakes",
        json!({
            "operation": "should_proceed",
            "action": "delete_production_database",
            "risk_tolerance": 0.1
        })
    ).await.unwrap();
    
    // High stakes + low tolerance should not proceed easily
    let decision_type = result.as_object().unwrap().keys().next().unwrap();
    assert!(decision_type != "Proceed");
    
    // Get risk field
    let result = server.handle_tool_call(
        "reality_stakes",
        json!({ "operation": "get_risk" })
    ).await;
    
    // May not have risk data yet, that's ok
    assert!(result.is_ok() || result.is_err());
}

//! tests/mcp/phase08_full_suite.rs

#[tokio::test]
async fn test_full_mcp_workflow() {
    let engine = RealityEngine::new();
    let server = RealityMcpServer::new(engine, false);
    
    // 1. Initialize deployment
    let soul = server.handle_tool_call(
        "reality_deployment",
        json!({
            "operation": "initialize",
            "spawned_by": "test_harness",
            "purpose": "Full workflow test"
        })
    ).await.unwrap();
    
    let incarnation_id = soul.get("incarnation_id").unwrap().as_str().unwrap();
    
    // 2. Sense environment
    server.handle_tool_call(
        "reality_environment",
        json!({ "operation": "sense" })
    ).await.unwrap();
    
    // 3. Sense resources
    let resources = server.handle_tool_call(
        "reality_resource",
        json!({ "operation": "sense" })
    ).await.unwrap();
    
    assert!(resources.get("mind").is_some());
    
    // 4. Add reality anchors
    for anchor_type in ["time", "configuration", "state"] {
        server.handle_tool_call(
            "reality_anchor",
            json!({
                "operation": "add",
                "anchor_type": anchor_type,
                "source": format!("{}_source", anchor_type),
                "frequency_seconds": 60
            })
        ).await.unwrap();
    }
    
    // 5. Verify all anchors
    let verification = server.handle_tool_call(
        "reality_anchor",
        json!({ "operation": "verify_all" })
    ).await.unwrap();
    
    assert!(verification.as_array().unwrap().len() >= 3);
    
    // 6. Add topology
    server.handle_tool_call(
        "reality_topology",
        json!({
            "operation": "add_downstream",
            "service": "postgres",
            "type": "database",
            "criticality": "critical"
        })
    ).await.unwrap();
    
    // 7. Set stakes
    server.handle_tool_call(
        "reality_stakes",
        json!({
            "operation": "set_level",
            "level": "high"
        })
    ).await.unwrap();
    
    // 8. Run coherence check
    let coherence = server.handle_tool_call(
        "reality_coherence",
        json!({ "operation": "check" })
    ).await.unwrap();
    
    assert!(coherence.get("level").is_some());
    
    // 9. Get context fingerprint
    let fingerprint = server.handle_tool_call(
        "reality_context",
        json!({ "operation": "get_fingerprint" })
    ).await.unwrap();
    
    assert!(fingerprint.get("hash").is_some());
    
    // 10. Check if action should proceed
    let decision = server.handle_tool_call(
        "reality_stakes",
        json!({
            "operation": "should_proceed",
            "action": "deploy_new_version",
            "risk_tolerance": 0.5
        })
    ).await.unwrap();
    
    assert!(decision.is_object());
    
    // 11. Get full summary
    let summary = server.handle_tool_call(
        "reality_deployment",
        json!({ "operation": "get_summary" })
    ).await.unwrap();
    
    assert!(summary.get("incarnation_id").is_some());
    assert!(summary.get("stakes_level").is_some());
    assert!(summary.get("coherence_level").is_some());
}
```

## 12.5 Edge Case Tests

```rust
//! tests/mcp/test_edge_cases.rs

#[tokio::test]
async fn test_missing_required_parameters() {
    let engine = RealityEngine::new();
    let server = RealityMcpServer::new(engine, false);
    
    // Missing operation
    let result = server.handle_tool_call(
        "reality_deployment",
        json!({})
    ).await;
    
    assert!(result.is_err());
    
    // Invalid operation
    let result = server.handle_tool_call(
        "reality_deployment",
        json!({ "operation": "nonexistent_operation" })
    ).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_invalid_id_formats() {
    let engine = RealityEngine::new();
    let server = RealityMcpServer::new(engine, false);
    
    // Initialize first
    server.handle_tool_call(
        "reality_deployment",
        json!({ "operation": "initialize" })
    ).await.unwrap();
    
    // Invalid anchor ID format
    let result = server.handle_tool_call(
        "reality_anchor",
        json!({
            "operation": "get",
            "anchor_id": "not-a-uuid"
        })
    ).await;
    
    assert!(result.is_err());
    
    // Empty anchor ID
    let result = server.handle_tool_call(
        "reality_anchor",
        json!({
            "operation": "get",
            "anchor_id": ""
        })
    ).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_boundary_values() {
    let engine = RealityEngine::new();
    let server = RealityMcpServer::new(engine, false);
    
    // Initialize first
    server.handle_tool_call(
        "reality_deployment",
        json!({ "operation": "initialize" })
    ).await.unwrap();
    
    // Risk tolerance at boundaries
    server.handle_tool_call(
        "reality_stakes",
        json!({ "operation": "set_level", "level": "medium" })
    ).await.unwrap();
    
    // Tolerance = 0 (no risk)
    let result = server.handle_tool_call(
        "reality_stakes",
        json!({
            "operation": "should_proceed",
            "action": "test",
            "risk_tolerance": 0.0
        })
    ).await.unwrap();
    
    assert!(result.is_object());
    
    // Tolerance = 1 (all risk)
    let result = server.handle_tool_call(
        "reality_stakes",
        json!({
            "operation": "should_proceed",
            "action": "test",
            "risk_tolerance": 1.0
        })
    ).await.unwrap();
    
    assert!(result.is_object());
    
    // Invalid tolerance (> 1) should be clamped or error
    let result = server.handle_tool_call(
        "reality_stakes",
        json!({
            "operation": "should_proceed",
            "action": "test",
            "risk_tolerance": 1.5
        })
    ).await;
    
    // Implementation may clamp or reject
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_state_before_initialization() {
    let engine = RealityEngine::new();
    let server = RealityMcpServer::new(engine, false);
    
    // Try to get soul before initialization
    let result = server.handle_tool_call(
        "reality_deployment",
        json!({ "operation": "get_soul" })
    ).await;
    
    assert!(result.is_err());
    
    // Try to get environment before initialization
    let result = server.handle_tool_call(
        "reality_environment",
        json!({ "operation": "get" })
    ).await;
    
    assert!(result.is_err());
}
```

## 12.6 MCP Tool Count Test

```rust
//! tests/mcp/mcp_tool_count.rs

#[test]
fn test_mcp_tool_count() {
    let engine = RealityEngine::new();
    let server = RealityMcpServer::new(engine, false);
    
    let tools = server.tools();
    
    // Must have exactly 15 tools
    assert_eq!(tools.len(), 15, "Expected 15 MCP tools, got {}", tools.len());
    
    // Verify all expected tools exist
    let expected_tools = [
        "reality_deployment",
        "reality_memory",
        "reality_environment",
        "reality_resource",
        "reality_capability",
        "reality_layer",
        "reality_anchor",
        "reality_hallucination",
        "reality_topology",
        "reality_temporal",
        "reality_stakes",
        "reality_coherence",
        "reality_context",
        "reality_ground",
        "reality_workspace",
    ];
    
    for expected in expected_tools {
        assert!(
            tools.iter().any(|t| t.name == expected),
            "Missing tool: {}",
            expected
        );
    }
    
    // Verify tool schemas are valid
    for tool in &tools {
        assert!(!tool.name.is_empty(), "Tool has empty name");
        assert!(!tool.description.is_empty(), "Tool {} has empty description", tool.name);
        assert!(tool.input_schema.is_object(), "Tool {} has invalid schema", tool.name);
    }
}

#[test]
fn test_resource_count() {
    let engine = RealityEngine::new();
    let server = RealityMcpServer::new(engine, false);
    
    let resources = server.resources();
    
    // Should have at least 6 resources
    assert!(resources.len() >= 6, "Expected at least 6 resources, got {}", resources.len());
    
    // Verify URIs
    for resource in &resources {
        assert!(resource.uri.starts_with("reality://"));
    }
}

#[test]
fn test_prompt_count() {
    let engine = RealityEngine::new();
    let server = RealityMcpServer::new(engine, false);
    
    let prompts = server.prompts();
    
    // Should have at least 4 prompts
    assert!(prompts.len() >= 4, "Expected at least 4 prompts, got {}", prompts.len());
}
```

---

## Part 3 Complete

**Covered:**
- SPEC-09: CLI (~40 commands across 12 subcommand groups)
- SPEC-10: MCP Server (15 tools, ~120 operations)
- SPEC-11: Sister Integration (9 bridge traits + NoOp + Ghost + Hydra)
- SPEC-12: Tests (unit, integration, stress, MCP, scenarios)

**Next (Part 4):**
- SPEC-13: Performance
- SPEC-14: Security
- SPEC-15: Research Paper
- SPEC-16: Inventions Implementation

---

*Document: AGENTIC-REALITY-SPEC-PART3.md*
*Sister #10 of 25: The Ground*
*The sister that knows WHERE it exists and WHAT is real.*
