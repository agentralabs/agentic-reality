//! MCP protocol handler — JSON-RPC message processing, negotiation, and validation.

pub mod compact;
pub mod handler;
pub mod negotiation;
pub mod validator;

pub use handler::ProtocolHandler;
pub use negotiation::{ProtocolNegotiator, SUPPORTED_VERSIONS};
pub use validator::RequestValidator;
