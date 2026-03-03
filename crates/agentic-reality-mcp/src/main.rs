//! AgenticReality MCP Server binary entry point.
//!
//! Reads JSON-RPC from stdin, writes responses to stdout, logs to stderr.
//! Supports --data-dir, --mode, and --port arguments.

use std::io::{self, BufRead, Write as _};
use std::sync::Arc;

use clap::Parser;
use tokio::sync::Mutex;

use agentic_reality_mcp::config::{self, ServerConfig};
use agentic_reality_mcp::protocol::ProtocolHandler;
use agentic_reality_mcp::session::SessionManager;

/// AgenticReality MCP Server — existential grounding for AI agents.
#[derive(Parser)]
#[command(name = "agentic-reality-mcp", version, about = "MCP server for AgenticReality")]
struct Cli {
    /// Server mode: stdio or http
    #[arg(long, default_value = "stdio")]
    mode: String,

    /// HTTP port (only for http mode)
    #[arg(long, default_value = "3010")]
    port: u16,

    /// Data directory for .areal file persistence
    #[arg(long)]
    data_dir: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // Merge CLI args with env-based config
    let mut server_config = config::load_config();
    server_config.mode = cli.mode.clone();
    server_config.port = cli.port;
    if cli.data_dir.is_some() {
        server_config.data_dir = cli.data_dir.clone();
    }

    // Initialize tracing to stderr
    init_stderr_logging(&server_config.log_level);

    tracing::info!(
        mode = %server_config.mode,
        port = server_config.port,
        "AgenticReality MCP server starting"
    );

    match server_config.mode.as_str() {
        "stdio" => {
            if let Err(e) = run_stdio(&server_config) {
                tracing::error!(error = %e, "stdio server error");
                std::process::exit(1);
            }
        }
        "http" => {
            tracing::info!(port = server_config.port, "HTTP/SSE mode not yet implemented");
            eprintln!(
                "AgenticReality MCP server: HTTP mode on port {} (not yet implemented)",
                server_config.port
            );
        }
        other => {
            tracing::error!(mode = %other, "unknown server mode");
            eprintln!("error: unknown mode '{}', expected 'stdio' or 'http'", other);
            std::process::exit(1);
        }
    }
}

/// Initialize tracing subscriber that logs to stderr.
fn init_stderr_logging(level: &str) {
    let filter = match level {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    let subscriber = tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_max_level(filter)
        .with_target(false)
        .compact()
        .finish();

    // If this fails (e.g., subscriber already set), continue silently
    let _ = tracing::subscriber::set_global_default(subscriber);
}

/// Run the MCP server in stdio mode.
///
/// Reads one JSON-RPC message per line from stdin, processes it,
/// and writes the response as a single line to stdout.
fn run_stdio(config: &ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("failed to build tokio runtime: {}", e))?;

    rt.block_on(async {
        let mut session = SessionManager::new();

        // Configure data path if provided
        if let Some(ref dir) = config.data_dir {
            let data_path = config::resolve_data_path(Some(dir));
            let areal_path = data_path.join("reality.areal");
            session = SessionManager::with_path(
                areal_path.to_string_lossy().to_string(),
            );
            session.set_autosave(config.autosave);

            // Attempt to load existing session data
            match session.load() {
                Ok(true) => tracing::info!("loaded existing session data"),
                Ok(false) => tracing::debug!("no existing session file found"),
                Err(e) => tracing::warn!(error = %e, "failed to load session data, starting fresh"),
            }
        }

        let session = Arc::new(Mutex::new(session));
        let handler = ProtocolHandler::new(session.clone());
        let stdin = io::stdin();
        let reader = stdin.lock();
        let stdout = io::stdout();

        for line_result in reader.lines() {
            let line = match line_result {
                Ok(l) => l,
                Err(e) => {
                    tracing::error!(error = %e, "failed to read from stdin");
                    break;
                }
            };

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let value: serde_json::Value = match serde_json::from_str(trimmed) {
                Ok(v) => v,
                Err(e) => {
                    // Send JSON-RPC parse error
                    let error_response = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": serde_json::Value::Null,
                        "error": {
                            "code": -32700,
                            "message": format!("Parse error: {}", e),
                        }
                    });
                    write_response(&stdout, &error_response);
                    continue;
                }
            };

            // Extract method before moving value
            let is_notification = value.get("id").is_none();
            let method = value
                .get("method")
                .and_then(|m| m.as_str())
                .unwrap_or("")
                .to_string();

            // Handle notifications/initialized silently
            if is_notification && method == "notifications/initialized" {
                tracing::info!("client confirmed initialization");
                continue;
            }

            // Handle notifications/cancelled
            if is_notification && method == "notifications/cancelled" {
                tracing::info!("client sent cancellation");
                continue;
            }

            // Process request (consumes value)
            let response = handler.handle_request(value).await;
            write_response(&stdout, &response);

            // Track initialization state
            if method == "initialize" {
                tracing::info!("initialization handshake sent");
            }

            // Autosave after tool calls if configured
            if method == "tools/call" {
                let mut session_guard = session.lock().await;
                if let Err(e) = session_guard.autosave_if_dirty() {
                    tracing::warn!(error = %e, "autosave failed");
                }
            }
        }

        // Final save on shutdown
        {
            let mut session_guard = session.lock().await;
            if session_guard.is_dirty() {
                match session_guard.save() {
                    Ok(true) => tracing::info!("session saved on shutdown"),
                    Ok(false) => {}
                    Err(e) => tracing::warn!(error = %e, "failed to save session on shutdown"),
                }
            }
        }

        tracing::info!("MCP server shutting down cleanly");
        Ok(())
    })
}

/// Write a JSON-RPC response as a single line to stdout.
fn write_response(stdout: &io::Stdout, response: &serde_json::Value) {
    let mut handle = stdout.lock();
    match serde_json::to_string(response) {
        Ok(json_str) => {
            let _ = handle.write_all(json_str.as_bytes());
            let _ = handle.write_all(b"\n");
            let _ = handle.flush();
        }
        Err(e) => {
            tracing::error!(error = %e, "failed to serialize response");
        }
    }
}
