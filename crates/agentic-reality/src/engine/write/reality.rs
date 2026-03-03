//! Reality physics write operations.

use crate::engine::write::WriteEngine;
use crate::types::error::{RealityError, RealityResult};
use crate::types::ids::AnchorId;
use crate::types::reality::*;

impl<'a> WriteEngine<'a> {
    /// Set the current reality layer.
    pub fn set_reality_layer(&mut self, layers: RealityLayers) -> RealityResult<()> {
        self.engine.reality_store.layers = Some(layers);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update a specific layer status.
    pub fn update_layer_status(&mut self, status: LayerStatus) -> RealityResult<()> {
        let layers = self.engine.reality_store.layers.as_mut().ok_or_else(|| {
            RealityError::NotInitialized("reality layers".into())
        })?;
        layers.layers.push(status);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update freshness perception.
    pub fn update_freshness(&mut self, freshness: FreshnessPerception) -> RealityResult<()> {
        self.engine.reality_store.freshness = Some(freshness);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add a reality anchor.
    pub fn add_anchor(&mut self, anchor: RealityAnchor) -> RealityResult<AnchorId> {
        let id = anchor.id;
        self.engine.reality_store.anchors.push(anchor);
        self.engine.mark_dirty();
        Ok(id)
    }

    /// Remove a reality anchor.
    pub fn remove_anchor(&mut self, id: &AnchorId) -> RealityResult<()> {
        let len = self.engine.reality_store.anchors.len();
        self.engine.reality_store.anchors.retain(|a| a.id != *id);
        if self.engine.reality_store.anchors.len() == len {
            return Err(RealityError::NotFound(format!("anchor {}", id)));
        }
        self.engine.mark_dirty();
        Ok(())
    }

    /// Verify an anchor and update its value.
    pub fn verify_anchor(&mut self, id: &AnchorId, value: AnchorValue) -> RealityResult<()> {
        let anchor = self.engine.reality_store.anchors.iter_mut().find(|a| a.id == *id)
            .ok_or_else(|| RealityError::NotFound(format!("anchor {}", id)))?;
        anchor.last_value = value;
        self.engine.mark_dirty();
        Ok(())
    }

    /// Record drift in an anchor.
    pub fn record_anchor_drift(&mut self, drift: AnchorDrift) -> RealityResult<()> {
        self.engine.reality_store.anchor_drifts.push(drift);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Detect a hallucination.
    pub fn detect_hallucination(&mut self, hallucination: DetectedHallucination) -> RealityResult<()> {
        let state = self.engine.reality_store.hallucination_state.get_or_insert_with(|| {
            HallucinationState {
                risk_level: HallucinationRisk::Low { confidence: 1.0 },
                detected: vec![],
                patterns: vec![],
                grounding: GroundingStatus {
                    grounded: true, anchor_count: 0, verified_count: 0,
                    last_verification: None, confidence: 1.0,
                },
                pending_verification: vec![],
            }
        });
        state.detected.push(hallucination);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Clear a resolved hallucination.
    pub fn clear_hallucination(&mut self, id: &str) -> RealityResult<()> {
        if let Some(state) = &mut self.engine.reality_store.hallucination_state {
            if let Some(h) = state.detected.iter_mut().find(|h| h.id == id) {
                h.resolved = true;
            }
        }
        self.engine.mark_dirty();
        Ok(())
    }

    /// Add an unverified claim.
    pub fn add_unverified_claim(&mut self, claim: UnverifiedClaim) -> RealityResult<()> {
        let state = self.engine.reality_store.hallucination_state.get_or_insert_with(|| {
            HallucinationState {
                risk_level: HallucinationRisk::Low { confidence: 1.0 },
                detected: vec![],
                patterns: vec![],
                grounding: GroundingStatus {
                    grounded: true, anchor_count: 0, verified_count: 0,
                    last_verification: None, confidence: 1.0,
                },
                pending_verification: vec![],
            }
        });
        state.pending_verification.push(claim);
        self.engine.mark_dirty();
        Ok(())
    }

    /// Verify a pending claim (remove from pending).
    pub fn verify_claim(&mut self, claim_text: &str) -> RealityResult<()> {
        if let Some(state) = &mut self.engine.reality_store.hallucination_state {
            state.pending_verification.retain(|c| c.claim != claim_text);
        }
        self.engine.mark_dirty();
        Ok(())
    }

    /// Update grounding status.
    pub fn update_grounding(&mut self, grounding: GroundingStatus) -> RealityResult<()> {
        if let Some(state) = &mut self.engine.reality_store.hallucination_state {
            state.grounding = grounding;
        }
        self.engine.mark_dirty();
        Ok(())
    }
}
