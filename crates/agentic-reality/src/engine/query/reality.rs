//! Reality query operations.

use crate::engine::query::QueryEngine;
use crate::types::error::{RealityError, RealityResult};
use crate::types::ids::AnchorId;
use crate::types::reality::*;

impl<'a> QueryEngine<'a> {
    pub fn get_reality_layers(&self) -> RealityResult<&RealityLayers> {
        self.engine.reality_store.layers.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("reality layers".into()))
    }

    pub fn get_current_layer(&self) -> RealityResult<&RealityLayer> {
        Ok(&self.get_reality_layers()?.current_layer)
    }

    pub fn get_freshness(&self) -> RealityResult<&FreshnessPerception> {
        self.engine.reality_store.freshness.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("freshness perception".into()))
    }

    pub fn is_fresh(&self, source: &str) -> bool {
        self.engine.reality_store.freshness.as_ref()
            .and_then(|f| f.by_source.get(source))
            .map(|s| matches!(s.level, FreshnessLevel::Live { .. } | FreshnessLevel::Fresh { .. }))
            .unwrap_or(false)
    }

    pub fn get_anchors(&self) -> &[RealityAnchor] {
        &self.engine.reality_store.anchors
    }

    pub fn get_anchor(&self, id: &AnchorId) -> RealityResult<&RealityAnchor> {
        self.engine.reality_store.anchors.iter().find(|a| a.id == *id)
            .ok_or_else(|| RealityError::NotFound(format!("anchor {}", id)))
    }

    pub fn get_anchor_drift(&self) -> &[AnchorDrift] {
        &self.engine.reality_store.anchor_drifts
    }

    pub fn get_hallucination_state(&self) -> RealityResult<&HallucinationState> {
        self.engine.reality_store.hallucination_state.as_ref()
            .ok_or_else(|| RealityError::NotInitialized("hallucination state".into()))
    }

    pub fn get_hallucination_risk(&self) -> Option<&HallucinationRisk> {
        self.engine.reality_store.hallucination_state.as_ref().map(|s| &s.risk_level)
    }

    pub fn get_grounding_status(&self) -> Option<&GroundingStatus> {
        self.engine.reality_store.hallucination_state.as_ref().map(|s| &s.grounding)
    }
}
