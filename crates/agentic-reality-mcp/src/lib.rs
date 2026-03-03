//! AgenticReality MCP Server — universal LLM access to existential grounding.

pub mod config;
pub mod prompts;
pub mod protocol;
pub mod resources;
pub mod session;
pub mod tools;
pub mod transport;
pub mod types;

pub use config::ServerConfig;
pub use protocol::ProtocolHandler;
pub use session::SessionManager;
pub use transport::StdioTransport;
