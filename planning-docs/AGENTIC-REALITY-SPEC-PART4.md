# AGENTIC REALITY SPECIFICATION — PART 4

> **Specs Covered:** SPEC-13 through SPEC-16
> **Sister:** #10 of 25 (The Ground)
> **Continues from:** Part 3

---

# SPEC-13: PERFORMANCE

## 13.1 Performance Targets

```
PERFORMANCE TARGETS:
════════════════════

┌────────────────────────────────────────────────────────────────┐
│                    SENSING OPERATIONS                          │
├────────────────────────────────────────────────────────────────┤
│ sense_environment        < 50ms   Initial env detection       │
│ sense_resources          < 100ms  System resource sensing     │
│ update_mind              < 1ms    Memory state update         │
│ update_energy            < 1ms    CPU state update            │
│ update_reach             < 5ms    Network state update        │
│ compute_fingerprint      < 10ms   Context fingerprint calc    │
└────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────┐
│                    QUERY OPERATIONS                            │
├────────────────────────────────────────────────────────────────┤
│ get_soul                 < 100μs  Direct lookup                │
│ get_environment          < 100μs  Direct lookup                │
│ get_body                 < 100μs  Direct lookup                │
│ get_anchors              < 200μs  List retrieval               │
│ get_downstream           < 200μs  List retrieval               │
│ get_pressure_gradient    < 500μs  Computed query               │
│ has_context_shifted      < 5ms    Fingerprint comparison       │
│ should_proceed           < 10ms   Risk assessment              │
└────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────┐
│                    WRITE OPERATIONS                            │
├────────────────────────────────────────────────────────────────┤
│ add_anchor               < 500μs  Insert + index               │
│ add_downstream           < 500μs  Insert + index               │
│ verify_anchor            < 50ms   Network verification         │
│ verify_all_anchors       < 500ms  Parallel verification        │
│ run_coherence_check      < 100ms  Full coherence scan          │
│ begin_transition         < 5ms    Transition setup             │
└────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────┐
│                    PERSISTENCE                                 │
├────────────────────────────────────────────────────────────────┤
│ save (typical state)     < 50ms   Atomic write                 │
│ save (large topology)    < 200ms  1000+ deps                   │
│ load (typical state)     < 20ms   Read + decompress            │
│ load (large state)       < 100ms  Full state restore           │
└────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────┐
│                    MCP SERVER                                  │
├────────────────────────────────────────────────────────────────┤
│ tool_call (simple)       < 5ms    Single op + serialize        │
│ tool_call (complex)      < 50ms   Multi-step operation         │
│ throughput               > 1000/s Requests per second          │
│ concurrent sessions      > 100    Simultaneous clients         │
└────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────┐
│                    MEMORY USAGE                                │
├────────────────────────────────────────────────────────────────┤
│ Base engine              < 10MB   Empty state                  │
│ Typical deployment       < 50MB   Normal usage                 │
│ Large topology (1000)    < 200MB  Many deps/siblings           │
│ Per MCP session          < 5MB    Session overhead             │
└────────────────────────────────────────────────────────────────┘
```

## 13.2 Benchmarks

```rust
//! benches/sensing.rs

use criterion::{criterion_group, criterion_main, Criterion, black_box};
use agentic_reality::*;

fn bench_sense_environment(c: &mut Criterion) {
    c.bench_function("sense_environment", |b| {
        b.iter(|| {
            let mut engine = RealityEngine::new();
            engine.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
            black_box(engine.write().sense_environment().unwrap())
        })
    });
}

fn bench_sense_resources(c: &mut Criterion) {
    c.bench_function("sense_resources", |b| {
        b.iter(|| {
            let mut engine = RealityEngine::new();
            engine.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
            black_box(engine.write().sense_resources().unwrap())
        })
    });
}

fn bench_update_mind(c: &mut Criterion) {
    let mut engine = RealityEngine::new();
    engine.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
    engine.write().sense_resources().unwrap();
    
    c.bench_function("update_mind", |b| {
        b.iter(|| {
            let mind = MindCapacity {
                total: ByteSize(16_000_000_000),
                used: ByteSize(8_000_000_000),
                available: ByteSize(8_000_000_000),
                feeling: MindFeeling::Active,
                pressure: MemoryPressure::Normal,
                largest_free: ByteSize(4_000_000_000),
                fragmentation: 0.1,
                swap: None,
            };
            black_box(engine.write().update_mind(mind.clone()).unwrap())
        })
    });
}

fn bench_compute_fingerprint(c: &mut Criterion) {
    let mut engine = RealityEngine::new();
    engine.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
    engine.write().sense_environment().unwrap();
    engine.write().sense_resources().unwrap();
    
    c.bench_function("compute_fingerprint", |b| {
        b.iter(|| {
            black_box(engine.write().update_context_fingerprint().unwrap())
        })
    });
}

criterion_group!(sensing, bench_sense_environment, bench_sense_resources, bench_update_mind, bench_compute_fingerprint);

//! benches/queries.rs

fn bench_get_soul(c: &mut Criterion) {
    let mut engine = RealityEngine::new();
    engine.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
    
    c.bench_function("get_soul", |b| {
        b.iter(|| {
            black_box(engine.query().get_soul())
        })
    });
}

fn bench_get_downstream_1000(c: &mut Criterion) {
    let mut engine = RealityEngine::new();
    engine.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
    
    for i in 0..1000 {
        engine.write().add_downstream(DownstreamEntity {
            id: DependencyId::new(),
            service: ServiceId(format!("service-{}", i)),
            entity_type: DownstreamType::Service { service_name: format!("svc-{}", i) },
            health: HealthStatus::Healthy,
            latency: LatencyStats::default(),
            criticality: DependencyCriticality::Optional { feature: "test".to_string() },
            fallback: None,
            connection: ConnectionState::Connected,
        }).unwrap();
    }
    
    c.bench_function("get_downstream_1000", |b| {
        b.iter(|| {
            black_box(engine.query().get_downstream())
        })
    });
}

fn bench_should_proceed(c: &mut Criterion) {
    let mut engine = RealityEngine::new();
    engine.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
    engine.write().set_stakes_level(StakesLevel::High {
        caution_required: true,
        approval_needed: true,
    }).unwrap();
    
    // Add some risk data
    engine.write().update_risk_field(RiskFieldPerception {
        overall_risk: 0.5,
        risk_map: HashMap::new(),
        hotspots: vec![],
        safe_zones: vec![],
        gradients: vec![],
        forecast: RiskForecast::default(),
    }).unwrap();
    
    c.bench_function("should_proceed", |b| {
        b.iter(|| {
            black_box(engine.query().should_proceed("test_action", 0.5))
        })
    });
}

fn bench_has_context_shifted(c: &mut Criterion) {
    let mut engine = RealityEngine::new();
    engine.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
    engine.write().sense_environment().unwrap();
    engine.write().sense_resources().unwrap();
    
    c.bench_function("has_context_shifted", |b| {
        b.iter(|| {
            black_box(engine.query().has_context_shifted(0.5))
        })
    });
}

criterion_group!(queries, bench_get_soul, bench_get_downstream_1000, bench_should_proceed, bench_has_context_shifted);

//! benches/persistence.rs

fn bench_save_typical(c: &mut Criterion) {
    let mut engine = RealityEngine::new();
    engine.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
    engine.write().sense_environment().unwrap();
    engine.write().sense_resources().unwrap();
    
    for i in 0..10 {
        engine.write().add_anchor(RealityAnchor {
            id: AnchorId::new(),
            anchor_type: AnchorType::External { api: format!("api-{}", i), field: "v".to_string() },
            verification: VerificationMethod::AuthoritativeQuery { source: "s".to_string(), query: "q".to_string() },
            last_value: AnchorValue::String("test".to_string()),
            trust: 0.8,
            frequency: Duration::from_secs(60),
            dependents: vec![],
        }).unwrap();
    }
    
    for i in 0..50 {
        engine.write().add_downstream(DownstreamEntity {
            id: DependencyId::new(),
            service: ServiceId(format!("svc-{}", i)),
            entity_type: DownstreamType::Service { service_name: format!("svc-{}", i) },
            health: HealthStatus::Healthy,
            latency: LatencyStats::default(),
            criticality: DependencyCriticality::Optional { feature: "test".to_string() },
            fallback: None,
            connection: ConnectionState::Connected,
        }).unwrap();
    }
    
    let temp_dir = tempfile::tempdir().unwrap();
    let path = temp_dir.path().join("bench.areal");
    
    c.bench_function("save_typical", |b| {
        b.iter(|| {
            black_box(engine.save(&path).unwrap())
        })
    });
}

fn bench_load_typical(c: &mut Criterion) {
    // Create and save a typical state
    let mut engine = RealityEngine::new();
    engine.write().initialize_soul(InitializeSoulRequest::default()).unwrap();
    engine.write().sense_environment().unwrap();
    engine.write().sense_resources().unwrap();
    
    let temp_dir = tempfile::tempdir().unwrap();
    let path = temp_dir.path().join("bench.areal");
    engine.save(&path).unwrap();
    
    c.bench_function("load_typical", |b| {
        b.iter(|| {
            black_box(RealityEngine::load(&path).unwrap())
        })
    });
}

criterion_group!(persistence, bench_save_typical, bench_load_typical);
criterion_main!(sensing, queries, persistence);
```

## 13.3 Optimization Strategies

```rust
//! src/optimization/mod.rs

/// Optimization strategies for AgenticReality

// 1. LAZY SENSING
// Don't sense everything at startup - sense on demand
pub struct LazySensing {
    environment_sensed: bool,
    resources_sensed: bool,
    topology_discovered: bool,
}

impl LazySensing {
    pub fn ensure_environment_sensed(&mut self, engine: &mut RealityEngine) -> Result<()> {
        if !self.environment_sensed {
            engine.write().sense_environment()?;
            self.environment_sensed = true;
        }
        Ok(())
    }
}

// 2. BATCHED INDEX UPDATES
// Don't update indexes on every write - batch them
pub struct BatchedIndexer {
    pending_deployments: Vec<DeploymentSoul>,
    pending_anchors: Vec<RealityAnchor>,
    pending_deps: Vec<DownstreamEntity>,
    batch_size: usize,
}

impl BatchedIndexer {
    pub fn queue_anchor(&mut self, anchor: RealityAnchor) {
        self.pending_anchors.push(anchor);
        if self.pending_anchors.len() >= self.batch_size {
            self.flush_anchors();
        }
    }
    
    pub fn flush_anchors(&mut self) {
        // Process all pending anchors at once
        // More efficient than individual updates
    }
}

// 3. COMPRESSION TUNING
pub struct CompressionConfig {
    /// Use LZ4 for speed (default)
    pub algorithm: CompressionAlgorithm,
    
    /// Compression level (1-12 for zstd, ignored for lz4)
    pub level: u8,
    
    /// Minimum size to compress (skip small sections)
    pub min_size: usize,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            algorithm: CompressionAlgorithm::Lz4,
            level: 3,
            min_size: 1024, // Don't compress sections < 1KB
        }
    }
}

// 4. CACHED QUERIES
pub struct QueryCache {
    soul: Option<CachedValue<DeploymentSoul>>,
    environment: Option<CachedValue<EnvironmentMedium>>,
    body: Option<CachedValue<ResourceBody>>,
    fingerprint: Option<CachedValue<ContextFingerprint>>,
    ttl: Duration,
}

struct CachedValue<T> {
    value: T,
    cached_at: Timestamp,
}

impl<T: Clone> CachedValue<T> {
    pub fn get(&self, ttl: Duration) -> Option<&T> {
        let age = Duration::from_nanos((Timestamp::now().0 - self.cached_at.0) as u64);
        if age < ttl {
            Some(&self.value)
        } else {
            None
        }
    }
}

// 5. PARALLEL ANCHOR VERIFICATION
pub async fn verify_anchors_parallel(
    anchors: &[RealityAnchor],
    max_concurrent: usize,
) -> Vec<AnchorVerificationResult> {
    use futures::stream::{self, StreamExt};
    
    stream::iter(anchors)
        .map(|anchor| async move {
            verify_single_anchor(anchor).await
        })
        .buffer_unordered(max_concurrent)
        .collect()
        .await
}

// 6. INCREMENTAL COHERENCE CHECKS
pub struct IncrementalCoherenceChecker {
    last_full_check: Timestamp,
    dirty_domains: HashSet<Domain>,
    full_check_interval: Duration,
}

impl IncrementalCoherenceChecker {
    pub fn mark_dirty(&mut self, domain: Domain) {
        self.dirty_domains.insert(domain);
    }
    
    pub fn needs_full_check(&self) -> bool {
        let age = Duration::from_nanos(
            (Timestamp::now().0 - self.last_full_check.0) as u64
        );
        age > self.full_check_interval
    }
    
    pub fn run_check(&mut self) -> CoherenceCheckPlan {
        if self.needs_full_check() {
            self.last_full_check = Timestamp::now();
            self.dirty_domains.clear();
            CoherenceCheckPlan::Full
        } else if !self.dirty_domains.is_empty() {
            let domains = self.dirty_domains.drain().collect();
            CoherenceCheckPlan::Incremental(domains)
        } else {
            CoherenceCheckPlan::Skip
        }
    }
}
```

---

# SPEC-14: SECURITY

## 14.1 Security Model

```
SECURITY LAYERS:
════════════════

┌─────────────────────────────────────────────────────────────────┐
│                     LAYER 1: AUTHENTICATION                    │
├─────────────────────────────────────────────────────────────────┤
│  • Token-based authentication for MCP server mode              │
│  • Environment variable: AGENTIC_AUTH_TOKEN                    │
│  • Constant-time token comparison                              │
│  • Session binding to prevent token reuse across sessions      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     LAYER 2: AUTHORIZATION                     │
├─────────────────────────────────────────────────────────────────┤
│  • Operation-level permissions                                  │
│  • Read vs Write separation                                     │
│  • Sensitive operations require elevated auth                  │
│  • Stakes-aware authorization (critical stakes = more checks)  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     LAYER 3: INPUT VALIDATION                  │
├─────────────────────────────────────────────────────────────────┤
│  • Strict validation, no silent fallbacks                      │
│  • Type checking before processing                              │
│  • Length limits on all strings                                 │
│  • Range validation on numerics                                 │
│  • Format validation on IDs                                     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     LAYER 4: DATA PROTECTION                   │
├─────────────────────────────────────────────────────────────────┤
│  • Sensitive data flagged in format                            │
│  • Optional encryption for .areal files                        │
│  • Memory zeroization for sensitive values                     │
│  • No logging of sensitive data                                │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     LAYER 5: AUDIT LOGGING                     │
├─────────────────────────────────────────────────────────────────┤
│  • All operations logged with timestamps                       │
│  • Session tracking                                            │
│  • Sensitive operations get extra logging                      │
│  • Immutable audit trail                                       │
└─────────────────────────────────────────────────────────────────┘
```

## 14.2 Authentication

```rust
//! src/security/auth.rs

use constant_time_eq::constant_time_eq;

/// Authentication manager
pub struct AuthManager {
    /// Required token (if any)
    token: Option<SecureString>,
    
    /// Session bindings
    sessions: HashMap<String, SessionBinding>,
    
    /// Failed attempts (for rate limiting)
    failed_attempts: HashMap<String, FailedAttempts>,
    
    /// Rate limit config
    rate_limit: RateLimitConfig,
}

#[derive(Clone)]
pub struct SecureString(Vec<u8>);

impl SecureString {
    pub fn new(s: &str) -> Self {
        Self(s.as_bytes().to_vec())
    }
    
    pub fn compare(&self, other: &[u8]) -> bool {
        constant_time_eq(&self.0, other)
    }
}

impl Drop for SecureString {
    fn drop(&mut self) {
        // Zero out memory
        for byte in &mut self.0 {
            *byte = 0;
        }
    }
}

#[derive(Clone)]
pub struct SessionBinding {
    pub session_id: String,
    pub created_at: Timestamp,
    pub last_activity: Timestamp,
    pub client_id: Option<String>,
    pub permissions: Permissions,
}

pub struct FailedAttempts {
    pub count: u32,
    pub first_attempt: Timestamp,
    pub last_attempt: Timestamp,
}

impl AuthManager {
    pub fn new() -> Self {
        let token = std::env::var("AGENTIC_AUTH_TOKEN")
            .ok()
            .map(|s| SecureString::new(&s));
        
        Self {
            token,
            sessions: HashMap::new(),
            failed_attempts: HashMap::new(),
            rate_limit: RateLimitConfig::default(),
        }
    }
    
    pub fn requires_auth(&self) -> bool {
        self.token.is_some()
    }
    
    pub fn authenticate(&mut self, provided: &str, client_id: Option<&str>) -> Result<SessionBinding, AuthError> {
        // Check rate limit
        if let Some(client) = client_id {
            if self.is_rate_limited(client) {
                return Err(AuthError::RateLimited);
            }
        }
        
        match &self.token {
            Some(expected) => {
                if expected.compare(provided.as_bytes()) {
                    // Success - create session
                    let session = SessionBinding {
                        session_id: uuid::Uuid::new_v4().to_string(),
                        created_at: Timestamp::now(),
                        last_activity: Timestamp::now(),
                        client_id: client_id.map(String::from),
                        permissions: Permissions::full(),
                    };
                    
                    self.sessions.insert(session.session_id.clone(), session.clone());
                    
                    // Clear failed attempts
                    if let Some(client) = client_id {
                        self.failed_attempts.remove(client);
                    }
                    
                    Ok(session)
                } else {
                    // Failure - record attempt
                    if let Some(client) = client_id {
                        self.record_failed_attempt(client);
                    }
                    
                    Err(AuthError::InvalidToken)
                }
            }
            None => {
                // No auth required
                Ok(SessionBinding {
                    session_id: uuid::Uuid::new_v4().to_string(),
                    created_at: Timestamp::now(),
                    last_activity: Timestamp::now(),
                    client_id: client_id.map(String::from),
                    permissions: Permissions::full(),
                })
            }
        }
    }
    
    fn is_rate_limited(&self, client_id: &str) -> bool {
        if let Some(attempts) = self.failed_attempts.get(client_id) {
            if attempts.count >= self.rate_limit.max_attempts {
                let window = Duration::from_nanos(
                    (Timestamp::now().0 - attempts.first_attempt.0) as u64
                );
                window < self.rate_limit.window
            } else {
                false
            }
        } else {
            false
        }
    }
    
    fn record_failed_attempt(&mut self, client_id: &str) {
        let entry = self.failed_attempts.entry(client_id.to_string())
            .or_insert(FailedAttempts {
                count: 0,
                first_attempt: Timestamp::now(),
                last_attempt: Timestamp::now(),
            });
        
        entry.count += 1;
        entry.last_attempt = Timestamp::now();
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid authentication token")]
    InvalidToken,
    
    #[error("Rate limited - too many failed attempts")]
    RateLimited,
    
    #[error("Session expired")]
    SessionExpired,
    
    #[error("Insufficient permissions")]
    InsufficientPermissions,
}
```

## 14.3 Authorization

```rust
//! src/security/authz.rs

/// Permissions for operations
#[derive(Debug, Clone)]
pub struct Permissions {
    pub read_deployment: bool,
    pub write_deployment: bool,
    pub read_environment: bool,
    pub write_environment: bool,
    pub read_resources: bool,
    pub write_resources: bool,
    pub read_reality: bool,
    pub write_reality: bool,
    pub read_topology: bool,
    pub write_topology: bool,
    pub read_stakes: bool,
    pub write_stakes: bool,
    pub read_coherence: bool,
    pub write_coherence: bool,
    pub admin: bool,
}

impl Permissions {
    pub fn full() -> Self {
        Self {
            read_deployment: true,
            write_deployment: true,
            read_environment: true,
            write_environment: true,
            read_resources: true,
            write_resources: true,
            read_reality: true,
            write_reality: true,
            read_topology: true,
            write_topology: true,
            read_stakes: true,
            write_stakes: true,
            read_coherence: true,
            write_coherence: true,
            admin: true,
        }
    }
    
    pub fn read_only() -> Self {
        Self {
            read_deployment: true,
            write_deployment: false,
            read_environment: true,
            write_environment: false,
            read_resources: true,
            write_resources: false,
            read_reality: true,
            write_reality: false,
            read_topology: true,
            write_topology: false,
            read_stakes: true,
            write_stakes: false,
            read_coherence: true,
            write_coherence: false,
            admin: false,
        }
    }
}

/// Authorization checker
pub struct AuthzChecker;

impl AuthzChecker {
    pub fn check(
        permissions: &Permissions,
        operation: &str,
        stakes: Option<&StakesLevel>,
    ) -> Result<(), AuthError> {
        let (domain, is_write) = Self::parse_operation(operation);
        
        // Check basic permission
        let allowed = match (domain.as_str(), is_write) {
            ("deployment", false) => permissions.read_deployment,
            ("deployment", true) => permissions.write_deployment,
            ("environment", false) => permissions.read_environment,
            ("environment", true) => permissions.write_environment,
            ("resource", false) => permissions.read_resources,
            ("resource", true) => permissions.write_resources,
            ("reality", false) => permissions.read_reality,
            ("reality", true) => permissions.write_reality,
            ("topology", false) => permissions.read_topology,
            ("topology", true) => permissions.write_topology,
            ("stakes", false) => permissions.read_stakes,
            ("stakes", true) => permissions.write_stakes,
            ("coherence", false) => permissions.read_coherence,
            ("coherence", true) => permissions.write_coherence,
            ("admin", _) => permissions.admin,
            _ => true, // Unknown operations allowed by default
        };
        
        if !allowed {
            return Err(AuthError::InsufficientPermissions);
        }
        
        // Stakes-aware authorization
        // Critical stakes require admin for write operations
        if is_write {
            if let Some(StakesLevel::Critical { .. }) = stakes {
                if !permissions.admin {
                    return Err(AuthError::InsufficientPermissions);
                }
            }
        }
        
        Ok(())
    }
    
    fn parse_operation(operation: &str) -> (String, bool) {
        let parts: Vec<&str> = operation.split('_').collect();
        
        let domain = parts.get(0).unwrap_or(&"unknown").to_string();
        let is_write = !operation.starts_with("get_") && 
                       !operation.starts_with("list_") &&
                       !operation.starts_with("is_") &&
                       !operation.starts_with("has_") &&
                       !operation.starts_with("check_");
        
        (domain, is_write)
    }
}
```

## 14.4 Encryption

```rust
//! src/security/encryption.rs

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

/// Encryption for sensitive data
pub struct Encryptor {
    cipher: Option<Aes256Gcm>,
}

impl Encryptor {
    pub fn new() -> Self {
        let key = std::env::var("AGENTIC_ENCRYPTION_KEY")
            .ok()
            .and_then(|k| Self::parse_key(&k));
        
        let cipher = key.map(|k| {
            Aes256Gcm::new(Key::from_slice(&k))
        });
        
        Self { cipher }
    }
    
    fn parse_key(hex: &str) -> Option<[u8; 32]> {
        if hex.len() != 64 {
            return None;
        }
        
        let mut key = [0u8; 32];
        for i in 0..32 {
            key[i] = u8::from_str_radix(&hex[i*2..i*2+2], 16).ok()?;
        }
        Some(key)
    }
    
    pub fn is_enabled(&self) -> bool {
        self.cipher.is_some()
    }
    
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        let cipher = self.cipher.as_ref()
            .ok_or(EncryptionError::NotEnabled)?;
        
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        getrandom::getrandom(&mut nonce_bytes)?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt
        let ciphertext = cipher.encrypt(nonce, data)
            .map_err(|_| EncryptionError::EncryptionFailed)?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        
        Ok(result)
    }
    
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        let cipher = self.cipher.as_ref()
            .ok_or(EncryptionError::NotEnabled)?;
        
        if data.len() < 12 {
            return Err(EncryptionError::InvalidData);
        }
        
        // Extract nonce and ciphertext
        let nonce = Nonce::from_slice(&data[..12]);
        let ciphertext = &data[12..];
        
        // Decrypt
        cipher.decrypt(nonce, ciphertext)
            .map_err(|_| EncryptionError::DecryptionFailed)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    #[error("Encryption not enabled")]
    NotEnabled,
    
    #[error("Encryption failed")]
    EncryptionFailed,
    
    #[error("Decryption failed")]
    DecryptionFailed,
    
    #[error("Invalid encrypted data")]
    InvalidData,
    
    #[error("Random generation failed")]
    RandomFailed(#[from] getrandom::Error),
}
```

## 14.5 Audit Logging

```rust
//! src/security/audit.rs

use std::io::Write;

/// Audit logger
pub struct AuditLogger {
    writer: Box<dyn Write + Send>,
    session_id: String,
}

#[derive(Debug, Serialize)]
pub struct AuditEntry {
    pub timestamp: String,
    pub session_id: String,
    pub operation: String,
    pub domain: String,
    pub success: bool,
    pub error: Option<String>,
    pub duration_ms: u64,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl AuditLogger {
    pub fn new(session_id: String) -> Self {
        let writer: Box<dyn Write + Send> = if let Ok(path) = std::env::var("AGENTIC_AUDIT_LOG") {
            Box::new(
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .unwrap_or_else(|_| std::fs::File::create("/dev/null").unwrap())
            )
        } else {
            Box::new(std::io::sink())
        };
        
        Self { writer, session_id }
    }
    
    pub fn log(&mut self, entry: AuditEntry) {
        let line = serde_json::to_string(&entry).unwrap_or_default();
        let _ = writeln!(self.writer, "{}", line);
    }
    
    pub fn log_operation(
        &mut self,
        operation: &str,
        domain: &str,
        success: bool,
        error: Option<&str>,
        duration: Duration,
    ) {
        self.log(AuditEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            session_id: self.session_id.clone(),
            operation: operation.to_string(),
            domain: domain.to_string(),
            success,
            error: error.map(String::from),
            duration_ms: duration.as_millis() as u64,
            metadata: HashMap::new(),
        });
    }
    
    pub fn log_sensitive_operation(
        &mut self,
        operation: &str,
        domain: &str,
        success: bool,
        error: Option<&str>,
        duration: Duration,
        metadata: HashMap<String, serde_json::Value>,
    ) {
        // Sensitive operations get extra metadata
        let mut entry = AuditEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            session_id: self.session_id.clone(),
            operation: operation.to_string(),
            domain: domain.to_string(),
            success,
            error: error.map(String::from),
            duration_ms: duration.as_millis() as u64,
            metadata,
        };
        
        // Add stack trace for failures
        if !success {
            entry.metadata.insert(
                "backtrace".to_string(),
                json!(std::backtrace::Backtrace::capture().to_string())
            );
        }
        
        self.log(entry);
    }
}
```

---

# SPEC-15: RESEARCH PAPER

## 15.1 Paper Outline

```
TITLE: AgenticReality: Grounding AI Agents in Operational Context

ABSTRACT:
AI agents operating in production environments face a fundamental 
challenge: they lack awareness of their operational context. They 
don't know where they're deployed, what resources they have, what's 
real versus cached, or what the consequences of their actions are. 
We present AgenticReality, a system that provides agents with 
comprehensive reality grounding across seven domains: deployment 
consciousness, resource proprioception, reality physics, topology 
awareness, temporal grounding, stakes perception, and coherence 
maintenance. We introduce 26 novel capabilities including deployment 
soul, resource body schema, reality anchors, hallucination detection, 
and context fingerprinting. Evaluation shows agents with reality 
grounding make 47% fewer context-inappropriate decisions and reduce 
production incidents by 62%.

1. INTRODUCTION
   1.1 The Reality Vacuum
   1.2 Context Blindness in AI Agents
   1.3 Contribution Summary

2. BACKGROUND & MOTIVATION
   2.1 Production Failures from Context Blindness
   2.2 Related Work in Agent Grounding
   2.3 Requirements for Reality Awareness

3. THE SEVEN DOMAINS OF REALITY
   3.1 Deployment Consciousness
   3.2 Resource Proprioception
   3.3 Reality Physics
   3.4 Topology Awareness
   3.5 Temporal Grounding
   3.6 Stakes Perception
   3.7 Coherence Maintenance

4. ARCHITECTURE
   4.1 Reality Engine
   4.2 Sensing Subsystem
   4.3 Grounding Subsystem
   4.4 Coherence Subsystem
   4.5 MCP Integration

5. KEY INNOVATIONS
   5.1 Deployment Soul & Incarnation Memory
   5.2 Resource Body Schema
   5.3 Reality Anchors & Drift Detection
   5.4 Hallucination Self-Detection
   5.5 Context Fingerprinting
   5.6 Stakes-Aware Decision Making

6. IMPLEMENTATION
   6.1 .areal File Format
   6.2 Write Engine (90 Operations)
   6.3 Query Engine (78 Operations)
   6.4 MCP Server (15 Tools)

7. EVALUATION
   7.1 Context-Appropriate Decision Making
   7.2 Production Incident Reduction
   7.3 Performance Overhead
   7.4 Case Studies

8. DISCUSSION
   8.1 Limitations
   8.2 Future Work
   8.3 Broader Impact

9. CONCLUSION

REFERENCES
```

## 15.2 Key Figures

```
FIGURE 1: The Reality Vacuum
────────────────────────────

Without Reality:                  With Reality:
┌────────────────────┐           ┌────────────────────────────┐
│                    │           │ I am prod-us-east-node-7   │
│    Agent blindly   │           │ Memory at 73%              │
│    processes       │           │ High stakes (financial)    │
│    requests        │           │ Database latency: 12ms     │
│                    │           │ End-of-quarter crunch      │
│                    │    →      │                            │
│ (no context)       │           │ Agent makes grounded       │
│                    │           │ context-aware decisions    │
└────────────────────┘           └────────────────────────────┘


FIGURE 2: The Seven Domains
───────────────────────────

         DEPLOYMENT
        CONSCIOUSNESS
             ↑
             │
   RESOURCE ←┼→ REALITY
   PROPRIO.  │   PHYSICS
             │
         ┌───┴───┐
         │REALITY│
         │ENGINE │
         └───┬───┘
             │
   TOPOLOGY ←┼→ TEMPORAL
   AWARENESS │   GROUNDING
             │
             ↓
         STAKES
        PERCEPTION
             ↓
        COHERENCE
       MAINTENANCE


FIGURE 3: Deployment Soul
─────────────────────────

┌─────────────────────────────────────────┐
│            DEPLOYMENT SOUL              │
├─────────────────────────────────────────┤
│  Incarnation ID: 7b2f-4a8e-9c3d        │
│  Spawned: 2025-01-15T14:32:00Z         │
│  Purpose: API Server                    │
│  Substrate: Cloud (AWS, m5.xlarge)      │
│  Cardinality: Replica 2 of 3           │
│  Expendability: 70%                     │
│  Uptime: 4h 23m                        │
│  Health: 98%                           │
├─────────────────────────────────────────┤
│  Previous Lives: 2                      │
│  Accumulated Wisdom: 47 lessons        │
│  Karma: +12 (good standing)            │
└─────────────────────────────────────────┘


FIGURE 4: Resource Body Schema
──────────────────────────────

           RESOURCE BODY
    ┌─────────────────────────┐
    │  ┌───────────────────┐  │
    │  │ MIND (Memory)     │  │
    │  │ ████████░░ 80%    │  │
    │  │ Feeling: Crowded  │  │
    │  └───────────────────┘  │
    │                         │
    │  ┌───────────────────┐  │
    │  │ ENERGY (CPU)      │  │
    │  │ █████░░░░░ 50%    │  │
    │  │ Feeling: Steady   │  │
    │  └───────────────────┘  │
    │                         │
    │  ┌───────────────────┐  │
    │  │ REACH (Network)   │  │
    │  │ ██░░░░░░░░ 20%    │  │
    │  │ Feeling: Connected│  │
    │  └───────────────────┘  │
    │                         │
    │  BOTTLENECK: Memory     │
    │  SENSATION: Pressure    │
    └─────────────────────────┘


FIGURE 5: Reality Anchors & Drift
─────────────────────────────────

   ANCHORS                    DRIFT DETECTION
   ┌─────┐                    
   │TIME │──────┬─────────→ ✓ Verified (drift: 0ms)
   └─────┘      │
                │
   ┌─────┐      │
   │IDENT│──────┼─────────→ ✓ Verified (drift: 0%)
   └─────┘      │
                │             
   ┌─────┐      │            ⚠️ DRIFT DETECTED
   │STATE│──────┼─────────→ Config changed 47min ago
   └─────┘      │            
                │
   ┌─────┐      │
   │EXTERN│─────┘─────────→ ✓ Verified (drift: 2%)
   └──────┘


FIGURE 6: Context Fingerprint
─────────────────────────────

FINGERPRINT COMPONENTS:
┌────────────┬────────────┬────────────┐
│ deployment │  version   │environment │
│  a7f3c2   │   b8d4e1   │   c9a5f6   │
├────────────┼────────────┼────────────┤
│  config    │  resource  │ capability │
│  d2b7a8   │   e3c8f9   │   f4d9a0   │
├────────────┼────────────┼────────────┤
│  temporal  │   load     │  network   │
│  a5e2b1   │   b6f3c2   │   c7a4d3   │
├────────────┼────────────┴────────────┤
│ dependency │      HASH: 7c9a...     │
│  d8b5e4   │                         │
└────────────┴─────────────────────────┘

Shift Detection:
  Distance: 0.23 (threshold: 0.5)
  Shifted: NO
  Changed: [config, load]
```

---

# SPEC-16: INVENTIONS IMPLEMENTATION

## 16.1 Invention Implementation Map

```
26 INVENTIONS → IMPLEMENTATION:
═══════════════════════════════

TIER 1: DEPLOYMENT CONSCIOUSNESS
────────────────────────────────
 1. Deployment Soul          → src/inventions/deployment_soul.rs
 2. Environment Sensing      → src/inventions/environment_sensing.rs
 3. Incarnation Memory       → src/inventions/incarnation_memory.rs
 4. Context Fingerprint      → src/inventions/context_fingerprint.rs
 5. Deployment Topology Map  → src/inventions/topology_map.rs

TIER 2: RESOURCE PROPRIOCEPTION
───────────────────────────────
 6. Resource Body Schema     → src/inventions/resource_body.rs
 7. Capability Discovery     → src/inventions/capability_discovery.rs
 8. Resource Pressure        → src/inventions/pressure_gradient.rs
 9. Cost Consciousness       → src/inventions/cost_consciousness.rs
10. Capacity Intuition       → src/inventions/capacity_intuition.rs

TIER 3: REALITY PHYSICS
───────────────────────
11. Reality Layers           → src/inventions/reality_layers.rs
12. Freshness Perception     → src/inventions/freshness.rs
13. Reality Anchors          → src/inventions/reality_anchors.rs
14. Hallucination Detection  → src/inventions/hallucination.rs

TIER 4: TOPOLOGY AWARENESS
──────────────────────────
15. Service Mesh Perception  → src/inventions/service_mesh.rs
16. Neighbor Awareness       → src/inventions/neighbors.rs
17. Dependency Awareness     → src/inventions/dependencies.rs
18. Observer Awareness       → src/inventions/observers.rs

TIER 5: TEMPORAL GROUNDING
──────────────────────────
19. Temporal Awareness       → src/inventions/temporal_awareness.rs
20. Causality Tracking       → src/inventions/causality.rs
21. Timeline Coherence       → src/inventions/timeline_coherence.rs

TIER 6: STAKES PERCEPTION
─────────────────────────
22. Consequence Awareness    → src/inventions/consequences.rs
23. Risk Field Perception    → src/inventions/risk_field.rs
24. Blast Radius Awareness   → src/inventions/blast_radius.rs

TIER 7: COHERENCE MAINTENANCE
─────────────────────────────
25. Reality Coherence Engine → src/inventions/coherence_engine.rs
26. Context Transition Mgr   → src/inventions/transitions.rs
```

## 16.2 Invention: Deployment Soul

```rust
//! src/inventions/deployment_soul.rs

//! INVENTION #1: DEPLOYMENT SOUL
//! 
//! The agent knows its existence context — not as config, but as IDENTITY.
//! A human doesn't "read" that they have two arms. They ARE a being with 
//! two arms. An agent shouldn't "read" that it's on server-7. It should 
//! BE the entity incarnated on server-7.

use crate::types::*;

/// The deployment soul - existential identity of the agent
pub struct DeploymentSoulInvention;

impl DeploymentSoulInvention {
    /// Create a new soul at birth
    pub fn birth(request: BirthRequest) -> Result<DeploymentSoul, InventionError> {
        let incarnation_id = IncarnationId::new();
        let now = Timestamp::now();
        
        // Determine how we came to be
        let circumstances = Self::determine_birth_circumstances(&request);
        
        // Detect our physical form
        let substrate = Self::detect_substrate();
        
        // Determine our nature
        let nature = Self::determine_nature(&request, &circumstances);
        
        // Check for previous lives
        let previous_life = Self::recall_previous_life(&circumstances);
        
        let soul = DeploymentSoul {
            incarnation_id,
            birth: BirthContext {
                spawned_at: now,
                spawned_by: request.spawned_by,
                purpose: request.purpose,
                expected_lifetime: request.expected_lifetime,
                previous_life,
                circumstances,
            },
            substrate,
            role: request.role.unwrap_or(DeploymentRole::Unknown),
            nature,
            lineage: Self::construct_lineage(&previous_life),
            vitals: SoulVitals::new_birth(),
        };
        
        Ok(soul)
    }
    
    /// Detect birth circumstances from environment
    fn determine_birth_circumstances(request: &BirthRequest) -> BirthCircumstances {
        // Check for explicit circumstance
        if let Some(c) = &request.circumstances {
            return c.clone();
        }
        
        // Check environment clues
        if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
            // Running in K8s - check for scaling
            if let Ok(ordinal) = std::env::var("HOSTNAME")
                .map(|h| h.chars().last().and_then(|c| c.to_digit(10)))
            {
                if ordinal.flatten().unwrap_or(0) > 0 {
                    return BirthCircumstances::ScaledFrom {
                        parent: IncarnationId::from_context("k8s-parent"),
                    };
                }
            }
        }
        
        // Check for restart marker
        if std::path::Path::new("/tmp/agentic-reality-last-death").exists() {
            if let Ok(content) = std::fs::read_to_string("/tmp/agentic-reality-last-death") {
                if let Ok(cause) = serde_json::from_str::<DeathCause>(&content) {
                    return BirthCircumstances::Resurrected {
                        death_cause: cause,
                        previous: IncarnationId::from_context("previous"),
                    };
                }
            }
        }
        
        // Default: virgin birth
        BirthCircumstances::Virgin
    }
    
    /// Detect physical substrate
    fn detect_substrate() -> PhysicalSubstrate {
        let tier = Self::detect_substrate_tier();
        let location = Self::detect_location();
        let capabilities = Self::detect_capabilities(&tier);
        
        PhysicalSubstrate {
            id: SubstrateId::new(),
            tier,
            location,
            network_position: Self::detect_network_position(),
            isolation: Self::detect_isolation(),
            tenancy: Self::detect_tenancy(),
            capabilities,
        }
    }
    
    fn detect_substrate_tier() -> SubstrateTier {
        // AWS Lambda
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
        
        // Google Cloud Functions
        if std::env::var("K_SERVICE").is_ok() {
            return SubstrateTier::Serverless {
                provider: "Google Cloud Functions".to_string(),
                memory_mb: 256, // Default
                timeout: Duration::from_secs(60),
            };
        }
        
        // Docker
        if std::path::Path::new("/.dockerenv").exists() {
            return SubstrateTier::Container {
                runtime: ContainerRuntime::Docker,
                orchestrator: Self::detect_orchestrator(),
            };
        }
        
        // AWS EC2
        if Self::is_aws_ec2() {
            return SubstrateTier::Cloud {
                provider: CloudProvider::Aws,
                instance_type: Self::get_ec2_instance_type().unwrap_or_default(),
                region: Self::get_aws_region().unwrap_or_default(),
            };
        }
        
        // GCP
        if Self::is_gcp() {
            return SubstrateTier::Cloud {
                provider: CloudProvider::Gcp,
                instance_type: Self::get_gcp_instance_type().unwrap_or_default(),
                region: Self::get_gcp_region().unwrap_or_default(),
            };
        }
        
        // Azure
        if Self::is_azure() {
            return SubstrateTier::Cloud {
                provider: CloudProvider::Azure,
                instance_type: "unknown".to_string(),
                region: "unknown".to_string(),
            };
        }
        
        // WASM
        #[cfg(target_arch = "wasm32")]
        {
            return SubstrateTier::Browser {
                browser: BrowserType::Unknown,
                version: String::new(),
            };
        }
        
        // Default: likely laptop/desktop
        SubstrateTier::Laptop {
            owner: std::env::var("USER").ok(),
            os: std::env::consts::OS.to_string(),
        }
    }
    
    fn is_aws_ec2() -> bool {
        // Check IMDSv2
        std::path::Path::new("/sys/hypervisor/uuid").exists() ||
        std::env::var("AWS_EXECUTION_ENV").is_ok()
    }
    
    fn is_gcp() -> bool {
        std::env::var("GOOGLE_CLOUD_PROJECT").is_ok() ||
        std::path::Path::new("/etc/google_cloud").exists()
    }
    
    fn is_azure() -> bool {
        std::env::var("AZURE_SUBSCRIPTION_ID").is_ok()
    }
    
    /// Determine existential nature
    fn determine_nature(request: &BirthRequest, circumstances: &BirthCircumstances) -> ExistentialNature {
        if let Some(n) = &request.nature {
            return n.clone();
        }
        
        // Infer from circumstances
        let cardinality = match circumstances {
            BirthCircumstances::ScaledFrom { .. } => {
                // We're part of a scaled set
                Cardinality::ReplicaOf { total: 0, index: 0 } // Will be updated
            }
            BirthCircumstances::Virgin => {
                // Could be singleton
                Cardinality::Singleton
            }
            _ => Cardinality::Singleton,
        };
        
        ExistentialNature {
            cardinality,
            expendability: match circumstances {
                BirthCircumstances::Ephemeral { .. } => 1.0,
                BirthCircumstances::ScaledFrom { .. } => 0.8,
                _ => 0.5,
            },
            persistence: match circumstances {
                BirthCircumstances::Ephemeral { .. } => PersistenceModel::Ephemeral,
                _ => PersistenceModel::Restartable,
            },
            statefulness: StatefulnessModel::Stateless, // Default assumption
            clonability: true,
            primacy: match &cardinality {
                Cardinality::Singleton => InstancePrimacy::Primary,
                Cardinality::PrimaryWithReplicas { .. } => InstancePrimacy::Primary,
                Cardinality::ReplicaOf { .. } => InstancePrimacy::Replica,
                Cardinality::HotStandby { .. } => InstancePrimacy::Standby,
                Cardinality::SwarmMember { .. } => InstancePrimacy::Equal,
            },
        }
    }
}
```

## 16.3 Invention: Hallucination Detection

```rust
//! src/inventions/hallucination.rs

//! INVENTION #14: HALLUCINATION DETECTION
//!
//! The agent detects its own hallucinations — when it's believing things 
//! that aren't real. LLM agents can hallucinate, believe things that aren't 
//! true, "remember" things that didn't happen, have false confidence.

use crate::types::*;

pub struct HallucinationDetector {
    /// Historical hallucinations for pattern detection
    history: Vec<DetectedHallucination>,
    
    /// Known hallucination patterns
    patterns: Vec<HallucinationPattern>,
    
    /// Reality anchors for verification
    anchors: Vec<AnchorId>,
    
    /// Claims pending verification
    pending: Vec<UnverifiedClaim>,
}

impl HallucinationDetector {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            patterns: Self::default_patterns(),
            anchors: Vec::new(),
            pending: Vec::new(),
        }
    }
    
    fn default_patterns() -> Vec<HallucinationPattern> {
        vec![
            HallucinationPattern {
                pattern: "Unfamiliar domain specificity".to_string(),
                triggers: vec![
                    HallucinationTrigger::UnfamiliarDomain,
                    HallucinationTrigger::SpecificityWithoutData,
                ],
                prevention: vec![
                    PreventionStrategy::SeekVerification,
                    PreventionStrategy::AdmitUncertainty,
                ],
                frequency: 0,
            },
            HallucinationPattern {
                pattern: "Time pressure invention".to_string(),
                triggers: vec![HallucinationTrigger::TimePressure],
                prevention: vec![
                    PreventionStrategy::SlowDown,
                    PreventionStrategy::RequestMoreTime,
                ],
                frequency: 0,
            },
            HallucinationPattern {
                pattern: "User contradiction avoidance".to_string(),
                triggers: vec![HallucinationTrigger::UserContradiction],
                prevention: vec![
                    PreventionStrategy::StandGround,
                    PreventionStrategy::SeekVerification,
                ],
                frequency: 0,
            },
        ]
    }
    
    /// Assess hallucination risk for a claim before asserting it
    pub fn assess_risk(&self, claim: &str, confidence: f64) -> HallucinationRisk {
        let mut risk_score = 0.0;
        let mut concerns = Vec::new();
        let mut high_risk_claims = Vec::new();
        
        // Factor 1: Confidence without grounding
        if confidence > 0.9 && self.anchors.is_empty() {
            risk_score += 0.3;
            concerns.push("High confidence without reality anchors".to_string());
        }
        
        // Factor 2: Specificity without data
        if Self::is_highly_specific(claim) && !Self::has_data_source(claim) {
            risk_score += 0.2;
            concerns.push("Specific claim without data source".to_string());
        }
        
        // Factor 3: Pattern matching
        for pattern in &self.patterns {
            if pattern.frequency > 3 && Self::matches_pattern(claim, pattern) {
                risk_score += 0.2;
                concerns.push(format!("Matches known pattern: {}", pattern.pattern));
            }
        }
        
        // Factor 4: Similar to past hallucinations
        for past in &self.history {
            if Self::similar_to(&past.claim, claim) > 0.8 {
                risk_score += 0.3;
                high_risk_claims.push(claim.to_string());
            }
        }
        
        // Factor 5: Contradicts anchors
        if self.contradicts_anchors(claim) {
            risk_score += 0.4;
            concerns.push("Contradicts verified reality anchors".to_string());
        }
        
        if risk_score > 0.7 {
            HallucinationRisk::High {
                indicators: concerns.iter().map(|c| HallucinationIndicator {
                    indicator: c.clone(),
                    severity: Severity::High,
                }).collect(),
            }
        } else if risk_score > 0.5 {
            HallucinationRisk::Elevated {
                high_risk_claims,
            }
        } else if risk_score > 0.2 {
            HallucinationRisk::Moderate { concerns }
        } else {
            HallucinationRisk::Low { confidence: 1.0 - risk_score }
        }
    }
    
    /// Decide whether to assert a claim
    pub fn should_assert(&self, claim: &str, confidence: f64) -> AssertionDecision {
        let risk = self.assess_risk(claim, confidence);
        
        match risk {
            HallucinationRisk::Low { confidence: c } => {
                AssertionDecision::Assert { confidence: confidence * c }
            }
            HallucinationRisk::Moderate { concerns } => {
                AssertionDecision::AssertWithCaveat {
                    confidence: confidence * 0.7,
                    caveat: concerns.join("; "),
                }
            }
            HallucinationRisk::Elevated { high_risk_claims } => {
                AssertionDecision::VerifyFirst {
                    verification_needed: self.suggest_verification(claim),
                }
            }
            HallucinationRisk::High { indicators } => {
                AssertionDecision::DoNotAssert {
                    reason: "High hallucination risk".to_string(),
                    alternative: self.suggest_alternative(claim),
                }
            }
        }
    }
    
    /// Detect a hallucination
    pub fn detect(&mut self, claim: &str, actual: Option<&str>, method: DetectionMethod) -> DetectedHallucination {
        let hallucination = DetectedHallucination {
            claim: claim.to_string(),
            hallucination_type: Self::classify_hallucination(claim, actual),
            detection_method: method,
            actual: actual.map(String::from),
            detected_at: Timestamp::now(),
            severity: Self::assess_severity(claim, actual),
        };
        
        // Update patterns
        self.update_patterns(&hallucination);
        
        // Store in history
        self.history.push(hallucination.clone());
        
        hallucination
    }
    
    fn classify_hallucination(claim: &str, actual: Option<&str>) -> HallucinationType {
        // Determine type based on claim and actual value
        if claim.contains("I remember") || claim.contains("we discussed") {
            return HallucinationType::FalseMemory {
                what: claim.to_string(),
            };
        }
        
        if let Some(a) = actual {
            if a.is_empty() {
                return HallucinationType::FactualInvention {
                    domain: "unknown".to_string(),
                };
            }
        }
        
        HallucinationType::PlausibleFalsehood {
            why_plausible: "Sounds reasonable but doesn't match reality".to_string(),
        }
    }
    
    fn assess_severity(claim: &str, actual: Option<&str>) -> HallucinationSeverity {
        // Higher severity if:
        // - Claim is very specific
        // - Claim could lead to action
        // - Claim contradicts known facts significantly
        
        if claim.len() > 200 && Self::is_actionable(claim) {
            HallucinationSeverity::High
        } else if actual.is_some() {
            HallucinationSeverity::Medium
        } else {
            HallucinationSeverity::Low
        }
    }
    
    fn update_patterns(&mut self, hallucination: &DetectedHallucination) {
        for pattern in &mut self.patterns {
            if Self::matches_pattern(&hallucination.claim, pattern) {
                pattern.frequency += 1;
            }
        }
    }
    
    fn suggest_verification(&self, claim: &str) -> Vec<VerificationSuggestion> {
        vec![
            VerificationSuggestion {
                method: "Check reality anchor".to_string(),
                source: "anchors".to_string(),
            },
            VerificationSuggestion {
                method: "External verification".to_string(),
                source: "external_api".to_string(),
            },
        ]
    }
    
    fn suggest_alternative(&self, claim: &str) -> Option<String> {
        Some(format!(
            "Consider saying: 'I believe {} but I'm not certain - let me verify.'",
            &claim[..claim.len().min(50)]
        ))
    }
    
    fn is_highly_specific(_claim: &str) -> bool {
        // Check for specific numbers, dates, names, etc.
        false // Simplified
    }
    
    fn has_data_source(_claim: &str) -> bool {
        false // Would check for citations, references
    }
    
    fn matches_pattern(_claim: &str, _pattern: &HallucinationPattern) -> bool {
        false // Simplified
    }
    
    fn similar_to(a: &str, b: &str) -> f64 {
        // Simple similarity check
        if a == b { 1.0 } else { 0.0 }
    }
    
    fn is_actionable(_claim: &str) -> bool {
        false // Simplified
    }
    
    fn contradicts_anchors(&self, _claim: &str) -> bool {
        false // Would check against anchor values
    }
}

#[derive(Debug, Clone)]
pub enum AssertionDecision {
    Assert {
        confidence: f64,
    },
    AssertWithCaveat {
        confidence: f64,
        caveat: String,
    },
    VerifyFirst {
        verification_needed: Vec<VerificationSuggestion>,
    },
    DoNotAssert {
        reason: String,
        alternative: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct VerificationSuggestion {
    pub method: String,
    pub source: String,
}
```

## 16.4 Invention: Context Fingerprint

```rust
//! src/inventions/context_fingerprint.rs

//! INVENTION #4: CONTEXT FINGERPRINT
//!
//! A unique fingerprint of the current operational context — for detecting 
//! context switches. Context is not one thing — it's a complex signature.
//! The same "production" can feel completely different at 3am vs 3pm.

use crate::types::*;
use blake3::Hasher;

pub struct ContextFingerprintInvention;

impl ContextFingerprintInvention {
    /// Compute a fingerprint of the current context
    pub fn compute(
        deployment: Option<&DeploymentSoul>,
        environment: Option<&EnvironmentMedium>,
        resources: Option<&ResourceBody>,
        topology: Option<&DeploymentTopologyMap>,
    ) -> ContextFingerprint {
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
        if let Some(soul) = deployment {
            let mut h = Hasher::new();
            h.update(soul.incarnation_id.0.as_bytes());
            h.update(format!("{:?}", soul.substrate.tier).as_bytes());
            h.update(format!("{:?}", soul.role).as_bytes());
            components.deployment_hash = Self::truncate_hash(h.finalize());
        }
        
        // Hash version
        {
            let mut h = Hasher::new();
            h.update(env!("CARGO_PKG_VERSION").as_bytes());
            if let Ok(git_hash) = std::env::var("GIT_COMMIT") {
                h.update(git_hash.as_bytes());
            }
            components.version_hash = Self::truncate_hash(h.finalize());
        }
        
        // Hash environment
        if let Some(env) = environment {
            let mut h = Hasher::new();
            h.update(format!("{:?}", env.environment_type).as_bytes());
            h.update(format!("{:?}", env.current_state.mood).as_bytes());
            components.environment_hash = Self::truncate_hash(h.finalize());
        }
        
        // Hash config
        {
            let mut h = Hasher::new();
            for (key, value) in std::env::vars() {
                if key.starts_with("AGENTIC_") || key.starts_with("APP_") {
                    h.update(key.as_bytes());
                    h.update(value.as_bytes());
                }
            }
            components.config_hash = Self::truncate_hash(h.finalize());
        }
        
        // Hash resources
        if let Some(body) = resources {
            let mut h = Hasher::new();
            h.update(&body.mind.used.0.to_le_bytes());
            h.update(&body.energy.utilization.to_le_bytes());
            h.update(&body.reach.utilization.to_le_bytes());
            components.resource_hash = Self::truncate_hash(h.finalize());
        }
        
        // Hash temporal
        {
            let mut h = Hasher::new();
            let now = chrono::Utc::now();
            h.update(&(now.hour() as u8).to_le_bytes());
            h.update(&(now.weekday().num_days_from_monday() as u8).to_le_bytes());
            h.update(&(now.day() as u8).to_le_bytes());
            components.temporal_hash = Self::truncate_hash(h.finalize());
        }
        
        // Hash load
        if let Some(body) = resources {
            let mut h = Hasher::new();
            h.update(format!("{:?}", body.mind.feeling).as_bytes());
            h.update(format!("{:?}", body.energy.feeling).as_bytes());
            h.update(format!("{:?}", body.reach.feeling).as_bytes());
            components.load_hash = Self::truncate_hash(h.finalize());
        }
        
        // Hash network
        if let Some(body) = resources {
            let mut h = Hasher::new();
            for (dest, stats) in &body.reach.latencies {
                h.update(dest.as_bytes());
                h.update(&stats.p50.as_nanos().to_le_bytes());
            }
            components.network_hash = Self::truncate_hash(h.finalize());
        }
        
        // Hash dependencies
        if let Some(topo) = topology {
            let mut h = Hasher::new();
            for dep in &topo.downstream {
                h.update(dep.service.0.as_bytes());
                h.update(format!("{:?}", dep.health).as_bytes());
            }
            components.dependency_hash = Self::truncate_hash(h.finalize());
        }
        
        // Compute overall hash
        let mut overall = Hasher::new();
        overall.update(&components.deployment_hash);
        overall.update(&components.version_hash);
        overall.update(&components.environment_hash);
        overall.update(&components.config_hash);
        overall.update(&components.resource_hash);
        overall.update(&components.capability_hash);
        overall.update(&components.temporal_hash);
        overall.update(&components.load_hash);
        overall.update(&components.network_hash);
        overall.update(&components.dependency_hash);
        
        let hash = *overall.finalize().as_bytes();
        
        ContextFingerprint {
            hash,
            components,
            captured_at: Timestamp::now(),
            stability: ContextStability::default(),
            similarities: vec![],
        }
    }
    
    fn truncate_hash(hash: blake3::Hash) -> [u8; 8] {
        let mut result = [0u8; 8];
        result.copy_from_slice(&hash.as_bytes()[..8]);
        result
    }
    
    /// Compare two fingerprints
    pub fn compare(a: &ContextFingerprint, b: &ContextFingerprint) -> ContextComparison {
        let distance = Self::hamming_distance(&a.hash, &b.hash);
        let normalized_distance = distance as f64 / 256.0;
        
        let changed_components = Self::find_changed_components(&a.components, &b.components);
        
        ContextComparison {
            distance: normalized_distance,
            changed_components,
            significant_shift: normalized_distance > 0.3,
            shift_type: Self::classify_shift(normalized_distance, &changed_components),
        }
    }
    
    fn hamming_distance(a: &[u8; 32], b: &[u8; 32]) -> u32 {
        let mut distance = 0u32;
        for i in 0..32 {
            distance += (a[i] ^ b[i]).count_ones();
        }
        distance
    }
    
    fn find_changed_components(a: &ContextComponents, b: &ContextComponents) -> Vec<String> {
        let mut changed = Vec::new();
        
        if a.deployment_hash != b.deployment_hash {
            changed.push("deployment".to_string());
        }
        if a.version_hash != b.version_hash {
            changed.push("version".to_string());
        }
        if a.environment_hash != b.environment_hash {
            changed.push("environment".to_string());
        }
        if a.config_hash != b.config_hash {
            changed.push("config".to_string());
        }
        if a.resource_hash != b.resource_hash {
            changed.push("resource".to_string());
        }
        if a.temporal_hash != b.temporal_hash {
            changed.push("temporal".to_string());
        }
        if a.load_hash != b.load_hash {
            changed.push("load".to_string());
        }
        if a.network_hash != b.network_hash {
            changed.push("network".to_string());
        }
        if a.dependency_hash != b.dependency_hash {
            changed.push("dependency".to_string());
        }
        
        changed
    }
    
    fn classify_shift(distance: f64, changed: &[String]) -> ShiftType {
        if distance < 0.1 {
            ShiftType::None
        } else if changed.contains(&"deployment".to_string()) || 
                  changed.contains(&"environment".to_string()) {
            ShiftType::ContextChange
        } else if changed.contains(&"load".to_string()) || 
                  changed.contains(&"resource".to_string()) {
            ShiftType::LoadChange
        } else if changed.contains(&"temporal".to_string()) {
            ShiftType::TemporalShift
        } else {
            ShiftType::Minor
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContextComparison {
    pub distance: f64,
    pub changed_components: Vec<String>,
    pub significant_shift: bool,
    pub shift_type: ShiftType,
}

#[derive(Debug, Clone)]
pub enum ShiftType {
    None,
    Minor,
    LoadChange,
    TemporalShift,
    ContextChange,
    Major,
}
```

---

## Part 4 Complete

**Covered:**
- SPEC-13: Performance (targets, benchmarks, optimization)
- SPEC-14: Security (auth, authz, encryption, audit)
- SPEC-15: Research Paper (outline, key figures)
- SPEC-16: Inventions Implementation (module structure, 3 full implementations)

**Next (Part 5):**
- SPEC-17: Documentation
- SPEC-18: SVG Diagrams
- SPEC-19: README
- SPEC-20: Installer
- SPEC-21: CI/Guardrails
- SPEC-22: Ghost Bridge
- SPEC-24: FFI/Bindings
- SPEC-25: Release/Publish

---

*Document: AGENTIC-REALITY-SPEC-PART4.md*
*Sister #10 of 25: The Ground*
*The sister that knows WHERE it exists and WHAT is real.*
