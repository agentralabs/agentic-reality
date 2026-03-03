# AgenticReality Examples

> Code examples for common use cases

## Example 1: Basic Reality Grounding

Initialize and ground an agent in its reality.

```rust
use agentic_reality::{
    RealityEngine, InitializeSoulRequest,
    DeploymentPurpose, PurposeCategory, SpawnerIdentity,
};

fn main() -> anyhow::Result<()> {
    let mut engine = RealityEngine::new();

    // 1. Initialize the deployment soul
    let soul = engine.write().initialize_soul(InitializeSoulRequest {
        spawned_by: SpawnerIdentity::System("main".to_string()),
        purpose: DeploymentPurpose {
            description: "Payment processing API".to_string(),
            category: PurposeCategory::Api,
        },
        expected_lifetime: None,
        circumstances: None,
        substrate: None,  // auto-detect
        role: None,
        nature: None,
    })?;

    println!("Incarnation: {}", soul.incarnation_id);
    println!("Substrate: {:?}", soul.substrate.tier);

    // 2. Sense the environment
    let env = engine.write().sense_environment()?;
    println!("Environment: {:?}", env.environment_type);
    println!("Mood: {:?}", env.current_state.mood);

    // 3. Sense resources
    let body = engine.write().sense_resources()?;
    println!("Mind: {:?} ({:.0}%)", body.mind.feeling,
        body.mind.used.0 as f64 / body.mind.total.0 as f64 * 100.0);
    println!("Energy: {:?} ({:.0}%)", body.energy.feeling,
        body.energy.utilization * 100.0);

    // 4. Save state
    engine.save("app.areal")?;

    Ok(())
}
```

## Example 2: Stakes-Aware Decision Making

Check stakes before performing a risky action.

```rust
use agentic_reality::{
    RealityEngine, StakesLevel, ProceedDecision,
};

fn should_deploy(engine: &RealityEngine) -> bool {
    // Check the stakes level
    let stakes = engine.query().get_stakes_level();

    match stakes {
        Some(StakesLevel::Critical { .. }) => {
            println!("CRITICAL stakes - deployment blocked without approval");
            false
        }
        Some(StakesLevel::High { .. }) => {
            // Check if conditions allow proceeding
            let decision = engine.query().should_proceed("deploy_new_version", 0.5);

            match decision {
                ProceedDecision::Proceed => {
                    println!("High stakes but conditions are favorable - proceeding");
                    true
                }
                ProceedDecision::ProceedWithCaution { conditions } => {
                    println!("Proceeding with conditions:");
                    for condition in &conditions {
                        println!("  - {}", condition);
                    }
                    true
                }
                ProceedDecision::Halt { reason } => {
                    println!("Deployment halted: {}", reason);
                    false
                }
                ProceedDecision::Defer { reason, .. } => {
                    println!("Deployment deferred: {}", reason);
                    false
                }
            }
        }
        _ => {
            println!("Low/medium stakes - proceeding normally");
            true
        }
    }
}
```

## Example 3: Resource-Aware Processing

Adapt behavior based on resource pressure.

```rust
use agentic_reality::{
    RealityEngine, MindFeeling, EnergyFeeling, ResourceType,
};

fn choose_processing_strategy(engine: &RealityEngine) -> ProcessingStrategy {
    let body = match engine.query().get_body() {
        Some(b) => b,
        None => return ProcessingStrategy::Default,
    };

    // Check mind (memory) feeling
    match body.mind.feeling {
        MindFeeling::Drowning | MindFeeling::Overwhelmed => {
            // Critical memory pressure - minimize memory usage
            return ProcessingStrategy::MinimalMemory;
        }
        MindFeeling::Strained => {
            // High memory pressure - reduce batch sizes
            return ProcessingStrategy::SmallBatches;
        }
        _ => {}
    }

    // Check energy (CPU) feeling
    match body.energy.feeling {
        EnergyFeeling::Depleted | EnergyFeeling::Constrained => {
            // CPU exhausted - defer non-critical work
            return ProcessingStrategy::DeferNonCritical;
        }
        _ => {}
    }

    // Check the bottleneck
    if let Some(bottleneck) = engine.query().get_bottleneck() {
        match bottleneck {
            ResourceType::Network => ProcessingStrategy::BatchNetworkCalls,
            ResourceType::Storage => ProcessingStrategy::UseCache,
            _ => ProcessingStrategy::Default,
        }
    } else {
        ProcessingStrategy::Default
    }
}

enum ProcessingStrategy {
    Default,
    MinimalMemory,
    SmallBatches,
    DeferNonCritical,
    BatchNetworkCalls,
    UseCache,
}
```

## Example 4: Reality Anchor Management

Set up and verify reality anchors.

```rust
use agentic_reality::{
    RealityEngine, RealityAnchor, AnchorId, AnchorType,
    VerificationMethod, AnchorValue,
};
use std::time::Duration;

fn setup_anchors(engine: &mut RealityEngine) -> anyhow::Result<()> {
    // Add a time anchor (verified against NTP)
    engine.write().add_anchor(RealityAnchor {
        id: AnchorId::new(),
        anchor_type: AnchorType::Time {
            source: "pool.ntp.org".to_string(),
        },
        verification: VerificationMethod::AuthoritativeQuery {
            source: "pool.ntp.org".to_string(),
            query: "time".to_string(),
        },
        last_value: AnchorValue::Timestamp(chrono::Utc::now()),
        trust: 0.99,
        frequency: Duration::from_secs(300),
        dependents: vec![],
    })?;

    // Add a database state anchor
    engine.write().add_anchor(RealityAnchor {
        id: AnchorId::new(),
        anchor_type: AnchorType::State {
            service: "postgres".to_string(),
            metric: "row_count".to_string(),
        },
        verification: VerificationMethod::AuthoritativeQuery {
            source: "postgres".to_string(),
            query: "SELECT count(*) FROM users".to_string(),
        },
        last_value: AnchorValue::Integer(1_500_000),
        trust: 0.90,
        frequency: Duration::from_secs(120),
        dependents: vec!["user_count_cache".to_string()],
    })?;

    // Add a configuration anchor
    engine.write().add_anchor(RealityAnchor {
        id: AnchorId::new(),
        anchor_type: AnchorType::Configuration {
            source: "env".to_string(),
            key: "MAX_CONNECTIONS".to_string(),
        },
        verification: VerificationMethod::EnvironmentVariable {
            var_name: "MAX_CONNECTIONS".to_string(),
        },
        last_value: AnchorValue::String("100".to_string()),
        trust: 0.95,
        frequency: Duration::from_secs(60),
        dependents: vec!["connection_pool_size".to_string()],
    })?;

    println!("Added 3 reality anchors");

    // Verify all anchors
    let anchors = engine.query().get_anchors();
    println!("Verifying {} anchors...", anchors.len());

    for anchor in anchors {
        let result = engine.write().verify_anchor(&anchor.id)?;
        println!("  {} - verified: {}, drift: {:?}",
            anchor.anchor_type, result.verified, result.drift);
    }

    Ok(())
}
```

## Example 5: Topology-Aware Load Handling

Use topology awareness to make load-balancing decisions.

```rust
use agentic_reality::{
    RealityEngine, HealthStatus, LoadLevel,
};

fn handle_request(engine: &RealityEngine, request: Request) -> Response {
    // Check if we should handle this or offload
    let body = engine.query().get_body();
    let siblings = engine.query().get_siblings();

    // If we are overloaded, try to offload to a sibling
    if let Some(body) = body {
        if body.energy.utilization > 0.85 {
            // Find an idle sibling
            if let Some(idle_sibling) = engine.query().get_sibling_for_offload() {
                println!("Offloading to sibling {} (load: {:?})",
                    idle_sibling.neighbor_id.0, idle_sibling.load);
                return offload_to_sibling(idle_sibling, request);
            }
        }
    }

    // Check dependency health before processing
    let failing = engine.query().get_failing_deps();
    if !failing.is_empty() {
        println!("Warning: {} dependencies failing", failing.len());

        // Check if any critical deps are failing
        let critical_failing = engine.query().get_critical_deps()
            .iter()
            .filter(|d| d.health != HealthStatus::Healthy)
            .count();

        if critical_failing > 0 {
            return Response::service_unavailable(
                "Critical dependency unavailable"
            );
        }
    }

    // Process normally
    process_request(request)
}
```

## Example 6: Context Shift Detection

Monitor for context shifts and adapt behavior.

```rust
use agentic_reality::RealityEngine;

fn check_context_stability(engine: &RealityEngine) {
    // Check if context has shifted
    match engine.query().has_context_shifted(0.3) {
        Ok(result) if result.shifted => {
            println!("Context has shifted (distance: {:.2})", result.distance);
            println!("Changed components:");
            for component in &result.changed_components {
                println!("  - {}", component);
            }

            // Increase caution
            match result.shift_severity {
                ShiftSeverity::Minor => {
                    println!("Minor shift - continuing with increased logging");
                }
                ShiftSeverity::Moderate => {
                    println!("Moderate shift - re-verifying anchors");
                    // engine.write().verify_all_anchors();
                }
                ShiftSeverity::Major => {
                    println!("Major shift - re-sensing environment");
                    // engine.write().sense_environment();
                    // engine.write().sense_resources();
                }
                ShiftSeverity::Extreme => {
                    println!("Extreme shift - full re-grounding required");
                    // engine.write().full_sense();
                }
            }
        }
        Ok(_) => {
            println!("Context is stable");
        }
        Err(e) => {
            println!("Could not check context: {}", e);
        }
    }
}
```

## Example 7: Coherence-Checked Operations

Run coherence checks before critical operations.

```rust
use agentic_reality::{
    RealityEngine, CoherenceLevel,
};

fn perform_critical_operation(engine: &mut RealityEngine) -> anyhow::Result<()> {
    // Run coherence check first
    let check = engine.write().run_coherence_check()?;

    match check.level {
        CoherenceLevel::Full => {
            println!("Full coherence - proceeding with critical operation");
            execute_operation()?;
        }
        CoherenceLevel::Partial => {
            println!("Partial coherence - proceeding with extra verification");
            println!("Minor inconsistencies:");
            for v in &check.violations {
                println!("  - {}: {}", v.domain, v.description);
            }
            execute_operation()?;
        }
        CoherenceLevel::Degraded => {
            println!("Degraded coherence - deferring critical operation");
            println!("Significant contradictions:");
            for v in &check.violations {
                println!("  - {}: {}", v.domain, v.description);
            }
            // Try to re-ground
            engine.write().sense_environment()?;
            engine.write().sense_resources()?;

            // Re-check
            let recheck = engine.write().run_coherence_check()?;
            if matches!(recheck.level, CoherenceLevel::Full | CoherenceLevel::Partial) {
                println!("Re-grounding restored coherence - proceeding");
                execute_operation()?;
            } else {
                anyhow::bail!("Cannot achieve coherence for critical operation");
            }
        }
        CoherenceLevel::Incoherent => {
            anyhow::bail!("Incoherent reality state - refusing critical operation");
        }
    }

    Ok(())
}
```

## Example 8: MCP Client Usage

Using AgenticReality as an MCP tool from an AI agent.

```
User: Deploy the new version of the payment service.

Agent (using reality_stakes tool):
  → reality_stakes { "operation": "should_proceed", "action": "deploy_payment_service", "risk_tolerance": 0.3 }
  ← { "ProceedWithCaution": { "conditions": ["Verify rollback plan", "Check blast radius"], "stakes_level": "High" } }

Agent (using reality_topology tool):
  → reality_topology { "operation": "get_health" }
  ← { "overall": "Healthy", "failing": [], "stressed": ["redis"] }

Agent (using reality_resource tool):
  → reality_resource { "operation": "get_bottleneck" }
  ← { "bottleneck": null, "mind_pressure": 0.42, "energy_pressure": 0.31 }

Agent (using reality_coherence tool):
  → reality_coherence { "operation": "check" }
  ← { "level": "Full", "violations": [] }

Agent: Based on my reality check:
- Stakes are HIGH but conditions are favorable
- Topology is healthy (redis is stressed but not critical)
- Resources are adequate (no bottleneck)
- Reality is coherent (no contradictions)

I recommend proceeding with the deployment, but:
1. Monitor redis closely during rollout
2. Have rollback ready
3. Deploy to canary first (10% traffic)
```

## Example 9: Incarnation Memory Across Restarts

Use incarnation memory to learn from past lives.

```rust
use agentic_reality::{
    RealityEngine, WisdomEntry, WisdomCategory,
};

fn startup_with_memory(engine: &mut RealityEngine) -> anyhow::Result<()> {
    // Load state from previous incarnation
    if let Ok(loaded) = RealityEngine::load("app.areal") {
        // Check past life wisdom
        if let Some(wisdom) = loaded.query().get_wisdom() {
            println!("Wisdom from past lives:");

            for entry in &wisdom.what_works {
                println!("  [WORKS] {}", entry.content);
            }
            for entry in &wisdom.what_fails {
                println!("  [FAILS] {}", entry.content);
            }

            // Apply learned configurations
            for (key, value) in &wisdom.optimal_configs {
                println!("  [CONFIG] {} = {:?}", key, value);
                apply_config(key, value);
            }
        }

        // Check for unfinished business
        let past_lives = loaded.query().get_past_lives();
        for past in past_lives {
            if !past.active_tasks.is_empty() {
                println!("Unfinished tasks from incarnation {}:", past.id);
                for task in &past.active_tasks {
                    println!("  - {}", task.description);
                }
            }

            // Learn from how past incarnation died
            println!("Past life died due to: {:?}", past.death.cause);
            if past.death.preventable {
                println!("  This was preventable. Lessons:");
                for lesson in &past.death.lessons {
                    println!("    - {}", lesson);
                }
            }
        }
    }

    // Initialize new incarnation
    engine.write().initialize_soul(InitializeSoulRequest::default())?;

    // Record wisdom from this session as we learn
    engine.write().record_wisdom(WisdomEntry {
        category: WisdomCategory::WhatWorks,
        content: "Connection pool size 50 optimal for this substrate".to_string(),
        context: "m5.xlarge with postgres".to_string(),
        confidence: 0.85,
        learned_at: chrono::Utc::now(),
    })?;

    Ok(())
}
```

## Example 10: Cost-Aware Operation Selection

Choose operations based on cost consciousness.

```rust
use agentic_reality::{
    RealityEngine, CostFeeling, CostDecision,
};

fn choose_embedding_strategy(engine: &RealityEngine, text: &str) -> EmbeddingStrategy {
    let cost = match engine.query().get_cost() {
        Some(c) => c,
        None => return EmbeddingStrategy::Default,
    };

    match cost.feeling {
        CostFeeling::Comfortable => {
            // Plenty of budget - use the best model
            EmbeddingStrategy::HighQuality
        }
        CostFeeling::Mindful => {
            // On track but be thoughtful
            if text.len() > 10_000 {
                EmbeddingStrategy::Chunked  // Smaller API calls
            } else {
                EmbeddingStrategy::HighQuality
            }
        }
        CostFeeling::Concerned | CostFeeling::Anxious => {
            // Budget pressure - use cheaper options
            EmbeddingStrategy::LocalModel  // Free but lower quality
        }
        CostFeeling::Panicked => {
            // Over budget - refuse expensive operations
            EmbeddingStrategy::CachedOnly  // Only return cached embeddings
        }
    }
}

enum EmbeddingStrategy {
    Default,
    HighQuality,
    Chunked,
    LocalModel,
    CachedOnly,
}
```

---

*Part of the AgenticOS ecosystem by Agentra Labs*
