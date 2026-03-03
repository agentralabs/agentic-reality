//! Server configuration.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub mode: String,
    pub port: u16,
    pub auth_token: Option<String>,
    pub data_dir: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            mode: "stdio".to_string(),
            port: 3010,
            auth_token: None,
            data_dir: None,
        }
    }
}
