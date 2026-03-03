//! Topology query operations.

use crate::engine::query::QueryEngine;
use crate::types::error::{RealityError, RealityResult};
use crate::types::ids::DependencyId;
use crate::types::topology::*;

impl<'a> QueryEngine<'a> {
    pub fn get_topology_map(&self) -> RealityResult<&DeploymentTopologyMap> {
        self.engine
            .topology_store
            .topology
            .as_ref()
            .ok_or_else(|| RealityError::NotInitialized("topology".into()))
    }

    pub fn get_position(&self) -> RealityResult<&TopologyPosition> {
        Ok(&self.get_topology_map()?.self_position)
    }

    pub fn get_upstream(&self) -> RealityResult<&[UpstreamEntity]> {
        Ok(&self.get_topology_map()?.upstream)
    }

    pub fn get_downstream(&self) -> RealityResult<&[DownstreamEntity]> {
        Ok(&self.get_topology_map()?.downstream)
    }

    pub fn get_dependency(&self, id: &DependencyId) -> RealityResult<&DownstreamEntity> {
        let topo = self.get_topology_map()?;
        topo.downstream
            .iter()
            .find(|d| d.id == *id)
            .ok_or_else(|| RealityError::NotFound(format!("dependency {}", id)))
    }

    pub fn get_siblings(&self) -> RealityResult<&[SiblingEntity]> {
        Ok(&self.get_topology_map()?.siblings)
    }

    pub fn get_observers(&self) -> RealityResult<&[ObserverEntity]> {
        Ok(&self.get_topology_map()?.observers)
    }

    pub fn get_topology_health(&self) -> RealityResult<&TopologyHealth> {
        Ok(&self.get_topology_map()?.topology_health)
    }

    pub fn get_weak_links(&self) -> RealityResult<&[String]> {
        Ok(&self.get_topology_map()?.topology_health.weak_links)
    }

    pub fn get_single_points(&self) -> RealityResult<&[String]> {
        Ok(&self
            .get_topology_map()?
            .topology_health
            .single_points_of_failure)
    }

    pub fn find_path(&self, _from: &str, _to: &str) -> RealityResult<Vec<String>> {
        Ok(vec![])
    }

    pub fn get_neighbor_awareness(&self) -> RealityResult<&NeighborAwareness> {
        self.engine
            .topology_store
            .neighbors
            .as_ref()
            .ok_or_else(|| RealityError::NotInitialized("neighbor awareness".into()))
    }
}
