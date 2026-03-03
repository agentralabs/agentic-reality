//! Coherence write operations.

use crate::engine::write::WriteEngine;
use crate::types::coherence::*;
use crate::types::error::{RealityError, RealityResult};
use crate::types::ids::{ContextId, TransitionId};

impl<'a> WriteEngine<'a> {
    /// Run a coherence check and record the result.
    pub fn run_coherence_check(&mut self, check: CoherenceCheck) -> RealityResult<()> {
        let state = self.engine.coherence_store.state.get_or_insert_with(|| {
            CoherenceState {
                level: CoherenceLevel::Full { confidence: 1.0 },
                checks: vec![], violations: vec![],
                strategies: vec![], history: vec![],
            }
        });
        state.checks.push(check);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Record a coherence violation.
    pub fn record_violation(&mut self, violation: CoherenceViolation) -> RealityResult<()> {
        let state = self.engine.coherence_store.state.get_or_insert_with(|| {
            CoherenceState {
                level: CoherenceLevel::Full { confidence: 1.0 },
                checks: vec![], violations: vec![],
                strategies: vec![], history: vec![],
            }
        });
        state.violations.push(violation);
        // Update coherence level based on violation count
        let unresolved = state.violations.iter().filter(|v| v.resolution.is_none()).count();
        if unresolved > 2 {
            state.level = CoherenceLevel::Significant {
                issues: state.violations.iter().filter(|v| v.resolution.is_none()).map(|v| v.description.clone()).collect(),
                impact: "multiple unresolved violations".into(),
            };
        } else if unresolved > 0 {
            state.level = CoherenceLevel::Minor {
                issues: state.violations.iter().filter(|v| v.resolution.is_none()).map(|v| v.description.clone()).collect(),
            };
        }
        self.engine.mark_dirty();
        Ok(())
    }

    /// Resolve a coherence violation.
    pub fn resolve_violation(&mut self, idx: usize, resolution: String) -> RealityResult<()> {
        let state = self.engine.coherence_store.state.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("coherence state".into())
        })?;
        let violation = state.violations.get_mut(idx)
            .ok_or_else(|| RealityError::NotFound(format!("violation index {}", idx)))?;
        violation.resolution = Some(resolution);
        // Recalculate coherence level
        let unresolved = state.violations.iter().filter(|v| v.resolution.is_none()).count();
        if unresolved == 0 {
            state.level = CoherenceLevel::Full { confidence: 0.9 };
        }
        self.engine.mark_dirty();
        Ok(())
    }

    /// Begin a context transition.
    pub fn begin_transition(&mut self, transition: PendingTransition) -> RealityResult<TransitionId> {
        let id = transition.id;
        let trans = self.engine.coherence_store.transitions.get_or_insert_with(|| {
            TransitionState {
                current: ContextId::new(),
                pending: vec![],
                history: vec![],
                rules: vec![],
            }
        });
        trans.pending.push(transition);
        self.engine.mark_dirty();
        Ok(id)
    }

    /// Advance a transition to the next phase.
    pub fn advance_transition(&mut self, id: &TransitionId, phase: TransitionPhase) -> RealityResult<()> {
        let trans = self.engine.coherence_store.transitions.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("transition state".into())
        })?;
        let pending = trans.pending.iter_mut().find(|t| t.id == *id)
            .ok_or_else(|| RealityError::NotFound(format!("transition {}", id)))?;
        pending.phase = phase;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Complete a transition.
    pub fn complete_transition(&mut self, id: &TransitionId) -> RealityResult<()> {
        let trans = self.engine.coherence_store.transitions.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("transition state".into())
        })?;
        let idx = trans.pending.iter().position(|t| t.id == *id)
            .ok_or_else(|| RealityError::NotFound(format!("transition {}", id)))?;
        let pending = trans.pending.remove(idx);
        trans.history.push(CompletedTransition {
            id: pending.id,
            transition_type: pending.transition_type,
            started: pending.started,
            completed: crate::types::now_micros() as i64,
            success: true,
            summary: format!("Completed {} transition", pending.transition_type),
        });
        self.engine.mark_dirty();
        Ok(())
    }

    /// Abort a transition.
    pub fn abort_transition(&mut self, id: &TransitionId) -> RealityResult<()> {
        let trans = self.engine.coherence_store.transitions.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("transition state".into())
        })?;
        let idx = trans.pending.iter().position(|t| t.id == *id)
            .ok_or_else(|| RealityError::NotFound(format!("transition {}", id)))?;
        let pending = trans.pending.remove(idx);
        trans.history.push(CompletedTransition {
            id: pending.id,
            transition_type: pending.transition_type,
            started: pending.started,
            completed: crate::types::now_micros() as i64,
            success: false,
            summary: format!("Aborted {} transition", pending.transition_type),
        });
        self.engine.mark_dirty();
        Ok(())
    }

    /// Rollback a transition.
    pub fn rollback_transition(&mut self, id: &TransitionId) -> RealityResult<()> {
        self.advance_transition(id, TransitionPhase::RollingBack)?;
        self.abort_transition(id)
    }
}
