//! Environment write operations.

use crate::engine::write::WriteEngine;
use crate::types::environment::*;
use crate::types::error::{RealityError, RealityResult};

impl<'a> WriteEngine<'a> {
    /// Sense the environment — set the full environment medium.
    pub fn sense_environment(&mut self, medium: EnvironmentMedium) -> RealityResult<()> {
        self.engine.environment_store.medium = Some(medium);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update environment state.
    pub fn update_environment_state(&mut self, state: EnvironmentState) -> RealityResult<()> {
        let medium = self
            .engine
            .environment_store
            .medium
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("environment".into()))?;
        medium.current_state = state;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update environment mood.
    pub fn update_mood(&mut self, mood: EnvironmentMood) -> RealityResult<()> {
        let medium = self
            .engine
            .environment_store
            .medium
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("environment".into()))?;
        medium.current_state.mood = mood;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Record an active incident.
    pub fn record_incident(&mut self, incident: ActiveIncident) -> RealityResult<()> {
        let medium = self
            .engine
            .environment_store
            .medium
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("environment".into()))?;
        medium.current_state.incidents.push(incident);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Clear an incident by ID.
    pub fn clear_incident(&mut self, incident_id: &str) -> RealityResult<()> {
        let medium = self
            .engine
            .environment_store
            .medium
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("environment".into()))?;
        medium
            .current_state
            .incidents
            .retain(|i| i.id != incident_id);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update environment physics (constraints).
    pub fn update_physics(&mut self, physics: EnvironmentPhysics) -> RealityResult<()> {
        let medium = self
            .engine
            .environment_store
            .medium
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("environment".into()))?;
        medium.physics = physics;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Record a weather event.
    pub fn record_weather(&mut self, event: WeatherEvent) -> RealityResult<()> {
        let medium = self
            .engine
            .environment_store
            .medium
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("environment".into()))?;
        medium.weather_history.push(event);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update the context fingerprint.
    pub fn update_fingerprint(&mut self, fingerprint: ContextFingerprint) -> RealityResult<()> {
        self.engine.environment_store.fingerprint = Some(fingerprint);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add an inhabitant.
    pub fn add_inhabitant(&mut self, inhabitant: String) -> RealityResult<()> {
        let medium = self
            .engine
            .environment_store
            .medium
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("environment".into()))?;
        if !medium.inhabitants.contains(&inhabitant) {
            medium.inhabitants.push(inhabitant);
        }
        self.engine.mark_dirty();
        Ok(())
    }

    /// Remove an inhabitant.
    pub fn remove_inhabitant(&mut self, inhabitant: &str) -> RealityResult<()> {
        let medium = self
            .engine
            .environment_store
            .medium
            .as_mut()
            .ok_or_else(|| RealityError::NotInitialized("environment".into()))?;
        medium.inhabitants.retain(|i| i != inhabitant);
        self.engine.mark_dirty();
        Ok(())
    }
}
