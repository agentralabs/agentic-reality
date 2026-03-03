//! Deployment write operations.

use crate::engine::write::WriteEngine;
use crate::types::deployment::*;
use crate::types::error::RealityResult;
use crate::types::ids::IncarnationId;

impl<'a> WriteEngine<'a> {
    /// Initialize the deployment soul — the agent's existential identity.
    pub fn initialize_soul(&mut self, soul: DeploymentSoul) -> RealityResult<IncarnationId> {
        if self.engine.deployment_store.soul.is_some() {
            return Err(crate::types::error::RealityError::AlreadyInitialized(
                "deployment soul".into(),
            ));
        }
        let id = soul.incarnation_id;
        self.engine.deployment_store.soul = Some(soul);
        self.engine.mark_dirty();
        Ok(id)
    }

    /// Update the physical substrate.
    pub fn update_substrate(&mut self, substrate: PhysicalSubstrate) -> RealityResult<()> {
        let soul = self.engine.deployment_store.soul.as_mut().ok_or_else(|| {
            crate::types::error::RealityError::NotInitialized("deployment soul".into())
        })?;
        soul.substrate = substrate;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Record birth context.
    pub fn record_birth(&mut self, birth: BirthContext) -> RealityResult<()> {
        let soul = self.engine.deployment_store.soul.as_mut().ok_or_else(|| {
            crate::types::error::RealityError::NotInitialized("deployment soul".into())
        })?;
        soul.birth = birth;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update soul vitals.
    pub fn update_vitals(&mut self, vitals: SoulVitals) -> RealityResult<()> {
        let soul = self.engine.deployment_store.soul.as_mut().ok_or_else(|| {
            crate::types::error::RealityError::NotInitialized("deployment soul".into())
        })?;
        soul.vitals = vitals;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Record death of the current incarnation.
    pub fn record_death(&mut self, death: DeathRecord) -> RealityResult<()> {
        let mem = self
            .engine
            .deployment_store
            .incarnation_memory
            .as_mut()
            .ok_or_else(|| {
                crate::types::error::RealityError::NotInitialized("incarnation memory".into())
            })?;
        if let Some(soul) = &self.engine.deployment_store.soul {
            let past = PastIncarnation {
                incarnation_id: soul.incarnation_id,
                birth: soul.birth.spawned_at,
                death,
                lessons: vec![],
                substrate_tier: format!("{:?}", soul.substrate.tier),
            };
            mem.past_lives.push(past);
            mem.total_incarnations += 1;
        }
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add a past incarnation.
    pub fn add_past_life(&mut self, past: PastIncarnation) -> RealityResult<()> {
        let mem = self.engine.deployment_store.incarnation_memory.get_or_insert_with(|| {
            IncarnationMemory {
                current: self.engine.deployment_store.soul.as_ref().map(|s| s.incarnation_id).unwrap_or_else(IncarnationId::new),
                past_lives: vec![],
                total_incarnations: 0,
                total_uptime_secs: 0,
                wisdom: vec![],
                karma: IncarnationKarma { score: 0.0, good_deeds: 0, incidents: 0, trend: KarmaTrend::Stable },
            }
        });
        mem.past_lives.push(past);
        mem.total_incarnations += 1;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update deployment lineage.
    pub fn update_lineage(&mut self, lineage: DeploymentLineage) -> RealityResult<()> {
        let soul = self.engine.deployment_store.soul.as_mut().ok_or_else(|| {
            crate::types::error::RealityError::NotInitialized("deployment soul".into())
        })?;
        soul.lineage = lineage;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Set the deployment role.
    pub fn set_role(&mut self, role: DeploymentRole) -> RealityResult<()> {
        let soul = self.engine.deployment_store.soul.as_mut().ok_or_else(|| {
            crate::types::error::RealityError::NotInitialized("deployment soul".into())
        })?;
        soul.role = role;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Set the existential nature.
    pub fn set_nature(&mut self, nature: ExistentialNature) -> RealityResult<()> {
        let soul = self.engine.deployment_store.soul.as_mut().ok_or_else(|| {
            crate::types::error::RealityError::NotInitialized("deployment soul".into())
        })?;
        soul.nature = nature;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update cardinality.
    pub fn update_cardinality(&mut self, cardinality: Cardinality) -> RealityResult<()> {
        let soul = self.engine.deployment_store.soul.as_mut().ok_or_else(|| {
            crate::types::error::RealityError::NotInitialized("deployment soul".into())
        })?;
        soul.nature.cardinality = cardinality;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Record a piece of wisdom from a past incarnation.
    pub fn record_wisdom(&mut self, wisdom: IncarnationWisdom) -> RealityResult<()> {
        let soul = self.engine.deployment_store.soul.as_mut().ok_or_else(|| {
            crate::types::error::RealityError::NotInitialized("deployment soul".into())
        })?;
        soul.lineage.wisdom.push(wisdom);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update incarnation karma.
    pub fn update_karma(&mut self, karma: IncarnationKarma) -> RealityResult<()> {
        let soul = self.engine.deployment_store.soul.as_mut().ok_or_else(|| {
            crate::types::error::RealityError::NotInitialized("deployment soul".into())
        })?;
        soul.lineage.karma = karma;
        self.engine.mark_dirty();
        Ok(())
    }
}
