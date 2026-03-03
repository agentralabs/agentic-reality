//! Stakes write operations.

use crate::engine::write::WriteEngine;
use crate::types::error::{RealityError, RealityResult};
use crate::types::stakes::*;

impl<'a> WriteEngine<'a> {
    /// Set the stakes level.
    pub fn set_stakes_level(&mut self, stakes: StakesLevel) -> RealityResult<()> {
        let awareness = self.engine.stakes_store.consequences.get_or_insert_with(|| {
            ConsequenceAwareness {
                stakes: StakesLevel::Minimal { can_experiment: true },
                consequence_map: vec![], irreversible: vec![],
                safety_margins: SafetyMargins { overall: 1.0, by_domain: Default::default(), guardrails: vec![] },
                history: vec![],
            }
        });
        awareness.stakes = stakes;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add a potential consequence.
    pub fn add_consequence(&mut self, consequence: Consequence) -> RealityResult<()> {
        let awareness = self.engine.stakes_store.consequences.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("consequence awareness".into())
        })?;
        awareness.consequence_map.push(consequence);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Remove a consequence by effect description.
    pub fn remove_consequence(&mut self, effect: &str) -> RealityResult<()> {
        let awareness = self.engine.stakes_store.consequences.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("consequence awareness".into())
        })?;
        awareness.consequence_map.retain(|c| c.effect != effect);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add an irreversible action.
    pub fn add_irreversible_action(&mut self, action: IrreversibleAction) -> RealityResult<()> {
        let awareness = self.engine.stakes_store.consequences.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("consequence awareness".into())
        })?;
        awareness.irreversible.push(action);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update safety margins.
    pub fn update_safety_margins(&mut self, margins: SafetyMargins) -> RealityResult<()> {
        let awareness = self.engine.stakes_store.consequences.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("consequence awareness".into())
        })?;
        awareness.safety_margins = margins;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add a guardrail.
    pub fn add_guardrail(&mut self, guardrail: Guardrail) -> RealityResult<()> {
        let awareness = self.engine.stakes_store.consequences.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("consequence awareness".into())
        })?;
        awareness.safety_margins.guardrails.push(guardrail);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Remove a guardrail by name.
    pub fn remove_guardrail(&mut self, name: &str) -> RealityResult<()> {
        let awareness = self.engine.stakes_store.consequences.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("consequence awareness".into())
        })?;
        awareness.safety_margins.guardrails.retain(|g| g.name != name);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update the risk field.
    pub fn update_risk_field(&mut self, risk_field: RiskFieldPerception) -> RealityResult<()> {
        self.engine.stakes_store.risk_field = Some(risk_field);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update blast radius awareness.
    pub fn update_blast_radius(&mut self, blast: BlastRadiusAwareness) -> RealityResult<()> {
        self.engine.stakes_store.blast_radius = Some(blast);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Record a consequence that occurred.
    pub fn record_consequence(&mut self, record: ConsequenceRecord) -> RealityResult<()> {
        let awareness = self.engine.stakes_store.consequences.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("consequence awareness".into())
        })?;
        awareness.history.push(record);
        self.engine.mark_dirty();
        Ok(())
    }
}
