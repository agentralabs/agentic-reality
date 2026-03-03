//! MCP type definitions — error handling, capabilities, messages, requests, responses, notifications.

pub mod capabilities;
pub mod error;
pub mod message;
pub mod notification;
pub mod request;
pub mod response;

pub use capabilities::{
    ClientCapabilities, PromptCapability, ResourceCapability, ServerCapabilities, ToolCapability,
};
pub use error::{McpError, McpResult, ToolCallResult, ToolContent, ToolDefinition};
pub use message::{
    JsonRpcErrorBody, JsonRpcMessage, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse,
    MessageId,
};
pub use notification::{CancelledNotification, LogLevel, LogNotification, ProgressNotification};
pub use request::{
    ClientInfo, InitializeRequest, PromptsListRequest, ResourcesListRequest, ToolsCallRequest,
    ToolsListRequest,
};
pub use response::{
    InitializeResponse, JsonRpcError, ServerInfo, ToolsCallResponse, ToolsListResponse,
};
