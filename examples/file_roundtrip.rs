//! File roundtrip example for AgenticReality.
//!
//! Creates a reality engine, populates it with data across multiple domains,
//! saves it to a .areal file, loads it back, and verifies the data survived
//! the roundtrip.
//!
//! Run with:
//!   cargo run --example file_roundtrip

use std::collections::HashMap;
use std::time::Duration;

use agentic_reality::engine::RealityEngine;
use agentic_reality::format::{ArealReader, ArealWriter};
use agentic_reality::types::deployment::*;
use agentic_reality::types::environment::*;
use agentic_reality::IncarnationId;

fn main() {
    println!("=== AgenticReality File Roundtrip Example ===\n");

    // -----------------------------------------------------------------------
    // 1. Create and populate an engine
    // -----------------------------------------------------------------------
    let mut engine = RealityEngine::new();

    let incarnation_id = IncarnationId::from_context("roundtrip-test-agent");

    // Initialize deployment soul
    let soul = DeploymentSoul {
        incarnation_id,
        birth: BirthContext {
            spawned_at: 1700000000,
            spawned_by: SpawnerIdentity {
                name: "roundtrip-example".into(),
                kind: SpawnerKind::Human,
                version: Some("0.1.0".into()),
            },
            purpose: DeploymentPurpose {
                summary: "Test .areal file roundtrip".into(),
                category: PurposeCategory::Testing,
                specifics: {
                    let mut m = HashMap::new();
                    m.insert("test_id".into(), "roundtrip-001".into());
                    m
                },
            },
            expected_lifetime: Some(Duration::from_secs(60)),
            previous_life: None,
            circumstances: BirthCircumstances::Virgin,
        },
        substrate: PhysicalSubstrate {
            id: "test-substrate".into(),
            tier: SubstrateTier::Laptop {
                owner: "tester".into(),
                os: "linux".into(),
            },
            location: GeographicLocation {
                region: Some("us-east".into()),
                zone: Some("us-east-1a".into()),
                country: Some("US".into()),
                coordinates: Some((40.7128, -74.0060)),
            },
            network_position: NetworkPosition {
                ip: Some("127.0.0.1".into()),
                hostname: Some("test-host".into()),
                port: Some(8080),
                vpc: None,
                subnet: None,
            },
            isolation: IsolationLevel::Process,
            tenancy: TenancyModel::SingleTenant,
            capabilities: SubstrateCapabilities {
                cpu_cores: 8,
                memory_mb: 32768,
                disk_gb: 1024,
                gpu_available: false,
                network_bandwidth_mbps: Some(1000),
            },
        },
        role: DeploymentRole {
            name: "roundtrip-agent".into(),
            responsibilities: vec![
                "save state".into(),
                "load state".into(),
                "verify integrity".into(),
            ],
            authority_level: AuthorityLevel::Contributor,
        },
        nature: ExistentialNature {
            cardinality: Cardinality::Singleton,
            expendability: 0.5,
            persistence: PersistenceModel::Persistent,
            statefulness: StatefulnessModel::FullyStateful,
            clonability: false,
            primacy: InstancePrimacy::Primary,
        },
        lineage: DeploymentLineage {
            generation: 1,
            ancestors: vec![],
            wisdom: vec![IncarnationWisdom {
                lesson: "Always verify file integrity after save".into(),
                learned_from: incarnation_id,
                confidence: 0.95,
                applicable: true,
            }],
            karma: IncarnationKarma {
                score: 10.0,
                good_deeds: 5,
                incidents: 0,
                trend: KarmaTrend::Improving,
            },
        },
        vitals: SoulVitals {
            health: 0.99,
            uptime_secs: 3600,
            restart_count: 0,
            last_health_check: 1700000000,
            issues: vec![],
        },
    };

    {
        let mut writer = engine.writer();
        let id = writer.initialize_soul(soul).expect("initialize soul");
        println!("Initialized soul: {}", id);
    }

    // Sense environment
    let medium = EnvironmentMedium {
        environment_type: EnvironmentType::Testing {
            test_type: "integration".into(),
            isolation: true,
        },
        current_state: EnvironmentState {
            health: EnvironmentHealth {
                score: 1.0,
                components: {
                    let mut m = HashMap::new();
                    m.insert("disk".into(), 0.98);
                    m.insert("network".into(), 1.0);
                    m
                },
            },
            pressure: EnvironmentPressure {
                level: 0.05,
                source: "none".into(),
                trend: PressureTrend::Stable,
            },
            stability: StabilityAssessment {
                score: 1.0,
                window_secs: 600,
                volatility: 0.0,
            },
            incidents: vec![],
            degradations: vec![],
            mood: EnvironmentMood::Calm,
            last_sensed: 1700000000,
        },
        properties: EnvironmentProperties {
            name: "test-env".into(),
            version: Some("0.1.0".into()),
            config: HashMap::new(),
            labels: {
                let mut m = HashMap::new();
                m.insert("purpose".into(), "roundtrip-test".into());
                m
            },
        },
        physics: EnvironmentPhysics {
            rate_limits: vec![],
            cost_constraints: vec![],
            time_constraints: vec![],
            quotas: vec![],
            permissions: vec!["read".into(), "write".into(), "delete".into()],
            forbidden_actions: vec![],
            compliance: vec!["SOC2".into()],
        },
        inhabitants: vec!["roundtrip-agent".into(), "test-harness".into()],
        boundaries: vec!["test-namespace".into()],
        weather_history: vec![WeatherEvent {
            event_type: "deploy".into(),
            severity: 0.1,
            timestamp: 1700000000,
            duration_secs: Some(5),
            description: "Initial deployment".into(),
        }],
    };

    {
        let mut writer = engine.writer();
        writer.sense_environment(medium).expect("sense environment");
    }
    println!("Environment sensed.");

    // -----------------------------------------------------------------------
    // 2. Save to .areal file
    // -----------------------------------------------------------------------
    let tmp_dir = std::env::temp_dir();
    let file_path = tmp_dir.join("roundtrip_example.areal");

    println!("\nSaving to: {}", file_path.display());
    ArealWriter::save(&engine, &file_path).expect("save .areal file");

    let file_size = std::fs::metadata(&file_path)
        .expect("file metadata")
        .len();
    println!("File size: {} bytes", file_size);

    // -----------------------------------------------------------------------
    // 3. Load back from .areal file
    // -----------------------------------------------------------------------
    println!("\nLoading from: {}", file_path.display());
    let loaded_engine = ArealReader::load(&file_path).expect("load .areal file");

    // -----------------------------------------------------------------------
    // 4. Verify roundtrip integrity
    // -----------------------------------------------------------------------
    println!("\nVerifying roundtrip integrity...");

    // Check incarnation ID survived
    let original_id = engine.incarnation_id().expect("original incarnation ID");
    let loaded_id = loaded_engine
        .incarnation_id()
        .expect("loaded incarnation ID");
    assert_eq!(original_id, loaded_id, "incarnation ID mismatch");
    println!("  Incarnation ID: MATCH ({})", loaded_id);

    // Check soul data
    let original_reader = engine.reader();
    let loaded_reader = loaded_engine.reader();

    let original_soul = original_reader.get_soul().expect("original soul");
    let loaded_soul = loaded_reader.get_soul().expect("loaded soul");

    assert_eq!(original_soul.role.name, loaded_soul.role.name);
    println!("  Role name: MATCH ({})", loaded_soul.role.name);

    assert_eq!(
        original_soul.role.responsibilities.len(),
        loaded_soul.role.responsibilities.len()
    );
    println!(
        "  Responsibilities count: MATCH ({})",
        loaded_soul.role.responsibilities.len()
    );

    assert!(
        (original_soul.vitals.health - loaded_soul.vitals.health).abs() < f64::EPSILON
    );
    println!("  Health: MATCH ({:.2})", loaded_soul.vitals.health);

    // Check environment survived
    let original_mood = original_reader.get_mood().expect("original mood");
    let loaded_mood = loaded_reader.get_mood().expect("loaded mood");
    assert_eq!(original_mood, loaded_mood);
    println!("  Environment mood: MATCH ({})", loaded_mood);

    let original_env = original_reader.get_environment().expect("original env");
    let loaded_env = loaded_reader.get_environment().expect("loaded env");
    assert_eq!(original_env.inhabitants.len(), loaded_env.inhabitants.len());
    println!(
        "  Inhabitants count: MATCH ({})",
        loaded_env.inhabitants.len()
    );

    assert_eq!(
        original_env.weather_history.len(),
        loaded_env.weather_history.len()
    );
    println!(
        "  Weather history count: MATCH ({})",
        loaded_env.weather_history.len()
    );

    // Check lineage wisdom
    let original_wisdom = original_reader.get_wisdom().expect("original wisdom");
    let loaded_wisdom = loaded_reader.get_wisdom().expect("loaded wisdom");
    assert_eq!(original_wisdom.len(), loaded_wisdom.len());
    println!("  Wisdom entries: MATCH ({})", loaded_wisdom.len());

    // Check context summary
    let original_summary = original_reader.get_context_summary();
    let loaded_summary = loaded_reader.get_context_summary();
    assert_eq!(
        original_summary.environment_mood,
        loaded_summary.environment_mood
    );
    println!(
        "  Context summary mood: MATCH ({:?})",
        loaded_summary.environment_mood
    );

    // -----------------------------------------------------------------------
    // 5. Clean up
    // -----------------------------------------------------------------------
    std::fs::remove_file(&file_path).expect("cleanup temp file");
    println!("\nCleaned up temp file.");

    println!("\n=== Roundtrip verification PASSED ===");
}
