//! Topology write operations.

use crate::engine::write::WriteEngine;
use crate::types::error::{RealityError, RealityResult};
use crate::types::ids::{DependencyId, NeighborId, ObserverId, ServiceId};
use crate::types::topology::*;

impl<'a> WriteEngine<'a> {
    /// Set the agent's position in the topology.
    pub fn set_position(&mut self, position: TopologyPosition) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.get_or_insert_with(|| {
            DeploymentTopologyMap {
                self_position: position.clone(),
                upstream: vec![], downstream: vec![], siblings: vec![],
                dependents: vec![], observers: vec![], full_graph: None,
                topology_health: TopologyHealth { score: 1.0, weak_links: vec![], single_points_of_failure: vec![], redundancy_score: 1.0 },
            }
        });
        topo.self_position = position;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add an upstream entity.
    pub fn add_upstream(&mut self, upstream: UpstreamEntity) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        topo.upstream.push(upstream);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Remove an upstream entity by service ID.
    pub fn remove_upstream(&mut self, id: &ServiceId) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        topo.upstream.retain(|u| u.id != *id);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add a downstream dependency.
    pub fn add_downstream(&mut self, downstream: DownstreamEntity) -> RealityResult<DependencyId> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        let id = downstream.id;
        topo.downstream.push(downstream);
        self.engine.mark_dirty();
        Ok(id)
    }

    /// Remove a downstream dependency.
    pub fn remove_downstream(&mut self, id: &DependencyId) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        topo.downstream.retain(|d| d.id != *id);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update the health of a downstream dependency.
    pub fn update_downstream_health(&mut self, id: &DependencyId, health: HealthStatus) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        let dep = topo.downstream.iter_mut().find(|d| d.id == *id)
            .ok_or_else(|| RealityError::NotFound(format!("downstream {}", id)))?;
        dep.health = health;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add a sibling entity.
    pub fn add_sibling(&mut self, sibling: SiblingEntity) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        topo.siblings.push(sibling);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Remove a sibling entity.
    pub fn remove_sibling(&mut self, id: &NeighborId) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        topo.siblings.retain(|s| s.neighbor_id != *id);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update sibling state.
    pub fn update_sibling_state(&mut self, id: &NeighborId, health: HealthStatus, load: LoadLevel) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        let sib = topo.siblings.iter_mut().find(|s| s.neighbor_id == *id)
            .ok_or_else(|| RealityError::NotFound(format!("sibling {}", id)))?;
        sib.health = health;
        sib.load = load;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add an observer.
    pub fn add_observer(&mut self, observer: ObserverEntity) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        topo.observers.push(observer);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Remove an observer.
    pub fn remove_observer(&mut self, id: &ObserverId) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        topo.observers.retain(|o| o.id != *id);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update topology health.
    pub fn update_topology_health(&mut self, health: TopologyHealth) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        topo.topology_health = health;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Record a mesh event.
    pub fn record_mesh_event(&mut self, _event: String) -> RealityResult<()> {
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update the full topology graph.
    pub fn update_graph(&mut self, graph: TopologyGraph) -> RealityResult<()> {
        let topo = self.engine.topology_store.topology.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("topology".into())
        })?;
        topo.full_graph = Some(graph);
        self.engine.mark_dirty();
        Ok(())
    }
}
