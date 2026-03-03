//! Basic usage of AgenticReality.
//!
//! Demonstrates creating a reality engine, initializing a deployment soul,
//! sensing the environment, and querying state.
//!
//! Run with:
//!   cargo run --example basic_usage

use std::collections::HashMap;
use std::time::Duration;

use agentic_reality::engine::RealityEngine;
use agentic_reality::types::deployment::*;
use agentic_reality::types::environment::*;

fn main() {
    println!("=== AgenticReality Basic Usage ===\n");

    // -----------------------------------------------------------------------
    // 1. Create a new reality engine
    // -----------------------------------------------------------------------
    let mut engine = RealityEngine::new();
    assert!(!engine.is_initialized());
    println!("Created empty reality engine.");

    // -----------------------------------------------------------------------
    // 2. Initialize the deployment soul
    // -----------------------------------------------------------------------
    let soul = DeploymentSoul {
        incarnation_id: agentic_reality::IncarnationId::from_context("example-agent-v1"),
        birth: BirthContext {
            spawned_at: chrono::Utc::now().timestamp(),
            spawned_by: SpawnerIdentity {
                name: "example-runner".into(),
                kind: SpawnerKind::Human,
                version: Some("1.0.0".into()),
            },
            purpose: DeploymentPurpose {
                summary: "Demonstrate AgenticReality basic usage".into(),
                category: PurposeCategory::Development,
                specifics: HashMap::new(),
            },
            expected_lifetime: Some(Duration::from_secs(3600)),
            previous_life: None,
            circumstances: BirthCircumstances::Virgin,
        },
        substrate: PhysicalSubstrate {
            id: "local-dev".into(),
            tier: SubstrateTier::Laptop {
                owner: "developer".into(),
                os: std::env::consts::OS.into(),
            },
            location: GeographicLocation {
                region: None,
                zone: None,
                country: None,
                coordinates: None,
            },
            network_position: NetworkPosition {
                ip: None,
                hostname: Some("localhost".into()),
                port: None,
                vpc: None,
                subnet: None,
            },
            isolation: IsolationLevel::Process,
            tenancy: TenancyModel::SingleTenant,
            capabilities: SubstrateCapabilities {
                cpu_cores: num_cpus(),
                memory_mb: 16384,
                disk_gb: 512,
                gpu_available: false,
                network_bandwidth_mbps: None,
            },
        },
        role: DeploymentRole {
            name: "example-agent".into(),
            responsibilities: vec!["demonstrate API".into()],
            authority_level: AuthorityLevel::ReadOnly,
        },
        nature: ExistentialNature {
            cardinality: Cardinality::Singleton,
            expendability: 1.0,
            persistence: PersistenceModel::Ephemeral,
            statefulness: StatefulnessModel::Stateless,
            clonability: true,
            primacy: InstancePrimacy::Primary,
        },
        lineage: DeploymentLineage {
            generation: 1,
            ancestors: vec![],
            wisdom: vec![],
            karma: IncarnationKarma {
                score: 0.0,
                good_deeds: 0,
                incidents: 0,
                trend: KarmaTrend::Stable,
            },
        },
        vitals: SoulVitals {
            health: 1.0,
            uptime_secs: 0,
            restart_count: 0,
            last_health_check: chrono::Utc::now().timestamp(),
            issues: vec![],
        },
    };

    {
        let mut writer = engine.writer();
        let incarnation_id = writer.initialize_soul(soul).expect("soul initialization");
        println!("Initialized deployment soul: {}", incarnation_id);
    }
    assert!(engine.is_initialized());

    // -----------------------------------------------------------------------
    // 3. Sense the environment
    // -----------------------------------------------------------------------
    let medium = EnvironmentMedium {
        environment_type: EnvironmentType::Development {
            developer: "example-user".into(),
            local: true,
        },
        current_state: EnvironmentState {
            health: EnvironmentHealth {
                score: 0.95,
                components: HashMap::new(),
            },
            pressure: EnvironmentPressure {
                level: 0.1,
                source: "none".into(),
                trend: PressureTrend::Stable,
            },
            stability: StabilityAssessment {
                score: 0.98,
                window_secs: 300,
                volatility: 0.02,
            },
            incidents: vec![],
            degradations: vec![],
            mood: EnvironmentMood::Calm,
            last_sensed: chrono::Utc::now().timestamp(),
        },
        properties: EnvironmentProperties {
            name: "local-dev".into(),
            version: Some("0.1.0".into()),
            config: HashMap::new(),
            labels: HashMap::new(),
        },
        physics: EnvironmentPhysics {
            rate_limits: vec![],
            cost_constraints: vec![],
            time_constraints: vec![],
            quotas: vec![],
            permissions: vec!["read".into(), "write".into()],
            forbidden_actions: vec![],
            compliance: vec![],
        },
        inhabitants: vec!["example-agent".into()],
        boundaries: vec![],
        weather_history: vec![],
    };

    {
        let mut writer = engine.writer();
        writer.sense_environment(medium).expect("environment sensing");
    }
    println!("Environment sensed: mood = calm");

    // -----------------------------------------------------------------------
    // 4. Query state
    // -----------------------------------------------------------------------
    let reader = engine.reader();

    let soul = reader.get_soul().expect("soul query");
    println!("\nDeployment Soul:");
    println!("  Incarnation: {}", soul.incarnation_id);
    println!("  Role: {}", soul.role.name);
    println!("  Health: {:.0}%", soul.vitals.health * 100.0);

    let mood = reader.get_mood().expect("mood query");
    println!("  Environment mood: {}", mood);

    let summary = reader.get_context_summary();
    println!("\nContext Summary:");
    println!("  Substrate: {:?}", summary.substrate_tier);
    println!("  Environment: {:?}", summary.environment_type);
    println!("  Mood: {:?}", summary.environment_mood);

    println!("\n=== Done ===");
}

fn num_cpus() -> u32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(1)
}
