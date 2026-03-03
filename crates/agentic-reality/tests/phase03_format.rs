//! Phase 03: .areal file format tests.

use agentic_reality::engine::RealityEngine;
use agentic_reality::format::{ArealReader, ArealWriter};
use agentic_reality::types::deployment::*;
use agentic_reality::types::environment::*;
use agentic_reality::types::ids::*;
use std::collections::HashMap;

fn make_soul() -> DeploymentSoul {
    DeploymentSoul {
        incarnation_id: IncarnationId::new(),
        birth: BirthContext {
            spawned_at: 1000,
            spawned_by: SpawnerIdentity {
                name: "test".into(),
                kind: SpawnerKind::Human,
                version: None,
            },
            purpose: DeploymentPurpose {
                summary: "test".into(),
                category: PurposeCategory::Testing,
                specifics: HashMap::new(),
            },
            expected_lifetime: None,
            previous_life: None,
            circumstances: BirthCircumstances::Virgin,
        },
        substrate: PhysicalSubstrate {
            id: "test".into(),
            tier: SubstrateTier::Laptop {
                owner: "dev".into(),
                os: "linux".into(),
            },
            location: GeographicLocation {
                region: None,
                zone: None,
                country: None,
                coordinates: None,
            },
            network_position: NetworkPosition {
                ip: None,
                hostname: None,
                port: None,
                vpc: None,
                subnet: None,
            },
            isolation: IsolationLevel::Process,
            tenancy: TenancyModel::SingleTenant,
            capabilities: SubstrateCapabilities {
                cpu_cores: 4,
                memory_mb: 8192,
                disk_gb: 256,
                gpu_available: false,
                network_bandwidth_mbps: None,
            },
        },
        role: DeploymentRole {
            name: "test".into(),
            responsibilities: vec![],
            authority_level: AuthorityLevel::Contributor,
        },
        nature: ExistentialNature {
            cardinality: Cardinality::Singleton,
            expendability: 0.5,
            persistence: PersistenceModel::SessionScoped,
            statefulness: StatefulnessModel::FullyStateful,
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
            last_health_check: 1000,
            issues: vec![],
        },
    }
}

#[test]
fn test_save_and_load_empty_engine() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.areal");
    let engine = RealityEngine::new();
    ArealWriter::save(&engine, &path).unwrap();
    let loaded = ArealReader::load(&path).unwrap();
    assert!(!loaded.is_initialized());
}

#[test]
fn test_save_and_load_with_soul() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.areal");
    let mut engine = RealityEngine::new();
    let soul = make_soul();
    let id = engine.writer().initialize_soul(soul).unwrap();
    ArealWriter::save(&engine, &path).unwrap();
    let loaded = ArealReader::load(&path).unwrap();
    assert!(loaded.is_initialized());
    assert_eq!(loaded.incarnation_id().unwrap(), id);
}

#[test]
fn test_save_and_load_with_environment() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.areal");
    let mut engine = RealityEngine::new();
    engine
        .writer()
        .sense_environment(EnvironmentMedium {
            environment_type: EnvironmentType::Testing {
                test_type: "unit".into(),
                isolation: true,
            },
            current_state: EnvironmentState {
                health: EnvironmentHealth {
                    score: 1.0,
                    components: HashMap::new(),
                },
                pressure: EnvironmentPressure {
                    level: 0.0,
                    source: "".into(),
                    trend: PressureTrend::Stable,
                },
                stability: StabilityAssessment {
                    score: 1.0,
                    window_secs: 300,
                    volatility: 0.0,
                },
                incidents: vec![],
                degradations: vec![],
                mood: EnvironmentMood::Calm,
                last_sensed: 1000,
            },
            properties: EnvironmentProperties {
                name: "test".into(),
                version: None,
                config: HashMap::new(),
                labels: HashMap::new(),
            },
            physics: EnvironmentPhysics {
                rate_limits: vec![],
                cost_constraints: vec![],
                time_constraints: vec![],
                quotas: vec![],
                permissions: vec![],
                forbidden_actions: vec![],
                compliance: vec![],
            },
            inhabitants: vec!["agent".into()],
            boundaries: vec![],
            weather_history: vec![],
        })
        .unwrap();
    ArealWriter::save(&engine, &path).unwrap();
    let loaded = ArealReader::load(&path).unwrap();
    let reader = loaded.reader();
    let env = reader.get_environment().unwrap();
    assert!(matches!(
        env.environment_type,
        EnvironmentType::Testing { .. }
    ));
    assert_eq!(env.inhabitants, vec!["agent"]);
}

#[test]
fn test_save_and_load_roundtrip_full() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.areal");
    let mut engine = RealityEngine::new();
    engine.writer().initialize_soul(make_soul()).unwrap();
    engine
        .writer()
        .set_stakes_level(agentic_reality::types::stakes::StakesLevel::High {
            caution_required: true,
            approval_needed: true,
        })
        .unwrap();
    engine
        .writer()
        .run_coherence_check(agentic_reality::types::coherence::CoherenceCheck {
            check_type: agentic_reality::types::coherence::CoherenceCheckType::TimeConsistency,
            passed: true,
            details: "ok".into(),
            timestamp: 1000,
            duration_ms: 5,
        })
        .unwrap();
    ArealWriter::save(&engine, &path).unwrap();
    let loaded = ArealReader::load(&path).unwrap();
    assert!(loaded.is_initialized());
    assert!(loaded.reader().is_coherent());
    assert_eq!(
        loaded.reader().get_stakes_level().unwrap().to_string(),
        "high"
    );
}

#[test]
fn test_load_nonexistent_file() {
    let result = ArealReader::load(std::path::Path::new("/nonexistent/test.areal"));
    assert!(result.is_err());
}

#[test]
fn test_load_invalid_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bad.areal");
    std::fs::write(&path, b"not a valid areal file").unwrap();
    let result = ArealReader::load(&path);
    assert!(result.is_err());
}

#[test]
fn test_header_read_write() {
    use agentic_reality::format::header::ArealHeader;
    let header = ArealHeader::new([0u8; 16]);
    let mut buf = Vec::new();
    header.write_to(&mut buf).unwrap();
    assert_eq!(buf.len(), 256);
    let parsed = ArealHeader::read_from(&buf).unwrap();
    assert_eq!(parsed.version, header.version);
}

#[test]
fn test_footer_read_write() {
    use agentic_reality::format::footer::ArealFooter;
    let footer = ArealFooter::new([42u8; 32], 8);
    let mut buf = Vec::new();
    footer.write_to(&mut buf);
    assert_eq!(buf.len(), 64);
    let parsed = ArealFooter::read_from(&buf).unwrap();
    assert_eq!(parsed.sections_verified, 8);
    assert_eq!(parsed.global_checksum[0], 42);
}

#[test]
fn test_section_entry_read_write() {
    use agentic_reality::format::sections::{SectionEntry, SectionType, SECTION_ENTRY_SIZE};
    let entry = SectionEntry {
        section_type: SectionType::Deployment,
        flags: 0,
        offset: 256,
        length: 1024,
        uncompressed_length: 1024,
        checksum: [1, 2, 3, 4, 5, 6, 7, 8],
    };
    let mut buf = Vec::new();
    entry.write_to(&mut buf);
    assert_eq!(buf.len(), SECTION_ENTRY_SIZE);
    let parsed = SectionEntry::read_from(&buf).unwrap();
    assert_eq!(parsed.section_type, SectionType::Deployment);
    assert_eq!(parsed.offset, 256);
    assert_eq!(parsed.length, 1024);
}

#[test]
fn test_atomic_write_no_partial_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("atomic.areal");
    let engine = RealityEngine::new();
    ArealWriter::save(&engine, &path).unwrap();
    // Verify no .tmp file left
    let tmp_path = dir.path().join("atomic.areal.tmp");
    assert!(!tmp_path.exists());
    // Original file should exist
    assert!(path.exists());
}
