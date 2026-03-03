//! Environment query operations.

use crate::engine::query::QueryEngine;
use crate::types::environment::*;
use crate::types::error::{RealityError, RealityResult};

impl<'a> QueryEngine<'a> {
    pub fn get_environment(&self) -> RealityResult<&EnvironmentMedium> {
        self.engine.environment_store.medium.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("environment".into()))
    }

    pub fn get_environment_type(&self) -> RealityResult<&EnvironmentType> {
        Ok(&self.get_environment()?.environment_type)
    }

    pub fn get_environment_state(&self) -> RealityResult<&EnvironmentState> {
        Ok(&self.get_environment()?.current_state)
    }

    pub fn get_mood(&self) -> RealityResult<EnvironmentMood> {
        Ok(self.get_environment()?.current_state.mood)
    }

    pub fn get_physics(&self) -> RealityResult<&EnvironmentPhysics> {
        Ok(&self.get_environment()?.physics)
    }

    pub fn get_incidents(&self) -> RealityResult<&[ActiveIncident]> {
        Ok(&self.get_environment()?.current_state.incidents)
    }

    pub fn get_fingerprint(&self) -> RealityResult<&ContextFingerprint> {
        self.engine.environment_store.fingerprint.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("context fingerprint".into()))
    }

    pub fn has_context_shifted(&self) -> bool {
        self.engine.environment_store.fingerprint.as_ref()
            .map(|f| f.stability != ContextStability::Stable)
            .unwrap_or(false)
    }
}
