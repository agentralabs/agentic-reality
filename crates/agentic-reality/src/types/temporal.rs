//! Temporal grounding types — "When am I?"

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ids::{EventId, TimelineId};

/// Temporal awareness — the agent's sense of time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalAwareness {
    pub wall_clock: i64,
    pub monotonic_ns: u64,
    pub time_source: TimeSource,
    pub time_confidence: f64,
    pub drift: Option<TimeDrift>,
    pub context: TemporalContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeSource {
    System,
    Ntp { server: String, offset_ms: f64 },
    Gps,
    Consensus { participants: u32 },
    External { source: String },
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeDrift {
    pub offset_ms: f64,
    pub direction: DriftDirection,
    pub source: String,
    pub detected_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriftDirection {
    Ahead,
    Behind,
    Oscillating,
}

/// Temporal context — where in various timelines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    pub session_start: i64,
    pub session_duration_secs: u64,
    pub request_count: u64,
    pub deadlines: Vec<Deadline>,
    pub time_budget: Option<TimeBudget>,
    pub causal_position: Option<CausalPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deadline {
    pub id: String,
    pub name: String,
    pub due_at: i64,
    pub priority: DeadlinePriority,
    pub remaining_secs: i64,
    pub met: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeadlinePriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBudget {
    pub total_secs: u64,
    pub used_secs: u64,
    pub remaining_secs: u64,
    pub extensions_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalPosition {
    pub depth: u32,
    pub breadth: u32,
    pub root_event: Option<EventId>,
}

/// Causal event in the causality graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEvent {
    pub id: EventId,
    pub event_type: CausalEventType,
    pub description: String,
    pub timestamp: i64,
    pub causes: Vec<EventId>,
    pub effects: Vec<EventId>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalEventType {
    Action,
    Observation,
    Decision,
    ExternalEvent,
    SystemEvent,
    Error,
}

/// Causality graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalityGraph {
    pub events: Vec<CausalEvent>,
    pub root_causes: Vec<EventId>,
    pub leaf_effects: Vec<EventId>,
}

/// Timeline for tracking coherence across multiple timelines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub id: TimelineId,
    pub name: String,
    pub events: Vec<TimelineEvent>,
    pub conflicts: Vec<TimelineConflict>,
    pub coherent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub event_id: EventId,
    pub timestamp: i64,
    pub description: String,
    pub timeline_id: TimelineId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineConflict {
    pub timelines: Vec<TimelineId>,
    pub description: String,
    pub severity: ConflictSeverity,
    pub resolved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictSeverity {
    Minor,
    Moderate,
    Major,
    Critical,
}

/// Unified timeline merging multiple timelines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedTimeline {
    pub timelines: Vec<TimelineId>,
    pub merged_events: Vec<TimelineEvent>,
    pub unresolved_conflicts: Vec<TimelineConflict>,
    pub coherence_score: f64,
}
