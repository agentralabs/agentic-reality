//! Stakes query operations.

use crate::engine::query::QueryEngine;
use crate::types::error::{RealityError, RealityResult};
use crate::types::stakes::*;

impl<'a> QueryEngine<'a> {
    pub fn get_stakes_level(&self) -> RealityResult<&StakesLevel> {
        let awareness = self.engine.stakes_store.consequences.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("consequence awareness".into()))?;
        Ok(&awareness.stakes)
    }

    pub fn get_consequences(&self) -> RealityResult<&[Consequence]> {
        let awareness = self.engine.stakes_store.consequences.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("consequence awareness".into()))?;
        Ok(&awareness.consequence_map)
    }

    pub fn get_irreversible_actions(&self) -> RealityResult<&[IrreversibleAction]> {
        let awareness = self.engine.stakes_store.consequences.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("consequence awareness".into()))?;
        Ok(&awareness.irreversible)
    }

    pub fn is_irreversible(&self, action: &str) -> bool {
        self.engine.stakes_store.consequences.as_ref()
            .map(|a| a.irreversible.iter().any(|i| i.action == action))
            .unwrap_or(false)
    }

    pub fn get_safety_margins(&self) -> RealityResult<&SafetyMargins> {
        let awareness = self.engine.stakes_store.consequences.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("consequence awareness".into()))?;
        Ok(&awareness.safety_margins)
    }

    pub fn get_guardrails(&self) -> RealityResult<&[Guardrail]> {
        Ok(&self.get_safety_margins()?.guardrails)
    }

    pub fn get_risk_field(&self) -> RealityResult<&RiskFieldPerception> {
        self.engine.stakes_store.risk_field.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("risk field".into()))
    }

    pub fn get_risk_for(&self, category: &RiskCategory) -> Option<&RiskLevel> {
        self.engine.stakes_store.risk_field.as_ref()
            .and_then(|f| f.risk_map.get(category))
    }

    pub fn get_blast_radius(&self) -> RealityResult<&BlastRadiusAwareness> {
        self.engine.stakes_store.blast_radius.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("blast radius".into()))
    }

    pub fn get_cascade_analysis(&self) -> RealityResult<&CascadeAnalysis> {
        Ok(&self.get_blast_radius()?.cascade)
    }
}
