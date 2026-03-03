//! Resource query operations.

use crate::engine::query::QueryEngine;
use crate::types::error::{RealityError, RealityResult};
use crate::types::resource::*;

impl<'a> QueryEngine<'a> {
    pub fn get_body(&self) -> RealityResult<&ResourceBody> {
        self.engine.resource_store.body.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("resource body".into()))
    }

    pub fn get_mind(&self) -> RealityResult<&MindCapacity> {
        Ok(&self.get_body()?.mind)
    }

    pub fn get_energy(&self) -> RealityResult<&ProcessingEnergy> {
        Ok(&self.get_body()?.energy)
    }

    pub fn get_reach(&self) -> RealityResult<&NetworkReach> {
        Ok(&self.get_body()?.reach)
    }

    pub fn get_storage(&self) -> RealityResult<&StorageCapacity> {
        Ok(&self.get_body()?.storage)
    }

    pub fn get_visual(&self) -> RealityResult<Option<&GpuCapacity>> {
        Ok(self.get_body()?.visual.as_ref())
    }

    pub fn get_sensations(&self) -> RealityResult<&[ResourceSensation]> {
        Ok(&self.get_body()?.sensations)
    }

    pub fn get_pressure_gradient(&self) -> RealityResult<&ResourcePressureGradient> {
        self.engine.resource_store.pressure_gradient.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("pressure gradient".into()))
    }

    pub fn get_bottleneck(&self) -> Option<ResourceType> {
        self.engine.resource_store.pressure_gradient.as_ref()
            .and_then(|g| g.bottleneck)
    }

    pub fn get_capabilities(&self) -> RealityResult<&CapabilityMap> {
        self.engine.resource_store.capabilities.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("capabilities".into()))
    }

    pub fn can_do(&self, capability: &str) -> bool {
        self.engine.resource_store.capabilities.as_ref()
            .map(|m| m.capabilities.iter().any(|c| c.name == capability && c.available))
            .unwrap_or(false)
    }

    pub fn get_cost(&self) -> RealityResult<&CostConsciousness> {
        self.engine.resource_store.cost.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("cost consciousness".into()))
    }
}
