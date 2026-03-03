//! Deployment query operations.

use crate::engine::query::QueryEngine;
use crate::engine::ContextSummary;
use crate::types::deployment::*;
use crate::types::error::{RealityError, RealityResult};
impl<'a> QueryEngine<'a> {
    pub fn get_soul(&self) -> RealityResult<&DeploymentSoul> {
        self.engine
            .deployment_store
            .soul
            .as_ref()
            .ok_or_else(|| RealityError::NotInitialized("deployment soul".into()))
    }

    pub fn get_birth_context(&self) -> RealityResult<&BirthContext> {
        Ok(&self.get_soul()?.birth)
    }

    pub fn get_substrate(&self) -> RealityResult<&PhysicalSubstrate> {
        Ok(&self.get_soul()?.substrate)
    }

    pub fn get_vitals(&self) -> RealityResult<&SoulVitals> {
        Ok(&self.get_soul()?.vitals)
    }

    pub fn get_lineage(&self) -> RealityResult<&DeploymentLineage> {
        Ok(&self.get_soul()?.lineage)
    }

    pub fn get_incarnation_memory(&self) -> RealityResult<&IncarnationMemory> {
        self.engine
            .deployment_store
            .incarnation_memory
            .as_ref()
            .ok_or_else(|| RealityError::NotInitialized("incarnation memory".into()))
    }

    pub fn get_wisdom(&self) -> RealityResult<&[IncarnationWisdom]> {
        Ok(&self.get_soul()?.lineage.wisdom)
    }

    pub fn get_karma(&self) -> RealityResult<&IncarnationKarma> {
        Ok(&self.get_soul()?.lineage.karma)
    }

    pub fn get_nature(&self) -> RealityResult<&ExistentialNature> {
        Ok(&self.get_soul()?.nature)
    }

    pub fn get_context_summary(&self) -> ContextSummary {
        let soul = self.engine.deployment_store.soul.as_ref();
        let env = self.engine.environment_store.medium.as_ref();
        let stakes = self.engine.stakes_store.consequences.as_ref();
        let coherence = self.engine.coherence_store.state.as_ref();
        let resource = self.engine.resource_store.pressure_gradient.as_ref();

        ContextSummary {
            incarnation_id: soul.map(|s| s.incarnation_id),
            substrate_tier: soul.map(|s| format!("{:?}", s.substrate.tier)),
            environment_type: env.map(|e| format!("{:?}", e.environment_type)),
            environment_mood: env.map(|e| e.current_state.mood.to_string()),
            resource_bottleneck: resource.and_then(|r| r.bottleneck.map(|b| b.to_string())),
            stakes_level: stakes.map(|s| s.stakes.to_string()),
            coherence_level: coherence.map(|c| c.level.to_string()),
            uptime_secs: soul.map(|s| s.vitals.uptime_secs),
        }
    }
}
