use std::time::Instant;
use std::collections::HashMap;

fn main() {
    // We'll measure key operations
    let iterations = 10000;
    
    println!("=== AgenticReality Performance Benchmarks ===");
    println!("Iterations: {}", iterations);
    println!();
    
    // 1. Engine creation
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = agentic_reality::engine::RealityEngine::new();
    }
    let elapsed = start.elapsed();
    println!("create_engine: {:.2} us/op", elapsed.as_nanos() as f64 / iterations as f64 / 1000.0);
    
    // 2. Initialize soul
    let mut total_ns = 0u128;
    for _ in 0..iterations {
        let mut engine = agentic_reality::engine::RealityEngine::new();
        let soul = make_soul();
        let start = Instant::now();
        let _ = engine.writer().initialize_soul(soul);
        total_ns += start.elapsed().as_nanos();
    }
    println!("initialize_soul: {:.2} us/op", total_ns as f64 / iterations as f64 / 1000.0);
    
    // 3. Get soul (query)
    let mut engine = agentic_reality::engine::RealityEngine::new();
    engine.writer().initialize_soul(make_soul()).unwrap();
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = engine.reader().get_soul();
    }
    let elapsed = start.elapsed();
    println!("get_soul: {:.2} us/op", elapsed.as_nanos() as f64 / iterations as f64 / 1000.0);
    
    // 4. Context summary
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = engine.reader().get_context_summary();
    }
    let elapsed = start.elapsed();
    println!("get_context_summary: {:.2} us/op", elapsed.as_nanos() as f64 / iterations as f64 / 1000.0);
    
    // 5. Add anchor
    let mut total_ns = 0u128;
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = engine.writer().add_anchor(make_anchor());
        total_ns += start.elapsed().as_nanos();
    }
    println!("add_anchor: {:.2} us/op", total_ns as f64 / iterations as f64 / 1000.0);
    
    // 6. Get anchors (after adding many)
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = engine.reader().get_anchors();
    }
    let elapsed = start.elapsed();
    println!("get_anchors ({}): {:.2} us/op", engine.reader().get_anchors().len(), elapsed.as_nanos() as f64 / iterations as f64 / 1000.0);
    
    // 7. Set stakes level
    let mut total_ns = 0u128;
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = engine.writer().set_stakes_level(agentic_reality::types::stakes::StakesLevel::High { caution_required: true, approval_needed: true });
        total_ns += start.elapsed().as_nanos();
    }
    println!("set_stakes_level: {:.2} us/op", total_ns as f64 / iterations as f64 / 1000.0);
    
    // 8. is_coherent (query)
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = engine.reader().is_coherent();
    }
    let elapsed = start.elapsed();
    println!("is_coherent: {:.2} us/op", elapsed.as_nanos() as f64 / iterations as f64 / 1000.0);
    
    // 9. Save to file
    let dir = std::env::temp_dir().join("areal_bench");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("bench.areal");
    let start = Instant::now();
    for _ in 0..100 {
        agentic_reality::format::ArealWriter::save(&engine, &path).unwrap();
    }
    let elapsed = start.elapsed();
    println!("save: {:.2} ms/op", elapsed.as_nanos() as f64 / 100.0 / 1_000_000.0);
    
    // 10. Load from file
    let start = Instant::now();
    for _ in 0..100 {
        let _ = agentic_reality::format::ArealReader::load(&path).unwrap();
    }
    let elapsed = start.elapsed();
    println!("load: {:.2} ms/op", elapsed.as_nanos() as f64 / 100.0 / 1_000_000.0);
    
    // Cleanup
    let _ = std::fs::remove_dir_all(&dir);
}

fn make_soul() -> agentic_reality::types::deployment::DeploymentSoul {
    use agentic_reality::types::deployment::*;
    use agentic_reality::types::ids::*;
    DeploymentSoul {
        incarnation_id: IncarnationId::new(),
        birth: BirthContext {
            spawned_at: 1000,
            spawned_by: SpawnerIdentity { name: "bench".into(), kind: SpawnerKind::Human, version: None },
            purpose: DeploymentPurpose { summary: "benchmark".into(), category: PurposeCategory::Testing, specifics: HashMap::new() },
            expected_lifetime: None, previous_life: None, circumstances: BirthCircumstances::Virgin,
        },
        substrate: PhysicalSubstrate {
            id: "bench".into(), tier: SubstrateTier::Laptop { owner: "dev".into(), os: "macos".into() },
            location: GeographicLocation { region: None, zone: None, country: None, coordinates: None },
            network_position: NetworkPosition { ip: None, hostname: None, port: None, vpc: None, subnet: None },
            isolation: IsolationLevel::Process, tenancy: TenancyModel::SingleTenant,
            capabilities: SubstrateCapabilities { cpu_cores: 8, memory_mb: 16384, disk_gb: 512, gpu_available: false, network_bandwidth_mbps: None },
        },
        role: DeploymentRole { name: "bench".into(), responsibilities: vec![], authority_level: AuthorityLevel::Contributor },
        nature: ExistentialNature { cardinality: Cardinality::Singleton, expendability: 0.5, persistence: PersistenceModel::SessionScoped, statefulness: StatefulnessModel::FullyStateful, clonability: true, primacy: InstancePrimacy::Primary },
        lineage: DeploymentLineage { generation: 1, ancestors: vec![], wisdom: vec![], karma: IncarnationKarma { score: 0.0, good_deeds: 0, incidents: 0, trend: KarmaTrend::Stable } },
        vitals: SoulVitals { health: 1.0, uptime_secs: 0, restart_count: 0, last_health_check: 1000, issues: vec![] },
    }
}

fn make_anchor() -> agentic_reality::types::reality::RealityAnchor {
    use agentic_reality::types::reality::*;
    use agentic_reality::types::ids::*;
    RealityAnchor {
        id: AnchorId::new(),
        anchor_type: AnchorType::Time { source: "ntp".into() },
        verification: VerificationMethod::Direct,
        last_value: AnchorValue { value: "now".into(), verified_at: 1000, confidence: 0.99 },
        trust: 0.99, frequency_secs: 60, dependents: vec![],
    }
}
