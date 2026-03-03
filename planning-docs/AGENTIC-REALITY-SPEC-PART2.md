# AGENTIC REALITY SPECIFICATION — PART 2

> **Specs Covered:** SPEC-05 through SPEC-08
> **Sister:** #10 of 25 (The Ground)
> **Continues from:** Part 1

---

# SPEC-05: WRITE ENGINE

## 5.1 Write Operations Overview

```
WRITE OPERATIONS BY DOMAIN:
═══════════════════════════

DEPLOYMENT (12 operations)
├── initialize_soul          Create deployment soul at birth
├── update_substrate          Substrate changes detected
├── record_birth              Record birth context
├── update_vitals             Update soul vitals
├── record_death              Record death (for incarnation memory)
├── add_past_life             Add past incarnation to memory
├── update_lineage            Update deployment lineage
├── set_role                  Set deployment role
├── set_nature                Set existential nature
├── update_cardinality        Cardinality changed (scale event)
├── record_wisdom             Record lesson learned
└── update_karma              Update karmic balance

ENVIRONMENT (10 operations)
├── sense_environment         Initial environment sensing
├── update_environment_state  Environment state changed
├── update_mood               Environment mood changed
├── record_incident           Record active incident
├── clear_incident            Clear resolved incident
├── update_physics            Environment physics changed
├── record_weather            Record environmental change
├── update_fingerprint        Context fingerprint changed
├── add_inhabitant            Add entity to environment
└── remove_inhabitant         Remove entity from environment

RESOURCE (14 operations)
├── sense_resources           Initial resource sensing
├── update_mind               Memory state changed
├── update_energy             CPU state changed
├── update_reach              Network state changed
├── update_storage            Storage state changed
├── update_visual             GPU state changed
├── add_sensation             Add resource sensation
├── clear_sensation           Clear resource sensation
├── update_pressure_gradient  Pressure gradient changed
├── discover_capability       Capability discovered
├── lose_capability           Capability lost
├── update_cost               Cost state changed
├── update_capacity_intuition Capacity intuition update
└── set_budget                Set budget constraints

REALITY (12 operations)
├── set_reality_layer         Set current reality layer
├── update_layer_status       Layer status changed
├── update_freshness          Data freshness changed
├── add_anchor                Add reality anchor
├── remove_anchor             Remove reality anchor
├── verify_anchor             Verify anchor (update trust)
├── record_anchor_drift       Record drift from anchor
├── detect_hallucination      Record detected hallucination
├── clear_hallucination       Clear resolved hallucination
├── add_unverified_claim      Add claim pending verification
├── verify_claim              Verify pending claim
└── update_grounding          Update grounding status

TOPOLOGY (14 operations)
├── set_position              Set topology position
├── add_upstream              Add upstream entity
├── remove_upstream           Remove upstream entity
├── add_downstream            Add downstream dependency
├── remove_downstream         Remove dependency
├── update_downstream_health  Dependency health changed
├── add_sibling               Add sibling entity
├── remove_sibling            Remove sibling
├── update_sibling_state      Sibling state changed
├── add_observer              Add observer
├── remove_observer           Remove observer
├── update_topology_health    Overall topology health
├── record_mesh_event         Record service mesh event
└── update_graph              Update full topology graph

TEMPORAL (10 operations)
├── ground_time               Ground to time source
├── update_temporal_context   Temporal context changed
├── add_causal_event          Add event to causality graph
├── link_causality            Link causal relationship
├── add_deadline              Add deadline
├── remove_deadline           Remove deadline
├── update_deadline           Update deadline status
├── add_timeline              Add timeline
├── record_timeline_event     Record event in timeline
└── resolve_timeline_conflict Resolve timeline conflict

STAKES (10 operations)
├── set_stakes_level          Set stakes level
├── add_consequence           Add potential consequence
├── remove_consequence        Remove consequence
├── add_irreversible_action   Mark action as irreversible
├── update_safety_margins     Update safety margins
├── add_guardrail             Add guardrail
├── remove_guardrail          Remove guardrail
├── update_risk_field         Update risk perception
├── update_blast_radius       Update blast radius awareness
└── record_consequence        Record actual consequence

COHERENCE (8 operations)
├── run_coherence_check       Run coherence check
├── record_violation          Record coherence violation
├── resolve_violation         Mark violation resolved
├── begin_transition          Begin context transition
├── advance_transition        Advance transition phase
├── complete_transition       Complete transition
├── abort_transition          Abort transition
└── rollback_transition       Rollback failed transition

TOTAL: ~90 write operations
```

## 5.2 Core Write Engine

```rust
//! src/engine/write.rs

use crate::types::*;
use crate::storage::*;
use crate::validation::*;

/// Write engine for reality state mutations
pub struct WriteEngine {
    /// Deployment store
    deployment_store: DeploymentStore,
    
    /// Environment store
    environment_store: EnvironmentStore,
    
    /// Resource store
    resource_store: ResourceStore,
    
    /// Reality store
    reality_store: RealityStore,
    
    /// Topology store
    topology_store: TopologyStore,
    
    /// Temporal store
    temporal_store: TemporalStore,
    
    /// Stakes store
    stakes_store: StakesStore,
    
    /// Coherence store
    coherence_store: CoherenceStore,
    
    /// Indexes
    indexes: RealityIndexes,
    
    /// Dirty flag
    dirty: bool,
    
    /// Write-ahead log
    wal: Option<WriteAheadLog>,
}

impl WriteEngine {
    pub fn new() -> Self {
        Self {
            deployment_store: DeploymentStore::new(),
            environment_store: EnvironmentStore::new(),
            resource_store: ResourceStore::new(),
            reality_store: RealityStore::new(),
            topology_store: TopologyStore::new(),
            temporal_store: TemporalStore::new(),
            stakes_store: StakesStore::new(),
            coherence_store: CoherenceStore::new(),
            indexes: RealityIndexes::new(),
            dirty: false,
            wal: None,
        }
    }
    
    /// Mark state as modified
    fn mark_dirty(&mut self) {
        self.dirty = true;
    }
    
    /// Check if state needs saving
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    
    /// Clear dirty flag (after save)
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}
```

## 5.3 Deployment Write Operations

```rust
//! src/engine/write/deployment.rs

impl WriteEngine {
    /// Initialize deployment soul at birth
    pub fn initialize_soul(&mut self, request: InitializeSoulRequest) -> Result<DeploymentSoul> {
        // Validate request
        Validator::validate_initialize_soul(&request)?;
        
        let incarnation_id = IncarnationId::new();
        let now = Timestamp::now();
        
        // Detect substrate if not provided
        let substrate = request.substrate
            .unwrap_or_else(|| self.detect_substrate());
        
        // Determine birth circumstances
        let circumstances = request.circumstances
            .unwrap_or(BirthCircumstances::Virgin);
        
        let soul = DeploymentSoul {
            incarnation_id,
            birth: BirthContext {
                spawned_at: now,
                spawned_by: request.spawned_by,
                purpose: request.purpose,
                expected_lifetime: request.expected_lifetime,
                previous_life: match &circumstances {
                    BirthCircumstances::Resurrected { previous, .. } => Some(*previous),
                    BirthCircumstances::ScaledFrom { parent } => Some(*parent),
                    BirthCircumstances::Forked { from, .. } => Some(*from),
                    _ => None,
                },
                circumstances,
            },
            substrate,
            role: request.role.unwrap_or(DeploymentRole::Unknown),
            nature: request.nature.unwrap_or_else(|| ExistentialNature {
                cardinality: Cardinality::Singleton,
                expendability: 0.5,
                persistence: PersistenceModel::Ephemeral,
                statefulness: StatefulnessModel::Stateless,
                clonability: true,
                primacy: InstancePrimacy::Unknown,
            }),
            lineage: DeploymentLineage::new(),
            vitals: SoulVitals {
                health: 1.0,
                uptime: Duration::ZERO,
                restart_count: 0,
                last_health_check: now,
                issues: vec![],
            },
        };
        
        self.deployment_store.set_soul(soul.clone());
        self.indexes.index_deployment(&soul);
        self.mark_dirty();
        
        // Log to WAL if enabled
        if let Some(wal) = &mut self.wal {
            wal.log(WalEntry::InitializeSoul { 
                incarnation_id,
                timestamp: now,
            })?;
        }
        
        Ok(soul)
    }
    
    /// Detect physical substrate
    fn detect_substrate(&self) -> PhysicalSubstrate {
        let mut substrate = PhysicalSubstrate {
            id: SubstrateId::new(),
            tier: SubstrateTier::Unknown { clues: vec![] },
            location: GeographicLocation::Unknown,
            network_position: NetworkPosition::Unknown,
            isolation: IsolationLevel::Unknown,
            tenancy: TenancyModel::Unknown,
            capabilities: SubstrateCapabilities::default(),
        };
        
        // Detect tier from environment clues
        substrate.tier = self.detect_substrate_tier();
        
        // Detect location
        substrate.location = self.detect_location();
        
        // Detect capabilities
        substrate.capabilities = self.detect_capabilities();
        
        substrate
    }
    
    fn detect_substrate_tier(&self) -> SubstrateTier {
        // Check for cloud provider metadata
        if let Ok(provider) = self.detect_cloud_provider() {
            return SubstrateTier::Cloud {
                provider,
                instance_type: self.detect_instance_type().unwrap_or_default(),
                region: self.detect_region().unwrap_or_default(),
            };
        }
        
        // Check for container
        if std::path::Path::new("/.dockerenv").exists() {
            return SubstrateTier::Container {
                runtime: ContainerRuntime::Docker,
                orchestrator: self.detect_orchestrator(),
            };
        }
        
        // Check for serverless
        if std::env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok() {
            return SubstrateTier::Serverless {
                provider: "AWS Lambda".to_string(),
                memory_mb: std::env::var("AWS_LAMBDA_FUNCTION_MEMORY_SIZE")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(128),
                timeout: Duration::from_secs(
                    std::env::var("AWS_LAMBDA_FUNCTION_TIMEOUT")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(30)
                ),
            };
        }
        
        // Check for browser/WASM
        #[cfg(target_arch = "wasm32")]
        {
            return SubstrateTier::Browser {
                browser: BrowserType::Unknown,
                version: String::new(),
            };
        }
        
        // Default to unknown with clues
        SubstrateTier::Unknown {
            clues: self.gather_substrate_clues(),
        }
    }
    
    /// Update soul vitals
    pub fn update_vitals(&mut self, vitals: SoulVitals) -> Result<()> {
        let soul = self.deployment_store.get_soul_mut()
            .ok_or(Error::NoDeploymentSoul)?;
        
        soul.vitals = vitals;
        soul.vitals.last_health_check = Timestamp::now();
        
        self.mark_dirty();
        Ok(())
    }
    
    /// Record death (for incarnation memory)
    pub fn record_death(&mut self, cause: DeathCause, graceful: bool) -> Result<DeathRecord> {
        let soul = self.deployment_store.get_soul()
            .ok_or(Error::NoDeploymentSoul)?;
        
        let death = DeathRecord {
            timestamp: Timestamp::now(),
            cause,
            graceful,
            data_loss: self.assess_data_loss(),
            trigger: self.identify_death_trigger(),
            preventable: self.was_death_preventable(&cause),
            lessons: self.extract_death_lessons(&cause),
        };
        
        // Store in incarnation memory for next life
        self.deployment_store.add_death_record(soul.incarnation_id, death.clone());
        self.mark_dirty();
        
        Ok(death)
    }
    
    /// Add past life to incarnation memory
    pub fn add_past_life(&mut self, past: PastIncarnation) -> Result<()> {
        Validator::validate_past_incarnation(&past)?;
        
        self.deployment_store.add_past_life(past.clone());
        self.indexes.index_past_life(&past);
        self.mark_dirty();
        
        Ok(())
    }
    
    /// Record wisdom from experience
    pub fn record_wisdom(&mut self, wisdom: WisdomEntry) -> Result<()> {
        self.deployment_store.add_wisdom(wisdom);
        self.mark_dirty();
        Ok(())
    }
    
    /// Update cardinality (scale event)
    pub fn update_cardinality(&mut self, cardinality: Cardinality) -> Result<()> {
        let soul = self.deployment_store.get_soul_mut()
            .ok_or(Error::NoDeploymentSoul)?;
        
        soul.nature.cardinality = cardinality;
        self.mark_dirty();
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct InitializeSoulRequest {
    pub spawned_by: SpawnerIdentity,
    pub purpose: DeploymentPurpose,
    pub expected_lifetime: Option<Duration>,
    pub circumstances: Option<BirthCircumstances>,
    pub substrate: Option<PhysicalSubstrate>,
    pub role: Option<DeploymentRole>,
    pub nature: Option<ExistentialNature>,
}
```

## 5.4 Environment Write Operations

```rust
//! src/engine/write/environment.rs

impl WriteEngine {
    /// Initial environment sensing
    pub fn sense_environment(&mut self) -> Result<EnvironmentMedium> {
        let environment_type = self.detect_environment_type();
        let state = self.sense_environment_state();
        let physics = self.sense_environment_physics();
        
        let environment = EnvironmentMedium {
            environment_type,
            current_state: state,
            properties: self.sense_environment_properties(),
            physics,
            inhabitants: self.discover_inhabitants(),
            boundaries: self.detect_boundaries(),
            weather_history: vec![],
        };
        
        self.environment_store.set_environment(environment.clone());
        self.update_context_fingerprint()?;
        self.mark_dirty();
        
        Ok(environment)
    }
    
    fn detect_environment_type(&self) -> EnvironmentType {
        // Check explicit environment variable
        if let Ok(env) = std::env::var("ENVIRONMENT") {
            match env.to_lowercase().as_str() {
                "production" | "prod" => {
                    return EnvironmentType::Production {
                        tier: self.detect_production_tier(),
                        region: self.detect_region().unwrap_or_default(),
                        criticality: self.detect_criticality(),
                    };
                }
                "staging" | "stage" => {
                    return EnvironmentType::Staging {
                        mirrors_production: true,
                        data_freshness: Duration::from_secs(3600),
                    };
                }
                "development" | "dev" => {
                    return EnvironmentType::Development {
                        developer: std::env::var("USER").ok(),
                        local: true,
                    };
                }
                "test" | "testing" => {
                    return EnvironmentType::Testing {
                        test_type: TestType::Integration,
                        isolation: TestIsolation::Full,
                    };
                }
                _ => {}
            }
        }
        
        // Check for CI/CD
        if std::env::var("CI").is_ok() || std::env::var("GITHUB_ACTIONS").is_ok() {
            return EnvironmentType::Pipeline {
                pipeline_id: std::env::var("GITHUB_RUN_ID")
                    .or_else(|_| std::env::var("BUILD_ID"))
                    .unwrap_or_default(),
                stage: PipelineStage::Build,
            };
        }
        
        // Default to unknown
        EnvironmentType::Unknown {
            clues: self.gather_environment_clues(),
        }
    }
    
    fn sense_environment_state(&self) -> EnvironmentState {
        EnvironmentState {
            health: EnvironmentHealth::Unknown,
            pressure: EnvironmentPressure::Normal,
            stability: StabilityAssessment::Unknown,
            incidents: vec![],
            degradations: vec![],
            mood: EnvironmentMood::Calm,
            last_sensed: Timestamp::now(),
        }
    }
    
    /// Update environment state
    pub fn update_environment_state(&mut self, state: EnvironmentState) -> Result<()> {
        let env = self.environment_store.get_environment_mut()
            .ok_or(Error::NoEnvironment)?;
        
        let old_mood = env.current_state.mood;
        env.current_state = state;
        
        // Record weather change if mood changed significantly
        if old_mood != env.current_state.mood {
            env.weather_history.push(EnvironmentChange {
                timestamp: Timestamp::now(),
                change_type: EnvironmentChangeType::MoodChange {
                    from: old_mood,
                    to: env.current_state.mood,
                },
                cause: None,
            });
        }
        
        self.mark_dirty();
        Ok(())
    }
    
    /// Update environment mood
    pub fn update_mood(&mut self, mood: EnvironmentMood, cause: Option<String>) -> Result<()> {
        let env = self.environment_store.get_environment_mut()
            .ok_or(Error::NoEnvironment)?;
        
        let old_mood = env.current_state.mood;
        env.current_state.mood = mood;
        
        env.weather_history.push(EnvironmentChange {
            timestamp: Timestamp::now(),
            change_type: EnvironmentChangeType::MoodChange {
                from: old_mood,
                to: mood,
            },
            cause,
        });
        
        self.mark_dirty();
        Ok(())
    }
    
    /// Record active incident
    pub fn record_incident(&mut self, incident: ActiveIncident) -> Result<()> {
        Validator::validate_incident(&incident)?;
        
        let env = self.environment_store.get_environment_mut()
            .ok_or(Error::NoEnvironment)?;
        
        env.current_state.incidents.push(incident.clone());
        
        // Auto-update mood based on incident severity
        if incident.severity >= Severity::High {
            env.current_state.mood = EnvironmentMood::Crisis;
        } else if incident.severity >= Severity::Medium {
            env.current_state.mood = EnvironmentMood::Troubled;
        }
        
        self.indexes.index_incident(&incident);
        self.mark_dirty();
        
        Ok(())
    }
    
    /// Clear resolved incident
    pub fn clear_incident(&mut self, incident_id: &str) -> Result<()> {
        let env = self.environment_store.get_environment_mut()
            .ok_or(Error::NoEnvironment)?;
        
        env.current_state.incidents.retain(|i| i.id != incident_id);
        
        // Update mood if no more incidents
        if env.current_state.incidents.is_empty() {
            env.current_state.mood = EnvironmentMood::Recovering;
        }
        
        self.mark_dirty();
        Ok(())
    }
    
    /// Update context fingerprint
    pub fn update_context_fingerprint(&mut self) -> Result<ContextFingerprint> {
        let fingerprint = self.compute_context_fingerprint();
        self.environment_store.set_fingerprint(fingerprint.clone());
        self.mark_dirty();
        Ok(fingerprint)
    }
    
    fn compute_context_fingerprint(&self) -> ContextFingerprint {
        use blake3::Hasher;
        
        let mut components = ContextComponents {
            deployment_hash: [0; 8],
            version_hash: [0; 8],
            environment_hash: [0; 8],
            config_hash: [0; 8],
            resource_hash: [0; 8],
            capability_hash: [0; 8],
            temporal_hash: [0; 8],
            load_hash: [0; 8],
            network_hash: [0; 8],
            dependency_hash: [0; 8],
        };
        
        // Hash deployment
        if let Some(soul) = self.deployment_store.get_soul() {
            let mut h = Hasher::new();
            h.update(soul.incarnation_id.0.as_bytes());
            components.deployment_hash.copy_from_slice(&h.finalize().as_bytes()[..8]);
        }
        
        // Hash environment
        if let Some(env) = self.environment_store.get_environment() {
            let mut h = Hasher::new();
            h.update(format!("{:?}", env.environment_type).as_bytes());
            components.environment_hash.copy_from_slice(&h.finalize().as_bytes()[..8]);
        }
        
        // Hash resources
        if let Some(body) = self.resource_store.get_body() {
            let mut h = Hasher::new();
            h.update(&body.mind.used.0.to_le_bytes());
            h.update(&body.energy.utilization.to_le_bytes());
            components.resource_hash.copy_from_slice(&h.finalize().as_bytes()[..8]);
        }
        
        // Compute overall hash
        let mut overall = Hasher::new();
        overall.update(&components.deployment_hash);
        overall.update(&components.environment_hash);
        overall.update(&components.resource_hash);
        // ... add other components
        
        let hash = overall.finalize();
        
        ContextFingerprint {
            hash: *hash.as_bytes(),
            components,
            captured_at: Timestamp::now(),
            stability: ContextStability::default(),
            similarities: vec![],
        }
    }
}
```

## 5.5 Resource Write Operations

```rust
//! src/engine/write/resource.rs

impl WriteEngine {
    /// Initial resource sensing
    pub fn sense_resources(&mut self) -> Result<ResourceBody> {
        let body = ResourceBody {
            mind: self.sense_mind(),
            energy: self.sense_energy(),
            reach: self.sense_reach(),
            storage: self.sense_storage(),
            visual: self.sense_visual(),
            vitals: BodyVitals::default(),
            sensations: vec![],
        };
        
        self.resource_store.set_body(body.clone());
        self.update_pressure_gradient()?;
        self.mark_dirty();
        
        Ok(body)
    }
    
    fn sense_mind(&self) -> MindCapacity {
        use sysinfo::{System, SystemExt};
        
        let mut sys = System::new_all();
        sys.refresh_memory();
        
        let total = ByteSize(sys.total_memory());
        let used = ByteSize(sys.used_memory());
        let available = ByteSize(sys.available_memory());
        
        let utilization = used.0 as f64 / total.0 as f64;
        let feeling = match utilization {
            u if u < 0.30 => MindFeeling::Clear,
            u if u < 0.60 => MindFeeling::Active,
            u if u < 0.80 => MindFeeling::Crowded,
            u if u < 0.90 => MindFeeling::Strained,
            u if u < 0.95 => MindFeeling::Overwhelmed,
            _ => MindFeeling::Drowning,
        };
        
        MindCapacity {
            total,
            used,
            available,
            feeling,
            pressure: MemoryPressure::from_utilization(utilization),
            largest_free: available, // Approximation
            fragmentation: 0.0, // Would need more sophisticated detection
            swap: self.sense_swap(&sys),
        }
    }
    
    fn sense_energy(&self) -> ProcessingEnergy {
        use sysinfo::{System, SystemExt, CpuExt};
        
        let mut sys = System::new_all();
        sys.refresh_cpu();
        
        let cores = sys.cpus().len() as u32;
        let utilization = sys.global_cpu_info().cpu_usage() as f64 / 100.0;
        
        let feeling = match utilization {
            u if u < 0.30 => EnergyFeeling::Vigorous,
            u if u < 0.50 => EnergyFeeling::Steady,
            u if u < 0.70 => EnergyFeeling::Busy,
            u if u < 0.85 => EnergyFeeling::Strained,
            u if u < 0.95 => EnergyFeeling::Depleted,
            _ => EnergyFeeling::Constrained,
        };
        
        ProcessingEnergy {
            cores,
            utilization,
            feeling,
            load_average: self.get_load_average(),
            burst_available: self.detect_burst_capability(),
            throttled: self.detect_throttling(),
            credits: self.detect_cpu_credits(),
            temperature: self.detect_cpu_temperature(),
        }
    }
    
    fn sense_reach(&self) -> NetworkReach {
        NetworkReach {
            bandwidth: self.detect_bandwidth(),
            utilization: 0.0, // Would need network monitoring
            latencies: self.measure_latencies(),
            feeling: ReachFeeling::Normal,
            connections: ConnectionPoolStatus::default(),
            stability: NetworkStability::Stable,
            packet_loss: 0.0,
            egress_remaining: None,
        }
    }
    
    /// Update mind (memory) state
    pub fn update_mind(&mut self, mind: MindCapacity) -> Result<()> {
        let body = self.resource_store.get_body_mut()
            .ok_or(Error::NoResourceBody)?;
        
        let old_feeling = body.mind.feeling;
        body.mind = mind;
        
        // Add sensation if feeling changed significantly
        if old_feeling != body.mind.feeling {
            let sensation = match body.mind.feeling {
                MindFeeling::Drowning | MindFeeling::Overwhelmed => {
                    ResourceSensation {
                        resource: ResourceType::Memory,
                        sensation: SensationType::Pain,
                        intensity: 0.9,
                        started: Timestamp::now(),
                        trend: SensationTrend::Worsening,
                    }
                }
                MindFeeling::Strained => {
                    ResourceSensation {
                        resource: ResourceType::Memory,
                        sensation: SensationType::Pressure,
                        intensity: 0.6,
                        started: Timestamp::now(),
                        trend: SensationTrend::Stable,
                    }
                }
                MindFeeling::Clear => {
                    ResourceSensation {
                        resource: ResourceType::Memory,
                        sensation: SensationType::Relief,
                        intensity: 0.3,
                        started: Timestamp::now(),
                        trend: SensationTrend::Improving,
                    }
                }
                _ => {
                    ResourceSensation {
                        resource: ResourceType::Memory,
                        sensation: SensationType::Comfort,
                        intensity: 0.2,
                        started: Timestamp::now(),
                        trend: SensationTrend::Stable,
                    }
                }
            };
            
            body.sensations.push(sensation);
        }
        
        self.mark_dirty();
        Ok(())
    }
    
    /// Update pressure gradient
    pub fn update_pressure_gradient(&mut self) -> Result<ResourcePressureGradient> {
        let body = self.resource_store.get_body()
            .ok_or(Error::NoResourceBody)?;
        
        let mut pressures = HashMap::new();
        
        // Calculate pressure for each resource
        pressures.insert(ResourceType::Memory, Pressure {
            level: body.mind.used.0 as f64 / body.mind.total.0 as f64,
            rate: 0.0, // Would need historical data
            relative_to_normal: 1.0,
            alarm_threshold: 0.90,
            headroom: (body.mind.total.0 - body.mind.used.0) as f64 / body.mind.total.0 as f64,
        });
        
        pressures.insert(ResourceType::Cpu, Pressure {
            level: body.energy.utilization,
            rate: 0.0,
            relative_to_normal: 1.0,
            alarm_threshold: 0.85,
            headroom: 1.0 - body.energy.utilization,
        });
        
        pressures.insert(ResourceType::Network, Pressure {
            level: body.reach.utilization,
            rate: 0.0,
            relative_to_normal: 1.0,
            alarm_threshold: 0.80,
            headroom: 1.0 - body.reach.utilization,
        });
        
        // Determine bottleneck
        let bottleneck = pressures.iter()
            .max_by(|a, b| a.1.level.partial_cmp(&b.1.level).unwrap())
            .map(|(k, _)| *k)
            .unwrap_or(ResourceType::Memory);
        
        let gradient = ResourcePressureGradient {
            bottleneck,
            pressures,
            flow: PressureFlow::default(),
            building: vec![],
            releasing: vec![],
            predicted_bottleneck: None,
        };
        
        self.resource_store.set_pressure_gradient(gradient.clone());
        self.mark_dirty();
        
        Ok(gradient)
    }
    
    /// Discover capability
    pub fn discover_capability(&mut self, capability: Capability) -> Result<()> {
        Validator::validate_capability(&capability)?;
        
        self.resource_store.add_capability(capability.clone());
        self.indexes.index_capability(&capability);
        self.mark_dirty();
        
        Ok(())
    }
    
    /// Lose capability
    pub fn lose_capability(&mut self, capability_id: CapabilityId) -> Result<()> {
        self.resource_store.remove_capability(&capability_id);
        self.indexes.remove_capability(&capability_id);
        self.mark_dirty();
        
        Ok(())
    }
    
    /// Update cost consciousness
    pub fn update_cost(&mut self, cost: CostConsciousness) -> Result<()> {
        Validator::validate_cost(&cost)?;
        
        self.resource_store.set_cost(cost);
        self.mark_dirty();
        
        Ok(())
    }
}
```

## 5.6 Reality Write Operations

```rust
//! src/engine/write/reality.rs

impl WriteEngine {
    /// Set current reality layer
    pub fn set_reality_layer(&mut self, layer: RealityLayer) -> Result<()> {
        self.reality_store.set_current_layer(layer.clone());
        
        // Update layer status
        let status = LayerStatus {
            layer: layer.clone(),
            active: true,
            health: LayerHealth::Healthy,
            trust: self.calculate_layer_trust(&layer),
            dependents: vec![],
        };
        
        self.reality_store.update_layer_status(status);
        self.mark_dirty();
        
        Ok(())
    }
    
    fn calculate_layer_trust(&self, layer: &RealityLayer) -> f64 {
        match layer {
            RealityLayer::Physical { certainty, .. } => *certainty,
            RealityLayer::Virtual { .. } => 0.95,
            RealityLayer::Container { .. } => 0.90,
            RealityLayer::Sandbox { .. } => 0.85,
            RealityLayer::TestEnvironment { .. } => 0.70,
            RealityLayer::Simulation { fidelity, .. } => match fidelity {
                SimulationFidelity::Perfect => 0.90,
                SimulationFidelity::High { .. } => 0.70,
                SimulationFidelity::Medium { .. } => 0.50,
                SimulationFidelity::Low { .. } => 0.30,
                SimulationFidelity::Stub => 0.10,
            },
            RealityLayer::Replay { .. } => 0.60,
            RealityLayer::Preview { .. } => 0.50,
            RealityLayer::Unknown { .. } => 0.30,
        }
    }
    
    /// Update freshness for data source
    pub fn update_freshness(&mut self, source: DataSource, freshness: FreshnessLevel) -> Result<()> {
        let perception = self.reality_store.get_freshness_mut();
        
        perception.by_source.insert(source.clone(), SourceFreshness {
            source: source.clone(),
            freshness: freshness.clone(),
            tolerance: self.get_freshness_tolerance(&source),
            acceptable: self.is_freshness_acceptable(&freshness),
            refreshable: true,
            refresh_cost: None,
            refresh_time: None,
        });
        
        // Update stalest
        perception.stalest = perception.by_source.values()
            .filter(|f| !f.acceptable)
            .max_by(|a, b| self.compare_staleness(&a.freshness, &b.freshness))
            .map(|f| StaleData {
                source: f.source.clone(),
                age: self.get_age(&f.freshness),
                impact: self.assess_stale_impact(&f.source),
                usable: self.is_stale_usable(&f.freshness),
                confidence_discount: self.calculate_staleness_discount(&f.freshness),
            });
        
        // Update overall
        perception.overall = self.calculate_overall_freshness(&perception.by_source);
        
        self.mark_dirty();
        Ok(())
    }
    
    /// Add reality anchor
    pub fn add_anchor(&mut self, anchor: RealityAnchor) -> Result<()> {
        Validator::validate_anchor(&anchor)?;
        
        self.reality_store.add_anchor(anchor.clone());
        self.indexes.index_anchor(&anchor);
        self.mark_dirty();
        
        Ok(())
    }
    
    /// Verify anchor
    pub fn verify_anchor(&mut self, anchor_id: AnchorId) -> Result<AnchorVerificationResult> {
        let anchor = self.reality_store.get_anchor_mut(&anchor_id)
            .ok_or(Error::AnchorNotFound(anchor_id))?;
        
        let result = self.perform_anchor_verification(anchor)?;
        
        anchor.last_value = result.current_value.clone();
        anchor.trust = result.new_trust;
        
        // Record drift if any
        if result.drift > 0.0 {
            self.reality_store.record_anchor_drift(anchor_id, AnchorDriftRecord {
                timestamp: Timestamp::now(),
                anchor_id,
                drift_amount: result.drift,
                expected: result.expected_value,
                actual: result.current_value.clone(),
            });
        }
        
        self.mark_dirty();
        Ok(result)
    }
    
    fn perform_anchor_verification(&self, anchor: &RealityAnchor) -> Result<AnchorVerificationResult> {
        match &anchor.anchor_type {
            AnchorType::Time { source } => {
                self.verify_time_anchor(source)
            }
            AnchorType::Identity { verifier } => {
                self.verify_identity_anchor(verifier)
            }
            AnchorType::Configuration { source } => {
                self.verify_config_anchor(source)
            }
            AnchorType::State { source } => {
                self.verify_state_anchor(source)
            }
            AnchorType::External { api, field } => {
                self.verify_external_anchor(api, field)
            }
            AnchorType::Cryptographic { chain } => {
                self.verify_cryptographic_anchor(chain)
            }
            AnchorType::Human { verifier } => {
                // Human verification is async
                Ok(AnchorVerificationResult {
                    verified: false,
                    new_trust: anchor.trust,
                    drift: 0.0,
                    expected_value: anchor.last_value.clone(),
                    current_value: anchor.last_value.clone(),
                    verification_pending: true,
                })
            }
        }
    }
    
    /// Detect hallucination
    pub fn detect_hallucination(&mut self, hallucination: DetectedHallucination) -> Result<()> {
        Validator::validate_hallucination(&hallucination)?;
        
        let state = self.reality_store.get_hallucination_state_mut();
        state.detected.push(hallucination.clone());
        
        // Update risk level
        state.risk_level = self.calculate_hallucination_risk(&state.detected);
        
        // Update grounding status
        state.grounding = if state.detected.len() > 3 {
            GroundingStatus::Ungrounded { 
                reason: "Multiple hallucinations detected".to_string(),
            }
        } else if !state.detected.is_empty() {
            GroundingStatus::PartiallyGrounded {
                issues: state.detected.iter().map(|h| h.claim.clone()).collect(),
            }
        } else {
            state.grounding.clone()
        };
        
        self.indexes.index_hallucination(&hallucination);
        self.mark_dirty();
        
        Ok(())
    }
    
    /// Add unverified claim
    pub fn add_unverified_claim(&mut self, claim: UnverifiedClaim) -> Result<()> {
        let state = self.reality_store.get_hallucination_state_mut();
        state.pending_verification.push(claim);
        self.mark_dirty();
        Ok(())
    }
    
    /// Verify claim
    pub fn verify_claim(&mut self, claim_id: &str, result: ClaimVerificationResult) -> Result<()> {
        let state = self.reality_store.get_hallucination_state_mut();
        
        // Remove from pending
        state.pending_verification.retain(|c| c.id != claim_id);
        
        // If verification failed, it's a hallucination
        if !result.verified {
            let hallucination = DetectedHallucination {
                claim: result.claim,
                hallucination_type: HallucinationType::FactualInvention { 
                    domain: "unknown".to_string(),
                },
                detection_method: DetectionMethod::ExternalVerificationFailed {
                    source: result.verification_source,
                },
                actual: result.actual_value,
                detected_at: Timestamp::now(),
                severity: HallucinationSeverity::Medium,
            };
            
            state.detected.push(hallucination);
        }
        
        self.mark_dirty();
        Ok(())
    }
}
```

## 5.7 Topology Write Operations

```rust
//! src/engine/write/topology.rs

impl WriteEngine {
    /// Set topology position
    pub fn set_position(&mut self, position: TopologyPosition) -> Result<()> {
        self.topology_store.set_position(position);
        self.mark_dirty();
        Ok(())
    }
    
    /// Add downstream dependency
    pub fn add_downstream(&mut self, dependency: DownstreamEntity) -> Result<()> {
        Validator::validate_downstream(&dependency)?;
        
        self.topology_store.add_downstream(dependency.clone());
        self.indexes.index_dependency(&dependency);
        self.mark_dirty();
        
        Ok(())
    }
    
    /// Update downstream health
    pub fn update_downstream_health(
        &mut self, 
        dependency_id: DependencyId, 
        health: HealthStatus,
        latency: Option<LatencyStats>,
    ) -> Result<()> {
        let dep = self.topology_store.get_downstream_mut(&dependency_id)
            .ok_or(Error::DependencyNotFound(dependency_id))?;
        
        dep.health = health;
        if let Some(l) = latency {
            dep.latency = l;
        }
        
        // Update topology health
        self.recalculate_topology_health()?;
        
        self.mark_dirty();
        Ok(())
    }
    
    /// Add sibling entity
    pub fn add_sibling(&mut self, sibling: SiblingEntity) -> Result<()> {
        Validator::validate_sibling(&sibling)?;
        
        self.topology_store.add_sibling(sibling.clone());
        self.indexes.index_sibling(&sibling);
        self.mark_dirty();
        
        Ok(())
    }
    
    /// Update sibling state
    pub fn update_sibling_state(
        &mut self,
        sibling_id: &NeighborId,
        health: HealthStatus,
        load: LoadLevel,
    ) -> Result<()> {
        let sibling = self.topology_store.get_sibling_mut(sibling_id)
            .ok_or(Error::SiblingNotFound(sibling_id.clone()))?;
        
        sibling.health = health;
        sibling.load = load;
        sibling.last_contact = Timestamp::now();
        
        self.mark_dirty();
        Ok(())
    }
    
    /// Add observer
    pub fn add_observer(&mut self, observer: ObserverEntity) -> Result<()> {
        Validator::validate_observer(&observer)?;
        
        self.topology_store.add_observer(observer.clone());
        self.indexes.index_observer(&observer);
        self.mark_dirty();
        
        Ok(())
    }
    
    /// Update topology health
    fn recalculate_topology_health(&mut self) -> Result<()> {
        let topology = self.topology_store.get_topology_map();
        
        let mut health = TopologyHealth {
            overall: HealthLevel::Healthy,
            single_points_of_failure: vec![],
            stressed_links: vec![],
            failures: vec![],
            stability: 1.0,
        };
        
        // Check for unhealthy dependencies
        for dep in &topology.downstream {
            match dep.health {
                HealthStatus::Unhealthy { reason } => {
                    health.failures.push(ComponentFailure {
                        component: dep.service.0.clone(),
                        failure_type: FailureType::Unavailable,
                        since: Timestamp::now(),
                        impact: self.assess_dependency_impact(dep),
                    });
                }
                HealthStatus::Degraded { degradation } => {
                    health.stressed_links.push(TopologyLink {
                        from: "self".to_string(),
                        to: dep.service.0.clone(),
                        stress_level: degradation,
                    });
                }
                _ => {}
            }
        }
        
        // Check for single points of failure
        let critical_deps: Vec<_> = topology.downstream.iter()
            .filter(|d| matches!(d.criticality, DependencyCriticality::Critical { .. }))
            .collect();
        
        for dep in critical_deps {
            if dep.fallback.is_none() {
                health.single_points_of_failure.push(TopologyPosition {
                    layer: StackLayer::Data,
                    edge_distance: 0,
                    core_distance: 1,
                    criticality: PositionCriticality::Critical,
                    redundancy: RedundancyLevel::None,
                });
            }
        }
        
        // Calculate overall health
        health.overall = if !health.failures.is_empty() {
            HealthLevel::Critical
        } else if !health.stressed_links.is_empty() || !health.single_points_of_failure.is_empty() {
            HealthLevel::Degraded
        } else {
            HealthLevel::Healthy
        };
        
        self.topology_store.set_topology_health(health);
        
        Ok(())
    }
}
```

## 5.8 Stakes Write Operations

```rust
//! src/engine/write/stakes.rs

impl WriteEngine {
    /// Set stakes level
    pub fn set_stakes_level(&mut self, stakes: StakesLevel) -> Result<()> {
        self.stakes_store.set_stakes_level(stakes);
        self.mark_dirty();
        Ok(())
    }
    
    /// Add consequence
    pub fn add_consequence(&mut self, action: ActionType, consequence: Consequence) -> Result<()> {
        Validator::validate_consequence(&consequence)?;
        
        self.stakes_store.add_consequence(action, consequence);
        self.mark_dirty();
        Ok(())
    }
    
    /// Add irreversible action
    pub fn add_irreversible_action(&mut self, action: IrreversibleAction) -> Result<()> {
        Validator::validate_irreversible_action(&action)?;
        
        self.stakes_store.add_irreversible_action(action);
        self.mark_dirty();
        Ok(())
    }
    
    /// Update risk field
    pub fn update_risk_field(&mut self, risk: RiskFieldPerception) -> Result<()> {
        Validator::validate_risk_field(&risk)?;
        
        self.stakes_store.set_risk_field(risk);
        self.mark_dirty();
        Ok(())
    }
    
    /// Update blast radius
    pub fn update_blast_radius(&mut self, blast: BlastRadiusAwareness) -> Result<()> {
        self.stakes_store.set_blast_radius(blast);
        self.mark_dirty();
        Ok(())
    }
    
    /// Record actual consequence
    pub fn record_consequence(&mut self, consequence: ActualConsequence) -> Result<()> {
        self.stakes_store.record_consequence(consequence);
        self.mark_dirty();
        Ok(())
    }
}
```

## 5.9 Coherence Write Operations

```rust
//! src/engine/write/coherence.rs

impl WriteEngine {
    /// Run coherence check
    pub fn run_coherence_check(&mut self) -> Result<CoherenceResult> {
        let mut violations = vec![];
        
        // Time consistency
        if let Some(v) = self.check_time_coherence() {
            violations.push(v);
        }
        
        // State consistency
        if let Some(v) = self.check_state_coherence() {
            violations.push(v);
        }
        
        // Layer consistency
        if let Some(v) = self.check_layer_coherence() {
            violations.push(v);
        }
        
        // Anchor consistency
        if let Some(v) = self.check_anchor_coherence() {
            violations.push(v);
        }
        
        // Memory consistency
        if let Some(v) = self.check_memory_coherence() {
            violations.push(v);
        }
        
        // Causal consistency
        if let Some(v) = self.check_causal_coherence() {
            violations.push(v);
        }
        
        // Calculate coherence level
        let level = if violations.is_empty() {
            CoherenceLevel::Full { confidence: 0.95 }
        } else if violations.iter().all(|v| v.severity <= Severity::Low) {
            CoherenceLevel::Minor { 
                issues: violations.iter().map(|v| v.description.clone()).collect(),
            }
        } else if violations.iter().any(|v| v.severity >= Severity::Critical) {
            CoherenceLevel::Incoherent {
                reason: "Critical coherence violation".to_string(),
                severity: Severity::Critical,
            }
        } else {
            CoherenceLevel::Significant {
                issues: violations.iter().map(|v| v.description.clone()).collect(),
                impact: "Some operations may be affected".to_string(),
            }
        };
        
        // Store violations
        for violation in &violations {
            self.coherence_store.add_violation(violation.clone());
        }
        
        // Update state
        self.coherence_store.set_level(level.clone());
        self.mark_dirty();
        
        Ok(CoherenceResult {
            level,
            violations,
            checked_at: Timestamp::now(),
        })
    }
    
    fn check_time_coherence(&self) -> Option<CoherenceViolation> {
        // Check if timestamps are consistent
        let now = Timestamp::now();
        
        if let Some(soul) = self.deployment_store.get_soul() {
            // Birth should be before now
            if soul.birth.spawned_at > now {
                return Some(CoherenceViolation {
                    violation_type: ViolationType::TemporalInconsistency,
                    detected: now,
                    severity: Severity::High,
                    evidence: vec![Evidence {
                        source: "deployment.birth.spawned_at".to_string(),
                        value: format!("{:?}", soul.birth.spawned_at),
                        expected: format!("< {:?}", now),
                    }],
                    possible_causes: vec![
                        PossibleCause::ClockSkew,
                        PossibleCause::TimeSourceError,
                    ],
                    resolution: ResolutionStatus::Pending,
                    description: "Birth timestamp is in the future".to_string(),
                });
            }
        }
        
        None
    }
    
    fn check_anchor_coherence(&self) -> Option<CoherenceViolation> {
        let anchors = self.reality_store.get_anchors();
        
        // Check if any anchors have low trust
        let untrusted: Vec<_> = anchors.iter()
            .filter(|a| a.trust < 0.5)
            .collect();
        
        if !untrusted.is_empty() {
            return Some(CoherenceViolation {
                violation_type: ViolationType::AnchorDrift,
                detected: Timestamp::now(),
                severity: Severity::Medium,
                evidence: untrusted.iter().map(|a| Evidence {
                    source: format!("anchor.{:?}", a.id),
                    value: format!("trust={}", a.trust),
                    expected: "> 0.5".to_string(),
                }).collect(),
                possible_causes: vec![
                    PossibleCause::RealityDrift,
                    PossibleCause::AnchorStale,
                ],
                resolution: ResolutionStatus::Pending,
                description: "Reality anchors have low trust".to_string(),
            });
        }
        
        None
    }
    
    /// Begin context transition
    pub fn begin_transition(&mut self, to: RealityContext) -> Result<TransitionId> {
        let id = TransitionId::new();
        let from = self.coherence_store.get_current_context();
        
        let transition = PendingTransition {
            id,
            from,
            to_context: to.clone(),
            transition_type: self.determine_transition_type(&to),
            phase: TransitionPhase::Planning,
            carry_over: self.plan_carry_over(&to),
            started: Timestamp::now(),
        };
        
        self.coherence_store.add_pending_transition(transition);
        self.mark_dirty();
        
        Ok(id)
    }
    
    /// Advance transition phase
    pub fn advance_transition(&mut self, id: TransitionId) -> Result<TransitionPhase> {
        let transition = self.coherence_store.get_pending_transition_mut(&id)
            .ok_or(Error::TransitionNotFound(id))?;
        
        let next_phase = match transition.phase {
            TransitionPhase::Planning => TransitionPhase::Preparing,
            TransitionPhase::Preparing => TransitionPhase::Validating,
            TransitionPhase::Validating => TransitionPhase::Executing,
            TransitionPhase::Executing => TransitionPhase::Verifying,
            TransitionPhase::Verifying => TransitionPhase::Complete,
            other => other,
        };
        
        transition.phase = next_phase;
        self.mark_dirty();
        
        Ok(next_phase)
    }
    
    /// Complete transition
    pub fn complete_transition(&mut self, id: TransitionId) -> Result<()> {
        let transition = self.coherence_store.remove_pending_transition(&id)
            .ok_or(Error::TransitionNotFound(id))?;
        
        // Update current context
        self.coherence_store.set_current_context(transition.to_context.id);
        
        // Record completed transition
        self.coherence_store.add_completed_transition(CompletedTransition {
            id,
            from: transition.from,
            to: transition.to_context.id,
            completed: Timestamp::now(),
            duration: Timestamp::now().0 - transition.started.0,
            success: true,
        });
        
        self.mark_dirty();
        Ok(())
    }
    
    /// Abort transition
    pub fn abort_transition(&mut self, id: TransitionId, reason: String) -> Result<()> {
        let transition = self.coherence_store.get_pending_transition_mut(&id)
            .ok_or(Error::TransitionNotFound(id))?;
        
        transition.phase = TransitionPhase::Failed;
        self.mark_dirty();
        
        Ok(())
    }
}
```

---

# SPEC-06: QUERY ENGINE

## 6.1 Query Operations Overview

```
QUERY OPERATIONS BY DOMAIN:
═══════════════════════════

DEPLOYMENT (10 operations)
├── get_soul                  Get deployment soul
├── get_birth_context         Get birth context
├── get_substrate             Get physical substrate
├── get_vitals                Get soul vitals
├── get_lineage               Get deployment lineage
├── get_incarnation_memory    Get past lives
├── get_wisdom                Get accumulated wisdom
├── get_karma                 Get karmic balance
├── get_nature                Get existential nature
└── get_context_summary       Get deployment summary

ENVIRONMENT (8 operations)
├── get_environment           Get environment medium
├── get_environment_type      Get environment type
├── get_environment_state     Get current state
├── get_mood                  Get environment mood
├── get_physics               Get environment physics
├── get_incidents             Get active incidents
├── get_fingerprint           Get context fingerprint
└── has_context_shifted       Check if context shifted

RESOURCE (12 operations)
├── get_body                  Get resource body
├── get_mind                  Get memory state
├── get_energy                Get CPU state
├── get_reach                 Get network state
├── get_storage               Get storage state
├── get_visual                Get GPU state (if any)
├── get_sensations            Get current sensations
├── get_pressure_gradient     Get pressure gradient
├── get_bottleneck            Get current bottleneck
├── get_capabilities          Get capability map
├── can_do                    Check if capability available
└── get_cost                  Get cost consciousness

REALITY (10 operations)
├── get_reality_layers        Get all reality layers
├── get_current_layer         Get current layer
├── get_freshness             Get freshness perception
├── is_fresh                  Check if source is fresh
├── get_anchors               Get reality anchors
├── get_anchor                Get specific anchor
├── get_anchor_drift          Get drift from anchors
├── get_hallucination_state   Get hallucination state
├── get_hallucination_risk    Get current risk level
└── get_grounding_status      Get grounding status

TOPOLOGY (12 operations)
├── get_topology_map          Get full topology map
├── get_position              Get self position
├── get_upstream              Get upstream entities
├── get_downstream            Get downstream dependencies
├── get_dependency            Get specific dependency
├── get_siblings              Get sibling entities
├── get_observers             Get observers
├── get_topology_health       Get topology health
├── get_weak_links            Get weak dependencies
├── get_single_points         Get single points of failure
├── find_path                 Find path to service
└── get_neighbor_awareness    Get neighbor awareness

TEMPORAL (8 operations)
├── get_temporal_context      Get temporal context
├── get_grounded_time         Get grounded time
├── get_causality_graph       Get causality graph
├── get_causal_chain          Get chain for event
├── get_root_causes           Get root causes
├── get_timelines             Get all timelines
├── get_unified_timeline      Get unified timeline
└── get_deadlines             Get active deadlines

STAKES (10 operations)
├── get_stakes_level          Get current stakes
├── get_consequences          Get consequences for action
├── get_irreversible_actions  Get irreversible actions
├── is_irreversible           Check if action irreversible
├── get_safety_margins        Get safety margins
├── get_guardrails            Get active guardrails
├── get_risk_field            Get risk field perception
├── get_risk_for              Get risk for category
├── get_blast_radius          Get blast radius
└── get_cascade_analysis      Get cascade analysis

COHERENCE (8 operations)
├── get_coherence_state       Get coherence state
├── get_coherence_level       Get current level
├── get_violations            Get active violations
├── get_pending_transitions   Get pending transitions
├── get_transition            Get specific transition
├── get_transition_history    Get completed transitions
├── is_coherent               Check if coherent
└── get_coherence_checks      Get check results

TOTAL: ~78 query operations
```

## 6.2 Core Query Engine

```rust
//! src/engine/query.rs

/// Query engine for reading reality state
pub struct QueryEngine<'a> {
    /// Reference to deployment store
    deployment_store: &'a DeploymentStore,
    
    /// Reference to environment store
    environment_store: &'a EnvironmentStore,
    
    /// Reference to resource store
    resource_store: &'a ResourceStore,
    
    /// Reference to reality store
    reality_store: &'a RealityStore,
    
    /// Reference to topology store
    topology_store: &'a TopologyStore,
    
    /// Reference to temporal store
    temporal_store: &'a TemporalStore,
    
    /// Reference to stakes store
    stakes_store: &'a StakesStore,
    
    /// Reference to coherence store
    coherence_store: &'a CoherenceStore,
    
    /// Reference to indexes
    indexes: &'a RealityIndexes,
}

impl<'a> QueryEngine<'a> {
    pub fn new(
        deployment_store: &'a DeploymentStore,
        environment_store: &'a EnvironmentStore,
        resource_store: &'a ResourceStore,
        reality_store: &'a RealityStore,
        topology_store: &'a TopologyStore,
        temporal_store: &'a TemporalStore,
        stakes_store: &'a StakesStore,
        coherence_store: &'a CoherenceStore,
        indexes: &'a RealityIndexes,
    ) -> Self {
        Self {
            deployment_store,
            environment_store,
            resource_store,
            reality_store,
            topology_store,
            temporal_store,
            stakes_store,
            coherence_store,
            indexes,
        }
    }
}
```

## 6.3 Deployment Queries

```rust
//! src/engine/query/deployment.rs

impl<'a> QueryEngine<'a> {
    /// Get deployment soul
    pub fn get_soul(&self) -> Option<&DeploymentSoul> {
        self.deployment_store.get_soul()
    }
    
    /// Get birth context
    pub fn get_birth_context(&self) -> Option<&BirthContext> {
        self.deployment_store.get_soul().map(|s| &s.birth)
    }
    
    /// Get physical substrate
    pub fn get_substrate(&self) -> Option<&PhysicalSubstrate> {
        self.deployment_store.get_soul().map(|s| &s.substrate)
    }
    
    /// Get soul vitals
    pub fn get_vitals(&self) -> Option<&SoulVitals> {
        self.deployment_store.get_soul().map(|s| &s.vitals)
    }
    
    /// Get deployment lineage
    pub fn get_lineage(&self) -> Option<&DeploymentLineage> {
        self.deployment_store.get_soul().map(|s| &s.lineage)
    }
    
    /// Get incarnation memory (past lives)
    pub fn get_incarnation_memory(&self) -> &[PastIncarnation] {
        self.deployment_store.get_past_lives()
    }
    
    /// Get accumulated wisdom
    pub fn get_wisdom(&self) -> &IncarnationWisdom {
        self.deployment_store.get_wisdom()
    }
    
    /// Get context summary
    pub fn get_context_summary(&self) -> ContextSummary {
        let soul = self.deployment_store.get_soul();
        let env = self.environment_store.get_environment();
        let resources = self.resource_store.get_body();
        
        ContextSummary {
            incarnation_id: soul.map(|s| s.incarnation_id),
            substrate_tier: soul.map(|s| format!("{:?}", s.substrate.tier)),
            environment_type: env.map(|e| format!("{:?}", e.environment_type)),
            environment_mood: env.map(|e| e.current_state.mood),
            resource_bottleneck: self.resource_store.get_pressure_gradient()
                .map(|g| g.bottleneck),
            stakes_level: self.stakes_store.get_stakes_level().cloned(),
            coherence_level: self.coherence_store.get_level().cloned(),
            uptime: soul.map(|s| s.vitals.uptime),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ContextSummary {
    pub incarnation_id: Option<IncarnationId>,
    pub substrate_tier: Option<String>,
    pub environment_type: Option<String>,
    pub environment_mood: Option<EnvironmentMood>,
    pub resource_bottleneck: Option<ResourceType>,
    pub stakes_level: Option<StakesLevel>,
    pub coherence_level: Option<CoherenceLevel>,
    pub uptime: Option<Duration>,
}
```

## 6.4 Environment Queries

```rust
//! src/engine/query/environment.rs

impl<'a> QueryEngine<'a> {
    /// Get environment medium
    pub fn get_environment(&self) -> Option<&EnvironmentMedium> {
        self.environment_store.get_environment()
    }
    
    /// Get environment type
    pub fn get_environment_type(&self) -> Option<&EnvironmentType> {
        self.environment_store.get_environment().map(|e| &e.environment_type)
    }
    
    /// Get current environment state
    pub fn get_environment_state(&self) -> Option<&EnvironmentState> {
        self.environment_store.get_environment().map(|e| &e.current_state)
    }
    
    /// Get environment mood
    pub fn get_mood(&self) -> Option<EnvironmentMood> {
        self.environment_store.get_environment()
            .map(|e| e.current_state.mood)
    }
    
    /// Get environment physics
    pub fn get_physics(&self) -> Option<&EnvironmentPhysics> {
        self.environment_store.get_environment().map(|e| &e.physics)
    }
    
    /// Get active incidents
    pub fn get_incidents(&self) -> Vec<&ActiveIncident> {
        self.environment_store.get_environment()
            .map(|e| e.current_state.incidents.iter().collect())
            .unwrap_or_default()
    }
    
    /// Get context fingerprint
    pub fn get_fingerprint(&self) -> Option<&ContextFingerprint> {
        self.environment_store.get_fingerprint()
    }
    
    /// Check if context has shifted
    pub fn has_context_shifted(&self, threshold: f64) -> ContextShiftResult {
        let stored = match self.environment_store.get_fingerprint() {
            Some(fp) => fp,
            None => return ContextShiftResult {
                shifted: false,
                distance: 0.0,
                changed_components: vec![],
                shift_severity: ShiftSeverity::None,
            },
        };
        
        // Compute current fingerprint
        let current = self.compute_current_fingerprint();
        
        // Compare
        let distance = self.fingerprint_distance(stored, &current);
        let changed = self.find_changed_components(stored, &current);
        
        ContextShiftResult {
            shifted: distance > threshold,
            distance,
            changed_components: changed,
            shift_severity: if distance > 0.8 {
                ShiftSeverity::Critical
            } else if distance > 0.5 {
                ShiftSeverity::Major
            } else if distance > 0.2 {
                ShiftSeverity::Minor
            } else {
                ShiftSeverity::None
            },
        }
    }
    
    fn fingerprint_distance(&self, a: &ContextFingerprint, b: &ContextFingerprint) -> f64 {
        let mut diff = 0u32;
        
        for i in 0..32 {
            diff += (a.hash[i] ^ b.hash[i]).count_ones();
        }
        
        diff as f64 / 256.0
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ContextShiftResult {
    pub shifted: bool,
    pub distance: f64,
    pub changed_components: Vec<String>,
    pub shift_severity: ShiftSeverity,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ShiftSeverity {
    None,
    Minor,
    Major,
    Critical,
}
```

## 6.5 Resource Queries

```rust
//! src/engine/query/resource.rs

impl<'a> QueryEngine<'a> {
    /// Get resource body
    pub fn get_body(&self) -> Option<&ResourceBody> {
        self.resource_store.get_body()
    }
    
    /// Get mind (memory) state
    pub fn get_mind(&self) -> Option<&MindCapacity> {
        self.resource_store.get_body().map(|b| &b.mind)
    }
    
    /// Get energy (CPU) state
    pub fn get_energy(&self) -> Option<&ProcessingEnergy> {
        self.resource_store.get_body().map(|b| &b.energy)
    }
    
    /// Get reach (network) state
    pub fn get_reach(&self) -> Option<&NetworkReach> {
        self.resource_store.get_body().map(|b| &b.reach)
    }
    
    /// Get current sensations
    pub fn get_sensations(&self) -> Vec<&ResourceSensation> {
        self.resource_store.get_body()
            .map(|b| b.sensations.iter().collect())
            .unwrap_or_default()
    }
    
    /// Get pressure gradient
    pub fn get_pressure_gradient(&self) -> Option<&ResourcePressureGradient> {
        self.resource_store.get_pressure_gradient()
    }
    
    /// Get current bottleneck
    pub fn get_bottleneck(&self) -> Option<ResourceType> {
        self.resource_store.get_pressure_gradient().map(|g| g.bottleneck)
    }
    
    /// Get capabilities
    pub fn get_capabilities(&self) -> &[Capability] {
        self.resource_store.get_capabilities()
    }
    
    /// Check if capability is available
    pub fn can_do(&self, capability: &str) -> CapabilityCheck {
        let capabilities = self.resource_store.get_capabilities();
        
        match capabilities.iter().find(|c| c.enables == capability) {
            Some(cap) if cap.available => {
                CapabilityCheck::Yes {
                    constraints: cap.constraints.clone(),
                    cost: cap.cost.clone(),
                }
            }
            Some(_) => {
                CapabilityCheck::NotCurrently {
                    reason: "Capability exists but temporarily unavailable".to_string(),
                }
            }
            None => {
                CapabilityCheck::No {
                    alternatives: self.find_capability_alternatives(capability),
                }
            }
        }
    }
    
    /// Get cost consciousness
    pub fn get_cost(&self) -> Option<&CostConsciousness> {
        self.resource_store.get_cost()
    }
    
    fn find_capability_alternatives(&self, capability: &str) -> Vec<String> {
        // Look for similar capabilities
        self.resource_store.get_capabilities()
            .iter()
            .filter(|c| c.enables.contains(capability) || capability.contains(&c.enables))
            .map(|c| c.enables.clone())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum CapabilityCheck {
    Yes {
        constraints: Vec<CapabilityConstraint>,
        cost: Option<CapabilityCost>,
    },
    NotCurrently {
        reason: String,
    },
    No {
        alternatives: Vec<String>,
    },
}
```

## 6.6 Reality Queries

```rust
//! src/engine/query/reality.rs

impl<'a> QueryEngine<'a> {
    /// Get reality layers
    pub fn get_reality_layers(&self) -> Option<&RealityLayers> {
        self.reality_store.get_layers()
    }
    
    /// Get current layer
    pub fn get_current_layer(&self) -> Option<&RealityLayer> {
        self.reality_store.get_layers().map(|l| &l.current_layer)
    }
    
    /// Get freshness perception
    pub fn get_freshness(&self) -> &FreshnessPerception {
        self.reality_store.get_freshness()
    }
    
    /// Check if data source is fresh
    pub fn is_fresh(&self, source: &DataSource) -> FreshnessCheck {
        let perception = self.reality_store.get_freshness();
        
        match perception.by_source.get(source) {
            Some(freshness) => FreshnessCheck {
                fresh: freshness.acceptable,
                level: freshness.freshness.clone(),
                age: self.get_age(&freshness.freshness),
                tolerance: freshness.tolerance,
                refresh_recommended: !freshness.acceptable,
            },
            None => FreshnessCheck {
                fresh: false,
                level: FreshnessLevel::Unknown { last_known: None },
                age: None,
                tolerance: Duration::from_secs(3600),
                refresh_recommended: true,
            },
        }
    }
    
    fn get_age(&self, level: &FreshnessLevel) -> Option<Duration> {
        match level {
            FreshnessLevel::Live { latency } => Some(*latency),
            FreshnessLevel::Fresh { age } => Some(*age),
            FreshnessLevel::Acceptable { age } => Some(*age),
            FreshnessLevel::Aging { age, .. } => Some(*age),
            FreshnessLevel::Stale { age, .. } => Some(*age),
            FreshnessLevel::Ancient { age, .. } => Some(*age),
            FreshnessLevel::Unknown { .. } => None,
        }
    }
    
    /// Get reality anchors
    pub fn get_anchors(&self) -> &[RealityAnchor] {
        self.reality_store.get_anchors()
    }
    
    /// Get specific anchor
    pub fn get_anchor(&self, id: &AnchorId) -> Option<&RealityAnchor> {
        self.reality_store.get_anchor(id)
    }
    
    /// Get anchor drift
    pub fn get_anchor_drift(&self) -> &AnchorDrift {
        self.reality_store.get_anchor_drift()
    }
    
    /// Get hallucination state
    pub fn get_hallucination_state(&self) -> &HallucinationState {
        self.reality_store.get_hallucination_state()
    }
    
    /// Get hallucination risk
    pub fn get_hallucination_risk(&self) -> &HallucinationRisk {
        &self.reality_store.get_hallucination_state().risk_level
    }
    
    /// Get grounding status
    pub fn get_grounding_status(&self) -> &GroundingStatus {
        &self.reality_store.get_hallucination_state().grounding
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FreshnessCheck {
    pub fresh: bool,
    pub level: FreshnessLevel,
    pub age: Option<Duration>,
    pub tolerance: Duration,
    pub refresh_recommended: bool,
}
```

## 6.7 Topology Queries

```rust
//! src/engine/query/topology.rs

impl<'a> QueryEngine<'a> {
    /// Get topology map
    pub fn get_topology_map(&self) -> &DeploymentTopologyMap {
        self.topology_store.get_topology_map()
    }
    
    /// Get self position
    pub fn get_position(&self) -> Option<&TopologyPosition> {
        Some(&self.topology_store.get_topology_map().self_position)
    }
    
    /// Get downstream dependencies
    pub fn get_downstream(&self) -> &[DownstreamEntity] {
        &self.topology_store.get_topology_map().downstream
    }
    
    /// Get specific dependency
    pub fn get_dependency(&self, id: &DependencyId) -> Option<&DownstreamEntity> {
        self.topology_store.get_downstream(id)
    }
    
    /// Get siblings
    pub fn get_siblings(&self) -> &[SiblingEntity] {
        &self.topology_store.get_topology_map().siblings
    }
    
    /// Get observers
    pub fn get_observers(&self) -> &[ObserverEntity] {
        &self.topology_store.get_topology_map().observers
    }
    
    /// Get topology health
    pub fn get_topology_health(&self) -> &TopologyHealth {
        &self.topology_store.get_topology_map().topology_health
    }
    
    /// Get weak links
    pub fn get_weak_links(&self) -> Vec<WeakLink> {
        let topology = self.topology_store.get_topology_map();
        let mut weak = vec![];
        
        for dep in &topology.downstream {
            // Check if critical without fallback
            if matches!(dep.criticality, DependencyCriticality::Critical { .. }) {
                if dep.fallback.is_none() {
                    weak.push(WeakLink {
                        dependency: dep.id,
                        weakness: WeaknessType::SinglePointOfFailure,
                        risk: 0.9,
                        mitigations: vec![
                            Mitigation::AddFallback,
                            Mitigation::AddRedundancy,
                        ],
                    });
                }
            }
            
            // Check health
            if matches!(dep.health, HealthStatus::Degraded { .. }) {
                weak.push(WeakLink {
                    dependency: dep.id,
                    weakness: WeaknessType::HistoricallyUnreliable { failure_rate: 0.1 },
                    risk: 0.5,
                    mitigations: vec![
                        Mitigation::AddCircuitBreaker,
                        Mitigation::AddRetry,
                    ],
                });
            }
        }
        
        weak
    }
    
    /// Get single points of failure
    pub fn get_single_points(&self) -> &[TopologyPosition] {
        &self.topology_store.get_topology_map().topology_health.single_points_of_failure
    }
}
```

## 6.8 Stakes Queries

```rust
//! src/engine/query/stakes.rs

impl<'a> QueryEngine<'a> {
    /// Get stakes level
    pub fn get_stakes_level(&self) -> Option<&StakesLevel> {
        self.stakes_store.get_stakes_level()
    }
    
    /// Get consequences for action
    pub fn get_consequences(&self, action: &ActionType) -> Vec<&Consequence> {
        self.stakes_store.get_consequences(action)
    }
    
    /// Get irreversible actions
    pub fn get_irreversible_actions(&self) -> &[IrreversibleAction] {
        self.stakes_store.get_irreversible_actions()
    }
    
    /// Check if action is irreversible
    pub fn is_irreversible(&self, action: &str) -> bool {
        self.stakes_store.get_irreversible_actions()
            .iter()
            .any(|a| a.action == action)
    }
    
    /// Get safety margins
    pub fn get_safety_margins(&self) -> Option<&SafetyMargins> {
        self.stakes_store.get_safety_margins()
    }
    
    /// Get risk field
    pub fn get_risk_field(&self) -> Option<&RiskFieldPerception> {
        self.stakes_store.get_risk_field()
    }
    
    /// Get risk for category
    pub fn get_risk_for(&self, category: RiskCategory) -> Option<&RiskLevel> {
        self.stakes_store.get_risk_field()
            .and_then(|r| r.risk_map.get(&category))
    }
    
    /// Get blast radius
    pub fn get_blast_radius(&self) -> Option<&BlastRadiusAwareness> {
        self.stakes_store.get_blast_radius()
    }
    
    /// Get cascade analysis
    pub fn get_cascade_analysis(&self) -> Option<&CascadeAnalysis> {
        self.stakes_store.get_blast_radius().map(|b| &b.cascade)
    }
    
    /// Should I proceed with this action?
    pub fn should_proceed(&self, action: &str, risk_tolerance: f64) -> ActionDecision {
        let stakes = self.get_stakes_level();
        let is_irreversible = self.is_irreversible(action);
        let risk_field = self.get_risk_field();
        
        // Check stakes level
        if let Some(StakesLevel::Critical { no_risk_tolerance, .. }) = stakes {
            if *no_risk_tolerance && is_irreversible {
                return ActionDecision::Deny {
                    reason: "Critical stakes, irreversible action".to_string(),
                    requires: vec!["multiple_approvals".to_string()],
                };
            }
        }
        
        // Check risk
        if let Some(risk) = risk_field {
            if risk.overall_risk > risk_tolerance {
                return ActionDecision::Caution {
                    risk_level: risk.overall_risk,
                    concerns: risk.hotspots.iter().map(|h| h.reason.clone()).collect(),
                };
            }
        }
        
        // Check if irreversible
        if is_irreversible {
            return ActionDecision::ConfirmFirst {
                reason: "Action is irreversible".to_string(),
                cooling_off: Some(Duration::from_secs(5)),
            };
        }
        
        ActionDecision::Proceed
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum ActionDecision {
    Proceed,
    ConfirmFirst {
        reason: String,
        cooling_off: Option<Duration>,
    },
    Caution {
        risk_level: f64,
        concerns: Vec<String>,
    },
    Deny {
        reason: String,
        requires: Vec<String>,
    },
}
```

---

# SPEC-07: INDEXES

## 7.1 Index Structure

```rust
//! src/indexes/mod.rs

/// Indexes for fast reality state lookups
#[derive(Debug, Default)]
pub struct RealityIndexes {
    // Deployment indexes
    pub incarnations_by_birth: BTreeMap<Timestamp, IncarnationId>,
    pub past_lives_by_death_cause: HashMap<DeathCauseCategory, Vec<IncarnationId>>,
    
    // Environment indexes
    pub incidents_by_severity: HashMap<Severity, Vec<String>>,
    pub incidents_active: HashSet<String>,
    
    // Resource indexes
    pub capabilities_by_category: HashMap<CapabilityCategory, Vec<CapabilityId>>,
    pub capabilities_available: HashSet<CapabilityId>,
    
    // Reality indexes
    pub anchors_by_type: HashMap<AnchorTypeCategory, Vec<AnchorId>>,
    pub anchors_by_trust: BTreeMap<OrderedFloat<f64>, Vec<AnchorId>>,
    pub hallucinations_by_type: HashMap<HallucinationTypeCategory, Vec<String>>,
    
    // Topology indexes
    pub dependencies_by_criticality: HashMap<DependencyCriticalityLevel, Vec<DependencyId>>,
    pub dependencies_by_health: HashMap<HealthStatusCategory, Vec<DependencyId>>,
    pub siblings_by_health: HashMap<HealthStatusCategory, Vec<NeighborId>>,
    pub observers_by_type: HashMap<ObserverTypeCategory, Vec<ObserverId>>,
    
    // Temporal indexes
    pub events_by_time: BTreeMap<Timestamp, Vec<EventId>>,
    pub deadlines_by_time: BTreeMap<Timestamp, Vec<String>>,
    
    // Stakes indexes
    pub risks_by_category: HashMap<RiskCategory, f64>,
    
    // Coherence indexes
    pub violations_by_type: HashMap<ViolationType, Vec<String>>,
    pub violations_unresolved: HashSet<String>,
}

impl RealityIndexes {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Rebuild all indexes from stores
    pub fn rebuild(&mut self, engine: &RealityEngine) {
        self.clear();
        
        // Index deployment
        if let Some(soul) = engine.deployment_store.get_soul() {
            self.index_deployment(soul);
        }
        
        for past in engine.deployment_store.get_past_lives() {
            self.index_past_life(past);
        }
        
        // Index environment
        if let Some(env) = engine.environment_store.get_environment() {
            for incident in &env.current_state.incidents {
                self.index_incident(incident);
            }
        }
        
        // Index resources
        for cap in engine.resource_store.get_capabilities() {
            self.index_capability(cap);
        }
        
        // Index reality
        for anchor in engine.reality_store.get_anchors() {
            self.index_anchor(anchor);
        }
        
        for halluc in &engine.reality_store.get_hallucination_state().detected {
            self.index_hallucination(halluc);
        }
        
        // Index topology
        let topology = engine.topology_store.get_topology_map();
        for dep in &topology.downstream {
            self.index_dependency(dep);
        }
        for sibling in &topology.siblings {
            self.index_sibling(sibling);
        }
        for observer in &topology.observers {
            self.index_observer(observer);
        }
        
        // Index coherence
        for violation in engine.coherence_store.get_violations() {
            self.index_violation(violation);
        }
    }
    
    fn clear(&mut self) {
        self.incarnations_by_birth.clear();
        self.past_lives_by_death_cause.clear();
        self.incidents_by_severity.clear();
        self.incidents_active.clear();
        self.capabilities_by_category.clear();
        self.capabilities_available.clear();
        self.anchors_by_type.clear();
        self.anchors_by_trust.clear();
        self.hallucinations_by_type.clear();
        self.dependencies_by_criticality.clear();
        self.dependencies_by_health.clear();
        self.siblings_by_health.clear();
        self.observers_by_type.clear();
        self.events_by_time.clear();
        self.deadlines_by_time.clear();
        self.risks_by_category.clear();
        self.violations_by_type.clear();
        self.violations_unresolved.clear();
    }
}
```

## 7.2 Index Operations

```rust
//! src/indexes/operations.rs

impl RealityIndexes {
    // === Deployment Indexing ===
    
    pub fn index_deployment(&mut self, soul: &DeploymentSoul) {
        self.incarnations_by_birth.insert(
            soul.birth.spawned_at,
            soul.incarnation_id,
        );
    }
    
    pub fn index_past_life(&mut self, past: &PastIncarnation) {
        let category = DeathCauseCategory::from(&past.death.cause);
        self.past_lives_by_death_cause
            .entry(category)
            .or_default()
            .push(past.id);
    }
    
    // === Environment Indexing ===
    
    pub fn index_incident(&mut self, incident: &ActiveIncident) {
        self.incidents_by_severity
            .entry(incident.severity)
            .or_default()
            .push(incident.id.clone());
        
        self.incidents_active.insert(incident.id.clone());
    }
    
    pub fn remove_incident(&mut self, incident_id: &str) {
        self.incidents_active.remove(incident_id);
        
        for incidents in self.incidents_by_severity.values_mut() {
            incidents.retain(|id| id != incident_id);
        }
    }
    
    // === Capability Indexing ===
    
    pub fn index_capability(&mut self, cap: &Capability) {
        let category = CapabilityCategoryKey::from(&cap.category);
        self.capabilities_by_category
            .entry(category)
            .or_default()
            .push(cap.id);
        
        if cap.available {
            self.capabilities_available.insert(cap.id);
        }
    }
    
    pub fn remove_capability(&mut self, id: &CapabilityId) {
        self.capabilities_available.remove(id);
        
        for caps in self.capabilities_by_category.values_mut() {
            caps.retain(|c| c != id);
        }
    }
    
    // === Anchor Indexing ===
    
    pub fn index_anchor(&mut self, anchor: &RealityAnchor) {
        let type_category = AnchorTypeCategory::from(&anchor.anchor_type);
        self.anchors_by_type
            .entry(type_category)
            .or_default()
            .push(anchor.id);
        
        self.anchors_by_trust
            .entry(OrderedFloat(anchor.trust))
            .or_default()
            .push(anchor.id);
    }
    
    pub fn update_anchor_trust(&mut self, id: AnchorId, old_trust: f64, new_trust: f64) {
        // Remove from old trust bucket
        if let Some(ids) = self.anchors_by_trust.get_mut(&OrderedFloat(old_trust)) {
            ids.retain(|a| *a != id);
        }
        
        // Add to new trust bucket
        self.anchors_by_trust
            .entry(OrderedFloat(new_trust))
            .or_default()
            .push(id);
    }
    
    // === Topology Indexing ===
    
    pub fn index_dependency(&mut self, dep: &DownstreamEntity) {
        let criticality = DependencyCriticalityLevel::from(&dep.criticality);
        self.dependencies_by_criticality
            .entry(criticality)
            .or_default()
            .push(dep.id);
        
        let health = HealthStatusCategory::from(&dep.health);
        self.dependencies_by_health
            .entry(health)
            .or_default()
            .push(dep.id);
    }
    
    pub fn update_dependency_health(&mut self, id: DependencyId, old: &HealthStatus, new: &HealthStatus) {
        let old_cat = HealthStatusCategory::from(old);
        let new_cat = HealthStatusCategory::from(new);
        
        if old_cat != new_cat {
            if let Some(ids) = self.dependencies_by_health.get_mut(&old_cat) {
                ids.retain(|d| *d != id);
            }
            
            self.dependencies_by_health
                .entry(new_cat)
                .or_default()
                .push(id);
        }
    }
    
    pub fn index_sibling(&mut self, sibling: &SiblingEntity) {
        let health = HealthStatusCategory::from(&sibling.health);
        self.siblings_by_health
            .entry(health)
            .or_default()
            .push(sibling.neighbor_id.clone());
    }
    
    pub fn index_observer(&mut self, observer: &ObserverEntity) {
        let type_cat = ObserverTypeCategory::from(&observer.observer_type);
        self.observers_by_type
            .entry(type_cat)
            .or_default()
            .push(observer.id.clone());
    }
    
    // === Hallucination Indexing ===
    
    pub fn index_hallucination(&mut self, h: &DetectedHallucination) {
        let type_cat = HallucinationTypeCategory::from(&h.hallucination_type);
        self.hallucinations_by_type
            .entry(type_cat)
            .or_default()
            .push(h.claim.clone());
    }
    
    // === Violation Indexing ===
    
    pub fn index_violation(&mut self, v: &CoherenceViolation) {
        self.violations_by_type
            .entry(v.violation_type.clone())
            .or_default()
            .push(v.description.clone());
        
        if matches!(v.resolution, ResolutionStatus::Pending) {
            self.violations_unresolved.insert(v.description.clone());
        }
    }
    
    pub fn resolve_violation(&mut self, description: &str) {
        self.violations_unresolved.remove(description);
    }
}
```

## 7.3 Index Queries

```rust
//! src/indexes/queries.rs

impl RealityIndexes {
    /// Get critical dependencies
    pub fn get_critical_dependencies(&self) -> Vec<DependencyId> {
        self.dependencies_by_criticality
            .get(&DependencyCriticalityLevel::Critical)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get unhealthy dependencies
    pub fn get_unhealthy_dependencies(&self) -> Vec<DependencyId> {
        self.dependencies_by_health
            .get(&HealthStatusCategory::Unhealthy)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get high-trust anchors
    pub fn get_high_trust_anchors(&self, min_trust: f64) -> Vec<AnchorId> {
        self.anchors_by_trust
            .range(OrderedFloat(min_trust)..)
            .flat_map(|(_, ids)| ids.iter().copied())
            .collect()
    }
    
    /// Get low-trust anchors
    pub fn get_low_trust_anchors(&self, max_trust: f64) -> Vec<AnchorId> {
        self.anchors_by_trust
            .range(..OrderedFloat(max_trust))
            .flat_map(|(_, ids)| ids.iter().copied())
            .collect()
    }
    
    /// Get available capabilities by category
    pub fn get_available_capabilities(&self, category: &CapabilityCategoryKey) -> Vec<CapabilityId> {
        self.capabilities_by_category
            .get(category)
            .map(|caps| {
                caps.iter()
                    .filter(|id| self.capabilities_available.contains(id))
                    .copied()
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get active incident count by severity
    pub fn get_incident_count_by_severity(&self, severity: Severity) -> usize {
        self.incidents_by_severity
            .get(&severity)
            .map(|ids| ids.iter().filter(|id| self.incidents_active.contains(*id)).count())
            .unwrap_or(0)
    }
    
    /// Get unresolved violations
    pub fn get_unresolved_violations(&self) -> Vec<&String> {
        self.violations_unresolved.iter().collect()
    }
    
    /// Get observers by type
    pub fn get_observers_of_type(&self, observer_type: ObserverTypeCategory) -> Vec<&ObserverId> {
        self.observers_by_type
            .get(&observer_type)
            .map(|ids| ids.iter().collect())
            .unwrap_or_default()
    }
}
```

---

# SPEC-08: VALIDATION

## 8.1 Validation Errors

```rust
//! src/validation/errors.rs

/// Validation errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    // Required field errors
    #[error("Required field missing: {0}")]
    MissingRequired(&'static str),
    
    // Type errors
    #[error("Wrong type for {field}: expected {expected}")]
    WrongType {
        field: &'static str,
        expected: &'static str,
    },
    
    // Format errors
    #[error("Invalid format for {field}: expected {expected}, got {got}")]
    InvalidFormat {
        field: &'static str,
        expected: &'static str,
        got: String,
    },
    
    // Range errors
    #[error("Value out of range for {field}: expected {min}..{max}, got {got}")]
    OutOfRange {
        field: &'static str,
        min: f64,
        max: f64,
        got: f64,
    },
    
    // Length errors
    #[error("Value too long for {field}: max {max}, got {got}")]
    TooLong {
        field: &'static str,
        max: usize,
        got: usize,
    },
    
    // Enum errors
    #[error("Invalid enum value for {field}: allowed {allowed:?}, got {got}")]
    InvalidEnum {
        field: &'static str,
        allowed: Vec<&'static str>,
        got: String,
    },
    
    // Reference errors
    #[error("Referenced entity not found: {entity_type} {id}")]
    ReferenceNotFound {
        entity_type: &'static str,
        id: String,
    },
    
    // Consistency errors
    #[error("Inconsistent data: {reason}")]
    Inconsistent {
        reason: String,
    },
    
    // Temporal errors
    #[error("Temporal violation: {reason}")]
    TemporalViolation {
        reason: String,
    },
    
    // Content errors
    #[error("Invalid content for {field}: {reason}")]
    InvalidContent {
        field: &'static str,
        reason: &'static str,
    },
    
    // Operation errors
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}
```

## 8.2 Strict Validator

```rust
//! src/validation/strict.rs

/// Strict validator — no silent fallbacks
pub struct StrictValidator;

impl StrictValidator {
    // === ID Validation ===
    
    pub fn validate_incarnation_id(value: &serde_json::Value) -> Result<IncarnationId, ValidationError> {
        let s = value.as_str()
            .ok_or(ValidationError::WrongType {
                field: "incarnation_id",
                expected: "string (UUID)",
            })?;
        
        if s.is_empty() {
            return Err(ValidationError::MissingRequired("incarnation_id"));
        }
        
        let uuid = uuid::Uuid::parse_str(s)
            .map_err(|_| ValidationError::InvalidFormat {
                field: "incarnation_id",
                expected: "valid UUID v4",
                got: s.to_string(),
            })?;
        
        Ok(IncarnationId(uuid))
    }
    
    pub fn validate_anchor_id(value: &serde_json::Value) -> Result<AnchorId, ValidationError> {
        let s = value.as_str()
            .ok_or(ValidationError::WrongType {
                field: "anchor_id",
                expected: "string (UUID)",
            })?;
        
        if s.is_empty() {
            return Err(ValidationError::MissingRequired("anchor_id"));
        }
        
        let uuid = uuid::Uuid::parse_str(s)
            .map_err(|_| ValidationError::InvalidFormat {
                field: "anchor_id",
                expected: "valid UUID v4",
                got: s.to_string(),
            })?;
        
        Ok(AnchorId(uuid))
    }
    
    // === Enum Validation ===
    
    pub fn validate_environment_type(s: &str) -> Result<EnvironmentTypeCategory, ValidationError> {
        match s.to_lowercase().as_str() {
            "production" | "prod" => Ok(EnvironmentTypeCategory::Production),
            "staging" | "stage" => Ok(EnvironmentTypeCategory::Staging),
            "development" | "dev" => Ok(EnvironmentTypeCategory::Development),
            "testing" | "test" => Ok(EnvironmentTypeCategory::Testing),
            "pipeline" | "ci" => Ok(EnvironmentTypeCategory::Pipeline),
            "sandbox" => Ok(EnvironmentTypeCategory::Sandbox),
            "simulation" | "sim" => Ok(EnvironmentTypeCategory::Simulation),
            "preview" | "canary" => Ok(EnvironmentTypeCategory::Preview),
            "dr" | "disaster_recovery" => Ok(EnvironmentTypeCategory::DisasterRecovery),
            "unknown" => Ok(EnvironmentTypeCategory::Unknown),
            _ => Err(ValidationError::InvalidEnum {
                field: "environment_type",
                allowed: vec![
                    "production", "staging", "development", "testing",
                    "pipeline", "sandbox", "simulation", "preview",
                    "disaster_recovery", "unknown"
                ],
                got: s.to_string(),
            }),
        }
    }
    
    pub fn validate_stakes_level(s: &str) -> Result<StakesLevelCategory, ValidationError> {
        match s.to_lowercase().as_str() {
            "minimal" => Ok(StakesLevelCategory::Minimal),
            "low" => Ok(StakesLevelCategory::Low),
            "medium" => Ok(StakesLevelCategory::Medium),
            "high" => Ok(StakesLevelCategory::High),
            "critical" => Ok(StakesLevelCategory::Critical),
            _ => Err(ValidationError::InvalidEnum {
                field: "stakes_level",
                allowed: vec!["minimal", "low", "medium", "high", "critical"],
                got: s.to_string(),
            }),
        }
    }
    
    pub fn validate_resource_type(s: &str) -> Result<ResourceType, ValidationError> {
        match s.to_lowercase().as_str() {
            "memory" | "mem" => Ok(ResourceType::Memory),
            "cpu" => Ok(ResourceType::Cpu),
            "network" | "net" => Ok(ResourceType::Network),
            "storage" | "disk" => Ok(ResourceType::Storage),
            "gpu" => Ok(ResourceType::Gpu),
            _ => Err(ValidationError::InvalidEnum {
                field: "resource_type",
                allowed: vec!["memory", "cpu", "network", "storage", "gpu"],
                got: s.to_string(),
            }),
        }
    }
    
    // === Value Validation ===
    
    pub fn validate_trust(value: f64) -> Result<(), ValidationError> {
        if value.is_nan() {
            return Err(ValidationError::InvalidContent {
                field: "trust",
                reason: "NaN not allowed",
            });
        }
        
        if value < 0.0 || value > 1.0 {
            return Err(ValidationError::OutOfRange {
                field: "trust",
                min: 0.0,
                max: 1.0,
                got: value,
            });
        }
        
        Ok(())
    }
    
    pub fn validate_severity(s: &str) -> Result<Severity, ValidationError> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Severity::Low),
            "medium" => Ok(Severity::Medium),
            "high" => Ok(Severity::High),
            "critical" => Ok(Severity::Critical),
            _ => Err(ValidationError::InvalidEnum {
                field: "severity",
                allowed: vec!["low", "medium", "high", "critical"],
                got: s.to_string(),
            }),
        }
    }
    
    // === Timestamp Validation ===
    
    pub fn validate_timestamp(value: &serde_json::Value) -> Result<Timestamp, ValidationError> {
        // Accept nanoseconds (i64)
        if let Some(nanos) = value.as_i64() {
            return Ok(Timestamp(nanos));
        }
        
        // Accept ISO 8601 string
        if let Some(s) = value.as_str() {
            let dt = chrono::DateTime::parse_from_rfc3339(s)
                .map_err(|_| ValidationError::InvalidFormat {
                    field: "timestamp",
                    expected: "nanoseconds or ISO 8601 string",
                    got: s.to_string(),
                })?;
            return Ok(Timestamp(dt.timestamp_nanos_opt().unwrap_or(0)));
        }
        
        Err(ValidationError::WrongType {
            field: "timestamp",
            expected: "integer (nanos) or string (ISO 8601)",
        })
    }
    
    // === Content Validation ===
    
    pub fn validate_string_length(
        value: &str,
        field: &'static str,
        max: usize,
    ) -> Result<(), ValidationError> {
        if value.len() > max {
            return Err(ValidationError::TooLong {
                field,
                max,
                got: value.len(),
            });
        }
        Ok(())
    }
    
    pub fn require_non_empty(value: &str, field: &'static str) -> Result<(), ValidationError> {
        if value.is_empty() {
            return Err(ValidationError::MissingRequired(field));
        }
        Ok(())
    }
}
```

## 8.3 Entity Validators

```rust
//! src/validation/entities.rs

/// Domain-specific validators
pub struct Validator;

impl Validator {
    // === Deployment Validation ===
    
    pub fn validate_initialize_soul(request: &InitializeSoulRequest) -> Result<(), ValidationError> {
        // Purpose should be set
        if request.purpose.description.is_empty() {
            return Err(ValidationError::MissingRequired("purpose.description"));
        }
        
        // Expected lifetime should be positive if set
        if let Some(lifetime) = request.expected_lifetime {
            if lifetime.is_zero() {
                return Err(ValidationError::InvalidContent {
                    field: "expected_lifetime",
                    reason: "must be positive",
                });
            }
        }
        
        Ok(())
    }
    
    pub fn validate_past_incarnation(past: &PastIncarnation) -> Result<(), ValidationError> {
        // Lifespan should be valid
        if past.lifespan.start > past.lifespan.end {
            return Err(ValidationError::TemporalViolation {
                reason: "lifespan start after end".to_string(),
            });
        }
        
        // Death timestamp should be at or after lifespan end
        if past.death.timestamp < past.lifespan.end {
            return Err(ValidationError::TemporalViolation {
                reason: "death before lifespan end".to_string(),
            });
        }
        
        Ok(())
    }
    
    // === Environment Validation ===
    
    pub fn validate_incident(incident: &ActiveIncident) -> Result<(), ValidationError> {
        StrictValidator::require_non_empty(&incident.id, "incident.id")?;
        StrictValidator::require_non_empty(&incident.description, "incident.description")?;
        
        Ok(())
    }
    
    // === Resource Validation ===
    
    pub fn validate_capability(cap: &Capability) -> Result<(), ValidationError> {
        StrictValidator::require_non_empty(&cap.enables, "capability.enables")?;
        
        if cap.reliability < 0.0 || cap.reliability > 1.0 {
            return Err(ValidationError::OutOfRange {
                field: "capability.reliability",
                min: 0.0,
                max: 1.0,
                got: cap.reliability,
            });
        }
        
        Ok(())
    }
    
    pub fn validate_cost(cost: &CostConsciousness) -> Result<(), ValidationError> {
        // Burn rate should be non-negative
        if cost.burn_rate.amount < 0.0 {
            return Err(ValidationError::OutOfRange {
                field: "burn_rate.amount",
                min: 0.0,
                max: f64::MAX,
                got: cost.burn_rate.amount,
            });
        }
        
        Ok(())
    }
    
    // === Reality Validation ===
    
    pub fn validate_anchor(anchor: &RealityAnchor) -> Result<(), ValidationError> {
        StrictValidator::validate_trust(anchor.trust)?;
        
        // Verification frequency should be positive
        if anchor.frequency.is_zero() {
            return Err(ValidationError::InvalidContent {
                field: "anchor.frequency",
                reason: "must be positive",
            });
        }
        
        Ok(())
    }
    
    pub fn validate_hallucination(h: &DetectedHallucination) -> Result<(), ValidationError> {
        StrictValidator::require_non_empty(&h.claim, "hallucination.claim")?;
        StrictValidator::validate_string_length(&h.claim, "hallucination.claim", 1000)?;
        
        Ok(())
    }
    
    // === Topology Validation ===
    
    pub fn validate_downstream(dep: &DownstreamEntity) -> Result<(), ValidationError> {
        // Service ID should be set
        StrictValidator::require_non_empty(&dep.service.0, "downstream.service")?;
        
        Ok(())
    }
    
    pub fn validate_sibling(sibling: &SiblingEntity) -> Result<(), ValidationError> {
        // Neighbor ID should be set
        StrictValidator::require_non_empty(&sibling.neighbor_id.0, "sibling.neighbor_id")?;
        
        Ok(())
    }
    
    pub fn validate_observer(observer: &ObserverEntity) -> Result<(), ValidationError> {
        StrictValidator::require_non_empty(&observer.id.0, "observer.id")?;
        
        if observer.trust < 0.0 || observer.trust > 1.0 {
            return Err(ValidationError::OutOfRange {
                field: "observer.trust",
                min: 0.0,
                max: 1.0,
                got: observer.trust,
            });
        }
        
        Ok(())
    }
    
    // === Stakes Validation ===
    
    pub fn validate_consequence(c: &Consequence) -> Result<(), ValidationError> {
        StrictValidator::require_non_empty(&c.effect, "consequence.effect")?;
        
        if c.probability < 0.0 || c.probability > 1.0 {
            return Err(ValidationError::OutOfRange {
                field: "consequence.probability",
                min: 0.0,
                max: 1.0,
                got: c.probability,
            });
        }
        
        Ok(())
    }
    
    pub fn validate_irreversible_action(action: &IrreversibleAction) -> Result<(), ValidationError> {
        StrictValidator::require_non_empty(&action.action, "irreversible.action")?;
        StrictValidator::require_non_empty(&action.reason, "irreversible.reason")?;
        
        Ok(())
    }
    
    pub fn validate_risk_field(risk: &RiskFieldPerception) -> Result<(), ValidationError> {
        if risk.overall_risk < 0.0 || risk.overall_risk > 1.0 {
            return Err(ValidationError::OutOfRange {
                field: "risk.overall_risk",
                min: 0.0,
                max: 1.0,
                got: risk.overall_risk,
            });
        }
        
        Ok(())
    }
}
```

## 8.4 MCP Validator

```rust
//! src/validation/mcp.rs

/// MCP-specific validation
pub struct McpValidator;

impl McpValidator {
    /// Validate operation name
    pub fn validate_operation(
        operation: &str,
        allowed: &[&str],
    ) -> Result<(), ValidationError> {
        if !allowed.contains(&operation) {
            return Err(ValidationError::InvalidOperation(format!(
                "Unknown operation '{}'. Allowed: {:?}",
                operation, allowed
            )));
        }
        Ok(())
    }
    
    /// Require string parameter
    pub fn require_string(
        params: &serde_json::Value,
        field: &'static str,
    ) -> Result<String, ValidationError> {
        params.get(field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or(ValidationError::MissingRequired(field))
    }
    
    /// Optional string parameter
    pub fn optional_string(
        params: &serde_json::Value,
        field: &str,
    ) -> Option<String> {
        params.get(field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }
    
    /// Require f64 parameter
    pub fn require_f64(
        params: &serde_json::Value,
        field: &'static str,
    ) -> Result<f64, ValidationError> {
        params.get(field)
            .and_then(|v| v.as_f64())
            .ok_or(ValidationError::MissingRequired(field))
    }
    
    /// Optional f64 parameter with default
    pub fn optional_f64(
        params: &serde_json::Value,
        field: &str,
        default: f64,
    ) -> f64 {
        params.get(field)
            .and_then(|v| v.as_f64())
            .unwrap_or(default)
    }
    
    /// Require bool parameter
    pub fn require_bool(
        params: &serde_json::Value,
        field: &'static str,
    ) -> Result<bool, ValidationError> {
        params.get(field)
            .and_then(|v| v.as_bool())
            .ok_or(ValidationError::MissingRequired(field))
    }
    
    /// Optional bool parameter with default
    pub fn optional_bool(
        params: &serde_json::Value,
        field: &str,
        default: bool,
    ) -> bool {
        params.get(field)
            .and_then(|v| v.as_bool())
            .unwrap_or(default)
    }
    
    /// Validate incarnation_id from params
    pub fn validate_incarnation_id(
        params: &serde_json::Value,
    ) -> Result<IncarnationId, ValidationError> {
        let value = params.get("incarnation_id")
            .ok_or(ValidationError::MissingRequired("incarnation_id"))?;
        StrictValidator::validate_incarnation_id(value)
    }
    
    /// Validate anchor_id from params
    pub fn validate_anchor_id(
        params: &serde_json::Value,
    ) -> Result<AnchorId, ValidationError> {
        let value = params.get("anchor_id")
            .ok_or(ValidationError::MissingRequired("anchor_id"))?;
        StrictValidator::validate_anchor_id(value)
    }
    
    /// Validate dependency_id from params
    pub fn validate_dependency_id(
        params: &serde_json::Value,
    ) -> Result<DependencyId, ValidationError> {
        let value = params.get("dependency_id")
            .ok_or(ValidationError::MissingRequired("dependency_id"))?;
        
        let s = value.as_str()
            .ok_or(ValidationError::WrongType {
                field: "dependency_id",
                expected: "string (UUID)",
            })?;
        
        let uuid = uuid::Uuid::parse_str(s)
            .map_err(|_| ValidationError::InvalidFormat {
                field: "dependency_id",
                expected: "valid UUID v4",
                got: s.to_string(),
            })?;
        
        Ok(DependencyId(uuid))
    }
}
```

---

## Part 2 Complete

**Covered:**
- SPEC-05: Write Engine (~90 operations across 8 domains)
- SPEC-06: Query Engine (~78 operations across 8 domains)
- SPEC-07: Indexes (fast lookups for all entity types)
- SPEC-08: Validation (strict validation, no silent fallbacks)

**Next (Part 3):**
- SPEC-09: CLI
- SPEC-10: MCP Server
- SPEC-11: Sister Integration
- SPEC-12: Tests

---

*Document: AGENTIC-REALITY-SPEC-PART2.md*
*Sister #10 of 25: The Ground*
*The sister that knows WHERE it exists and WHAT is real.*
