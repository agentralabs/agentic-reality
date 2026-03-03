//! Temporal query operations.

use crate::engine::query::QueryEngine;
use crate::types::error::{RealityError, RealityResult};
use crate::types::ids::EventId;
use crate::types::temporal::*;

impl<'a> QueryEngine<'a> {
    pub fn get_temporal_context(&self) -> RealityResult<&TemporalContext> {
        let awareness = self.engine.temporal_store.awareness.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("temporal awareness".into()))?;
        Ok(&awareness.context)
    }

    pub fn get_grounded_time(&self) -> RealityResult<&TemporalAwareness> {
        self.engine.temporal_store.awareness.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("temporal awareness".into()))
    }

    pub fn get_causality_graph(&self) -> RealityResult<&CausalityGraph> {
        self.engine.temporal_store.causality.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("causality graph".into()))
    }

    pub fn get_causal_chain(&self, event_id: &EventId) -> RealityResult<Vec<&CausalEvent>> {
        let graph = self.get_causality_graph()?;
        let mut chain = vec![];
        let mut stack = vec![*event_id];
        while let Some(id) = stack.pop() {
            if let Some(evt) = graph.events.iter().find(|e| e.id == id) {
                chain.push(evt);
                for cause in &evt.causes {
                    if !chain.iter().any(|e| e.id == *cause) {
                        stack.push(*cause);
                    }
                }
            }
        }
        Ok(chain)
    }

    pub fn get_root_causes(&self) -> RealityResult<&[EventId]> {
        Ok(&self.get_causality_graph()?.root_causes)
    }

    pub fn get_timelines(&self) -> &[Timeline] {
        &self.engine.temporal_store.timelines
    }

    pub fn get_unified_timeline(&self) -> UnifiedTimeline {
        let timelines = &self.engine.temporal_store.timelines;
        let mut merged = vec![];
        let mut conflicts = vec![];
        for t in timelines {
            merged.extend(t.events.clone());
            conflicts.extend(t.conflicts.iter().filter(|c| !c.resolved).cloned());
        }
        merged.sort_by_key(|e| e.timestamp);
        UnifiedTimeline {
            timelines: timelines.iter().map(|t| t.id).collect(),
            merged_events: merged,
            unresolved_conflicts: conflicts,
            coherence_score: if timelines.is_empty() { 1.0 } else {
                timelines.iter().filter(|t| t.coherent).count() as f64 / timelines.len() as f64
            },
        }
    }

    pub fn get_deadlines(&self) -> RealityResult<&[Deadline]> {
        Ok(&self.get_temporal_context()?.deadlines)
    }
}
