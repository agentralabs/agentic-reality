//! Sister bridge traits — 9 traits with NoOp defaults for standalone operation.

/// Time bridge for AgenticTime integration.
pub trait TimeBridge: Send + Sync {
    fn link_deadline(&self, _anchor_id: &str, _deadline_id: &str) -> Result<(), String> {
        Ok(())
    }
    fn temporal_context(&self, _topic: &str) -> Vec<String> {
        vec![]
    }
    fn is_deadline_past(&self, _deadline_id: &str) -> Option<bool> {
        None
    }
}

/// Contract bridge for AgenticContract integration.
pub trait ContractBridge: Send + Sync {
    fn check_policy(&self, _operation: &str, _context: &str) -> Result<bool, String> {
        Ok(true)
    }
    fn record_operation(&self, _operation: &str, _context: &str) -> Result<(), String> {
        Ok(())
    }
}

/// Identity bridge for AgenticIdentity integration.
pub trait IdentityBridge: Send + Sync {
    fn verify_identity(&self, _agent_id: &str) -> Result<bool, String> {
        Ok(true)
    }
    fn resolve_identity(&self, _agent_id: &str) -> Option<String> {
        None
    }
    fn sign_data(&self, _data: &[u8]) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

/// Memory bridge for AgenticMemory integration.
pub trait MemoryBridge: Send + Sync {
    fn store_context(&self, _key: &str, _value: &str) -> Result<(), String> {
        Ok(())
    }
    fn recall_context(&self, _key: &str) -> Option<String> {
        None
    }
    fn ground_claim(&self, _claim: &str) -> Result<f64, String> {
        Ok(0.0)
    }
}

/// Cognition bridge for AgenticCognition integration.
pub trait CognitionBridge: Send + Sync {
    fn assess_coherence(&self, _context: &str) -> Result<f64, String> {
        Ok(1.0)
    }
    fn suggest_action(&self, _context: &str) -> Option<String> {
        None
    }
}

/// Communication bridge for AgenticComm integration.
pub trait CommBridge: Send + Sync {
    fn broadcast_state(&self, _state: &str) -> Result<(), String> {
        Ok(())
    }
    fn receive_state(&self) -> Option<String> {
        None
    }
}

/// Codebase bridge for AgenticCodebase integration.
pub trait CodebaseBridge: Send + Sync {
    fn get_context(&self, _path: &str) -> Option<String> {
        None
    }
    fn analyze_impact(&self, _change: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }
}

/// Vision bridge for AgenticVision integration.
pub trait VisionBridge: Send + Sync {
    fn capture_state(&self, _description: &str) -> Result<String, String> {
        Ok(String::new())
    }
    fn query_visual(&self, _query: &str) -> Vec<String> {
        vec![]
    }
}

/// Planning bridge for AgenticPlanning integration.
pub trait PlanningBridge: Send + Sync {
    fn register_constraint(&self, _constraint: &str) -> Result<(), String> {
        Ok(())
    }
    fn get_plan_context(&self) -> Option<String> {
        None
    }
}

/// NoOp implementation of all bridges for standalone operation.
pub struct NoOpBridges;

impl TimeBridge for NoOpBridges {}
impl ContractBridge for NoOpBridges {}
impl IdentityBridge for NoOpBridges {}
impl MemoryBridge for NoOpBridges {}
impl CognitionBridge for NoOpBridges {}
impl CommBridge for NoOpBridges {}
impl CodebaseBridge for NoOpBridges {}
impl VisionBridge for NoOpBridges {}
impl PlanningBridge for NoOpBridges {}

/// Hydra adapter trait for orchestrator integration.
pub trait HydraAdapter: Send + Sync {
    fn register_with_hydra(&self) -> Result<(), String> {
        Ok(())
    }
    fn report_health(&self) -> Result<String, String> {
        Ok("healthy".to_string())
    }
    fn accept_command(&self, _command: &str) -> Result<String, String> {
        Ok(String::new())
    }
}

impl HydraAdapter for NoOpBridges {}

/// Ghost writer trait for snapshot/restore.
pub trait RealityGhostWriter: Send + Sync {
    fn snapshot(&self) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
    fn restore(&self, _data: &[u8]) -> Result<(), String> {
        Ok(())
    }
    fn get_delta(&self, _since: i64) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
    fn apply_delta(&self, _delta: &[u8]) -> Result<(), String> {
        Ok(())
    }
    fn get_checksum(&self) -> Result<String, String> {
        Ok(String::new())
    }
    fn get_ghost_hint(&self) -> Option<String> {
        None
    }
}

impl RealityGhostWriter for NoOpBridges {}
