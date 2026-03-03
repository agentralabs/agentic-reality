//! Coherence query operations.

use crate::engine::query::QueryEngine;
use crate::types::coherence::*;
use crate::types::error::{RealityError, RealityResult};
use crate::types::ids::TransitionId;

impl<'a> QueryEngine<'a> {
    pub fn get_coherence_state(&self) -> RealityResult<&CoherenceState> {
        self.engine.coherence_store.state.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("coherence state".into()))
    }

    pub fn get_coherence_level(&self) -> RealityResult<&CoherenceLevel> {
        Ok(&self.get_coherence_state()?.level)
    }

    pub fn get_violations(&self) -> RealityResult<&[CoherenceViolation]> {
        Ok(&self.get_coherence_state()?.violations)
    }

    pub fn get_pending_transitions(&self) -> RealityResult<&[PendingTransition]> {
        let trans = self.engine.coherence_store.transitions.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("transition state".into()))?;
        Ok(&trans.pending)
    }

    pub fn get_transition(&self, id: &TransitionId) -> RealityResult<&PendingTransition> {
        let trans = self.engine.coherence_store.transitions.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("transition state".into()))?;
        trans.pending.iter().find(|t| t.id == *id)
            .ok_or_else(|| RealityError::NotFound(format!("transition {}", id)))
    }

    pub fn get_transition_history(&self) -> RealityResult<&[CompletedTransition]> {
        let trans = self.engine.coherence_store.transitions.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("transition state".into()))?;
        Ok(&trans.history)
    }

    pub fn is_coherent(&self) -> bool {
        self.engine.coherence_store.state.as_ref()
            .map(|s| matches!(s.level, CoherenceLevel::Full { .. }))
            .unwrap_or(true)
    }

    pub fn get_coherence_checks(&self) -> RealityResult<&[CoherenceCheck]> {
        Ok(&self.get_coherence_state()?.checks)
    }
}
