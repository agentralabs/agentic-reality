//! Indexes for fast lookups across reality domains.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::ids::*;

/// All indexes for reality data.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RealityIndexes {
    pub anchor_index: HashMap<AnchorId, usize>,
    pub timeline_index: HashMap<TimelineId, usize>,
    pub event_index: HashMap<EventId, usize>,
    pub service_index: HashMap<String, usize>,
    pub dependency_index: HashMap<DependencyId, usize>,
}

impl RealityIndexes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rebuild(&mut self) {
        // Indexes are rebuilt when loading from file
    }
}
