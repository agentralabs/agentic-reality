//! Coherence maintenance types — "How do I stay grounded?"

use serde::{Deserialize, Serialize};

use super::ids::{ContextId, TransitionId};

/// Overall coherence state across all reality domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceState {
    pub level: CoherenceLevel,
    pub checks: Vec<CoherenceCheck>,
    pub violations: Vec<CoherenceViolation>,
    pub strategies: Vec<CoherenceStrategy>,
    pub history: Vec<CoherenceEvent>,
}

/// Level of coherence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoherenceLevel {
    Full { confidence: f64 },
    Minor { issues: Vec<String> },
    Significant { issues: Vec<String>, impact: String },
    Incoherent { reason: String, severity: f64 },
}

impl std::fmt::Display for CoherenceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full { .. } => write!(f, "full"),
            Self::Minor { .. } => write!(f, "minor"),
            Self::Significant { .. } => write!(f, "significant"),
            Self::Incoherent { .. } => write!(f, "incoherent"),
        }
    }
}

/// A coherence check performed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceCheck {
    pub check_type: CoherenceCheckType,
    pub passed: bool,
    pub details: String,
    pub timestamp: i64,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoherenceCheckType {
    TimeConsistency,
    StateConsistency,
    LayerConsistency,
    AnchorConsistency,
    MemoryConsistency,
    CausalConsistency,
}

impl std::fmt::Display for CoherenceCheckType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimeConsistency => write!(f, "time"),
            Self::StateConsistency => write!(f, "state"),
            Self::LayerConsistency => write!(f, "layer"),
            Self::AnchorConsistency => write!(f, "anchor"),
            Self::MemoryConsistency => write!(f, "memory"),
            Self::CausalConsistency => write!(f, "causal"),
        }
    }
}

/// A detected coherence violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceViolation {
    pub violation_type: CoherenceCheckType,
    pub detected: i64,
    pub severity: f64,
    pub evidence: Vec<String>,
    pub possible_causes: Vec<String>,
    pub resolution: Option<String>,
    pub description: String,
}

/// Strategy for maintaining coherence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceStrategy {
    pub name: String,
    pub description: String,
    pub active: bool,
    pub effectiveness: f64,
}

/// Historical coherence event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceEvent {
    pub event_type: CoherenceEventType,
    pub timestamp: i64,
    pub details: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoherenceEventType {
    CheckPassed,
    CheckFailed,
    ViolationDetected,
    ViolationResolved,
    TransitionStarted,
    TransitionCompleted,
}

/// Context transition state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionState {
    pub current: ContextId,
    pub pending: Vec<PendingTransition>,
    pub history: Vec<CompletedTransition>,
    pub rules: Vec<TransitionRule>,
}

/// A pending context transition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTransition {
    pub id: TransitionId,
    pub from: ContextId,
    pub to_context: String,
    pub transition_type: TransitionType,
    pub phase: TransitionPhase,
    pub carry_over: Vec<String>,
    pub started: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionType {
    EnvironmentChange,
    LayerChange,
    ScaleEvent,
    Migration,
    Failover,
    Upgrade,
    Restoration,
}

impl std::fmt::Display for TransitionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnvironmentChange => write!(f, "environment_change"),
            Self::LayerChange => write!(f, "layer_change"),
            Self::ScaleEvent => write!(f, "scale_event"),
            Self::Migration => write!(f, "migration"),
            Self::Failover => write!(f, "failover"),
            Self::Upgrade => write!(f, "upgrade"),
            Self::Restoration => write!(f, "restoration"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionPhase {
    Planning,
    Preparing,
    Validating,
    Executing,
    Verifying,
    Complete,
    Failed,
    RollingBack,
}

impl std::fmt::Display for TransitionPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Planning => write!(f, "planning"),
            Self::Preparing => write!(f, "preparing"),
            Self::Validating => write!(f, "validating"),
            Self::Executing => write!(f, "executing"),
            Self::Verifying => write!(f, "verifying"),
            Self::Complete => write!(f, "complete"),
            Self::Failed => write!(f, "failed"),
            Self::RollingBack => write!(f, "rolling_back"),
        }
    }
}

/// A completed transition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedTransition {
    pub id: TransitionId,
    pub transition_type: TransitionType,
    pub started: i64,
    pub completed: i64,
    pub success: bool,
    pub summary: String,
}

/// Rules governing transitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRule {
    pub name: String,
    pub condition: String,
    pub action: String,
    pub priority: u32,
}
