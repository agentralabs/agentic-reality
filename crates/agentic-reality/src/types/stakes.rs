//! Stakes perception types — "What are the consequences?"

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Consequence awareness — understanding impacts of actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsequenceAwareness {
    pub stakes: StakesLevel,
    pub consequence_map: Vec<Consequence>,
    pub irreversible: Vec<IrreversibleAction>,
    pub safety_margins: SafetyMargins,
    pub history: Vec<ConsequenceRecord>,
}

/// Level of stakes for the current context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StakesLevel {
    Minimal { can_experiment: bool },
    Low { rollback_available: bool },
    Medium { review_recommended: bool },
    High { caution_required: bool, approval_needed: bool },
    Critical { multiple_approvals: bool, audit_required: bool, no_risk_tolerance: bool },
}

impl std::fmt::Display for StakesLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minimal { .. } => write!(f, "minimal"),
            Self::Low { .. } => write!(f, "low"),
            Self::Medium { .. } => write!(f, "medium"),
            Self::High { .. } => write!(f, "high"),
            Self::Critical { .. } => write!(f, "critical"),
        }
    }
}

/// A potential consequence of an action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consequence {
    pub effect: String,
    pub probability: f64,
    pub severity: Severity,
    pub reversibility: Reversibility,
    pub affected: Vec<String>,
    pub latency_secs: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Negligible,
    Minor,
    Moderate,
    Major,
    Catastrophic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reversibility {
    Easy,
    WithEffort,
    Partial,
    Irreversible,
}

/// An action that cannot be undone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrreversibleAction {
    pub action: String,
    pub consequences: Vec<String>,
    pub safeguards: Vec<String>,
    pub requires_approval: bool,
}

/// Safety margins for operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyMargins {
    pub overall: f64,
    pub by_domain: HashMap<String, f64>,
    pub guardrails: Vec<Guardrail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guardrail {
    pub name: String,
    pub description: String,
    pub active: bool,
    pub triggered_count: u32,
}

/// Record of a consequence that occurred.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsequenceRecord {
    pub action: String,
    pub consequence: String,
    pub severity: Severity,
    pub timestamp: i64,
    pub expected: bool,
}

/// Risk field perception — risk as a continuous field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFieldPerception {
    pub overall_risk: f64,
    pub risk_map: HashMap<RiskCategory, RiskLevel>,
    pub hotspots: Vec<RiskHotspot>,
    pub safe_zones: Vec<String>,
    pub gradients: Vec<RiskGradient>,
    pub forecast: Vec<RiskForecast>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskCategory {
    DataRisk,
    SecurityRisk,
    AvailabilityRisk,
    PerformanceRisk,
    FinancialRisk,
    ComplianceRisk,
    ReputationRisk,
    UserHarmRisk,
}

impl std::fmt::Display for RiskCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataRisk => write!(f, "data"),
            Self::SecurityRisk => write!(f, "security"),
            Self::AvailabilityRisk => write!(f, "availability"),
            Self::PerformanceRisk => write!(f, "performance"),
            Self::FinancialRisk => write!(f, "financial"),
            Self::ComplianceRisk => write!(f, "compliance"),
            Self::ReputationRisk => write!(f, "reputation"),
            Self::UserHarmRisk => write!(f, "user_harm"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLevel {
    pub score: f64,
    pub trend: RiskTrend,
    pub mitigations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskTrend {
    Increasing,
    Stable,
    Decreasing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskHotspot {
    pub area: String,
    pub risk_score: f64,
    pub categories: Vec<RiskCategory>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskGradient {
    pub from: String,
    pub to: String,
    pub gradient: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskForecast {
    pub category: RiskCategory,
    pub horizon_secs: u64,
    pub predicted_level: f64,
    pub confidence: f64,
}

/// Blast radius awareness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlastRadiusAwareness {
    pub my_blast_radius: BlastRadius,
    pub dependency_blast: Vec<DependencyBlast>,
    pub cascade: CascadeAnalysis,
    pub isolation: IsolationAssessment,
    pub containment: Vec<ContainmentStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlastRadius {
    pub affected_services: Vec<String>,
    pub affected_users: EstimatedImpact,
    pub affected_data: Vec<String>,
    pub geographic_scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatedImpact {
    pub count: u64,
    pub percentage: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyBlast {
    pub dependency: String,
    pub blast_radius: BlastRadius,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CascadeAnalysis {
    pub max_depth: u32,
    pub affected_count: u32,
    pub critical_path: Vec<String>,
    pub containment_possible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsolationAssessment {
    pub isolation_score: f64,
    pub blast_walls: Vec<String>,
    pub leaks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainmentStrategy {
    pub name: String,
    pub description: String,
    pub effectiveness: f64,
    pub cost: String,
}
