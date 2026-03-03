//! Temporal write operations.

use crate::engine::write::WriteEngine;
use crate::types::error::{RealityError, RealityResult};
use crate::types::ids::{EventId, TimelineId};
use crate::types::temporal::*;

impl<'a> WriteEngine<'a> {
    /// Ground the agent's time awareness.
    pub fn ground_time(&mut self, awareness: TemporalAwareness) -> RealityResult<()> {
        self.engine.temporal_store.awareness = Some(awareness);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update temporal context.
    pub fn update_temporal_context(&mut self, context: TemporalContext) -> RealityResult<()> {
        let awareness = self
            .engine
            .temporal_store
            .awareness
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("temporal awareness".into()))?;
        awareness.context = context;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add a causal event.
    pub fn add_causal_event(&mut self, event: CausalEvent) -> RealityResult<EventId> {
        let id = event.id;
        let graph = self
            .engine
            .temporal_store
            .causality
            .get_or_insert_with(|| CausalityGraph {
                events: vec![],
                root_causes: vec![],
                leaf_effects: vec![],
            });
        graph.events.push(event);
        self.engine.mark_dirty();
        Ok(id)
    }

    /// Link two events causally.
    pub fn link_causality(&mut self, cause: &EventId, effect: &EventId) -> RealityResult<()> {
        let graph = self
            .engine
            .temporal_store
            .causality
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("causality graph".into()))?;
        if let Some(evt) = graph.events.iter_mut().find(|e| e.id == *cause) {
            if !evt.effects.contains(effect) {
                evt.effects.push(*effect);
            }
        }
        if let Some(evt) = graph.events.iter_mut().find(|e| e.id == *effect) {
            if !evt.causes.contains(cause) {
                evt.causes.push(*cause);
            }
        }
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add a deadline.
    pub fn add_deadline(&mut self, deadline: Deadline) -> RealityResult<()> {
        let awareness = self
            .engine
            .temporal_store
            .awareness
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("temporal awareness".into()))?;
        awareness.context.deadlines.push(deadline);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Remove a deadline by ID.
    pub fn remove_deadline(&mut self, id: &str) -> RealityResult<()> {
        let awareness = self
            .engine
            .temporal_store
            .awareness
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("temporal awareness".into()))?;
        awareness.context.deadlines.retain(|d| d.id != id);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update a deadline.
    pub fn update_deadline(&mut self, deadline: Deadline) -> RealityResult<()> {
        let awareness = self
            .engine
            .temporal_store
            .awareness
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("temporal awareness".into()))?;
        if let Some(d) = awareness
            .context
            .deadlines
            .iter_mut()
            .find(|d| d.id == deadline.id)
        {
            *d = deadline;
        }
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add a timeline.
    pub fn add_timeline(&mut self, timeline: Timeline) -> RealityResult<TimelineId> {
        let id = timeline.id;
        self.engine.temporal_store.timelines.push(timeline);
        self.engine.mark_dirty();
        Ok(id)
    }

    /// Record an event on a timeline.
    pub fn record_timeline_event(
        &mut self,
        timeline_id: &TimelineId,
        event: TimelineEvent,
    ) -> RealityResult<()> {
        let timeline = self
            .engine
            .temporal_store
            .timelines
            .iter_mut()
            .find(|t| t.id == *timeline_id)
            .ok_or_else(|| RealityError::NotFound(format!("timeline {}", timeline_id)))?;
        timeline.events.push(event);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Resolve a timeline conflict.
    pub fn resolve_timeline_conflict(
        &mut self,
        timeline_id: &TimelineId,
        conflict_idx: usize,
    ) -> RealityResult<()> {
        let timeline = self
            .engine
            .temporal_store
            .timelines
            .iter_mut()
            .find(|t| t.id == *timeline_id)
            .ok_or_else(|| RealityError::NotFound(format!("timeline {}", timeline_id)))?;
        if let Some(conflict) = timeline.conflicts.get_mut(conflict_idx) {
            conflict.resolved = true;
        }
        self.engine.mark_dirty();
        Ok(())
    }
}
