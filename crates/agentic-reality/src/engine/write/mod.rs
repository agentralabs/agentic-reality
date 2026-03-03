//! Write engine — all mutation operations (~90 total).

pub mod coherence;
pub mod deployment;
pub mod environment;
pub mod reality;
pub mod resource;
pub mod stakes;
pub mod temporal;
pub mod topology;

use crate::engine::RealityEngine;

/// Write engine providing mutation operations across all domains.
pub struct WriteEngine<'a> {
    engine: &'a mut RealityEngine,
}

impl<'a> WriteEngine<'a> {
    pub fn new(engine: &'a mut RealityEngine) -> Self {
        Self { engine }
    }
}
