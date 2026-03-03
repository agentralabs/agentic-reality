//! Environment types — "What medium do I exist in?"

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The operational environment as a living medium.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentMedium {
    pub environment_type: EnvironmentType,
    pub current_state: EnvironmentState,
    pub properties: EnvironmentProperties,
    pub physics: EnvironmentPhysics,
    pub inhabitants: Vec<String>,
    pub boundaries: Vec<String>,
    pub weather_history: Vec<WeatherEvent>,
}

/// What kind of environment the agent operates in.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentType {
    Production {
        tier: String,
        region: String,
        criticality: f64,
    },
    Staging {
        mirrors_production: bool,
        data_freshness: String,
    },
    Development {
        developer: String,
        local: bool,
    },
    Testing {
        test_type: String,
        isolation: bool,
    },
    Pipeline {
        pipeline_id: String,
        stage: String,
    },
    Sandbox {
        owner: String,
        expires: Option<i64>,
    },
    Simulation {
        fidelity: String,
        purpose: String,
    },
    Preview {
        traffic_percentage: f64,
        rollback_ready: bool,
    },
    DisasterRecovery {
        is_active_failover: bool,
        primary_region: String,
    },
    Unknown {
        clues: Vec<String>,
    },
}

/// Current state of the environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentState {
    pub health: EnvironmentHealth,
    pub pressure: EnvironmentPressure,
    pub stability: StabilityAssessment,
    pub incidents: Vec<ActiveIncident>,
    pub degradations: Vec<String>,
    pub mood: EnvironmentMood,
    pub last_sensed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentHealth {
    pub score: f64,
    pub components: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentPressure {
    pub level: f64,
    pub source: String,
    pub trend: PressureTrend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PressureTrend {
    Rising,
    Stable,
    Falling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityAssessment {
    pub score: f64,
    pub window_secs: u64,
    pub volatility: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveIncident {
    pub id: String,
    pub severity: String,
    pub summary: String,
    pub started_at: i64,
    pub acknowledged: bool,
}

/// The emotional state of the environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentMood {
    Calm,
    Busy,
    Stressed,
    Troubled,
    Crisis,
    Recovering,
    Maintenance,
    Dying,
}

impl std::fmt::Display for EnvironmentMood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Calm => write!(f, "calm"),
            Self::Busy => write!(f, "busy"),
            Self::Stressed => write!(f, "stressed"),
            Self::Troubled => write!(f, "troubled"),
            Self::Crisis => write!(f, "crisis"),
            Self::Recovering => write!(f, "recovering"),
            Self::Maintenance => write!(f, "maintenance"),
            Self::Dying => write!(f, "dying"),
        }
    }
}

/// Properties of the environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentProperties {
    pub name: String,
    pub version: Option<String>,
    pub config: HashMap<String, String>,
    pub labels: HashMap<String, String>,
}

/// The physics (constraints) governing this environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentPhysics {
    pub rate_limits: Vec<RateLimit>,
    pub cost_constraints: Vec<CostConstraint>,
    pub time_constraints: Vec<TimeConstraint>,
    pub quotas: Vec<Quota>,
    pub permissions: Vec<String>,
    pub forbidden_actions: Vec<String>,
    pub compliance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub resource: String,
    pub limit: u64,
    pub window_secs: u64,
    pub current: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostConstraint {
    pub resource: String,
    pub budget: f64,
    pub spent: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConstraint {
    pub name: String,
    pub deadline: Option<i64>,
    pub budget_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quota {
    pub resource: String,
    pub limit: u64,
    pub used: u64,
}

/// A weather event in the environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEvent {
    pub event_type: String,
    pub severity: f64,
    pub timestamp: i64,
    pub duration_secs: Option<u64>,
    pub description: String,
}

/// Unique hash fingerprint of entire operational context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFingerprint {
    pub hash: [u8; 32],
    pub components: ContextComponents,
    pub captured_at: i64,
    pub stability: ContextStability,
    pub similarities: Vec<FingerprintSimilarity>,
}

/// Individual hashed components making up the fingerprint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextComponents {
    pub deployment_hash: [u8; 8],
    pub version_hash: [u8; 8],
    pub environment_hash: [u8; 8],
    pub config_hash: [u8; 8],
    pub resource_hash: [u8; 8],
    pub capability_hash: [u8; 8],
    pub temporal_hash: [u8; 8],
    pub load_hash: [u8; 8],
    pub network_hash: [u8; 8],
    pub dependency_hash: [u8; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContextStability {
    Stable,
    Drifting,
    Shifting,
    Volatile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintSimilarity {
    pub context_id: String,
    pub similarity: f64,
    pub common_components: Vec<String>,
}
