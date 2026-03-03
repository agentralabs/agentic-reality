//! Resource write operations.

use crate::engine::write::WriteEngine;
use crate::types::error::{RealityError, RealityResult};
use crate::types::resource::*;

impl<'a> WriteEngine<'a> {
    /// Sense resources — set the full resource body.
    pub fn sense_resources(&mut self, body: ResourceBody) -> RealityResult<()> {
        self.engine.resource_store.body = Some(body);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update mind (memory) capacity.
    pub fn update_mind(&mut self, mind: MindCapacity) -> RealityResult<()> {
        let body = self.engine.resource_store.body.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("resource body".into())
        })?;
        body.mind = mind;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update processing energy (CPU).
    pub fn update_energy(&mut self, energy: ProcessingEnergy) -> RealityResult<()> {
        let body = self.engine.resource_store.body.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("resource body".into())
        })?;
        body.energy = energy;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update network reach.
    pub fn update_reach(&mut self, reach: NetworkReach) -> RealityResult<()> {
        let body = self.engine.resource_store.body.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("resource body".into())
        })?;
        body.reach = reach;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update storage capacity.
    pub fn update_storage(&mut self, storage: StorageCapacity) -> RealityResult<()> {
        let body = self.engine.resource_store.body.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("resource body".into())
        })?;
        body.storage = storage;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update GPU capacity.
    pub fn update_visual(&mut self, visual: Option<GpuCapacity>) -> RealityResult<()> {
        let body = self.engine.resource_store.body.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("resource body".into())
        })?;
        body.visual = visual;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add a resource sensation.
    pub fn add_sensation(&mut self, sensation: ResourceSensation) -> RealityResult<()> {
        let body = self.engine.resource_store.body.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("resource body".into())
        })?;
        body.sensations.push(sensation);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Clear sensations for a resource type.
    pub fn clear_sensation(&mut self, resource: ResourceType) -> RealityResult<()> {
        let body = self.engine.resource_store.body.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("resource body".into())
        })?;
        body.sensations.retain(|s| s.resource != resource);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update the resource pressure gradient.
    pub fn update_pressure_gradient(&mut self, gradient: ResourcePressureGradient) -> RealityResult<()> {
        self.engine.resource_store.pressure_gradient = Some(gradient);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Discover a new capability.
    pub fn discover_capability(&mut self, capability: Capability) -> RealityResult<()> {
        let map = self.engine.resource_store.capabilities.get_or_insert_with(|| {
            CapabilityMap {
                capabilities: vec![],
                discovered_at: crate::types::now_micros() as i64,
                stale_after_secs: 3600,
            }
        });
        map.capabilities.push(capability);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Lose a capability by name.
    pub fn lose_capability(&mut self, name: &str) -> RealityResult<()> {
        if let Some(map) = &mut self.engine.resource_store.capabilities {
            map.capabilities.retain(|c| c.name != name);
            self.engine.mark_dirty();
        }
        Ok(())
    }

    /// Update cost consciousness.
    pub fn update_cost(&mut self, cost: CostConsciousness) -> RealityResult<()> {
        self.engine.resource_store.cost = Some(cost);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update capacity planning intuition.
    pub fn update_capacity_intuition(&mut self, intuition: CapacityIntuition) -> RealityResult<()> {
        self.engine.resource_store.capacity_intuition = Some(intuition);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Set budget constraints.
    pub fn set_budget(&mut self, budget: BudgetConstraints) -> RealityResult<()> {
        let cost = self.engine.resource_store.cost.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("cost consciousness".into())
        })?;
        cost.budget = Some(budget);
        self.engine.mark_dirty();
        Ok(())
    }
}
