//! Query engine — all read operations (~78 total).

pub mod coherence;
pub mod deployment;
pub mod environment;
pub mod reality;
pub mod resource;
pub mod stakes;
pub mod temporal;
pub mod topology;

use crate::engine::RealityEngine;

/// Query engine providing read-only access across all domains.
pub struct QueryEngine<'a> {
    engine: &'a RealityEngine,
}

impl<'a> QueryEngine<'a> {
    pub fn new(engine: &'a RealityEngine) -> Self {
        Self { engine }
    }
}
