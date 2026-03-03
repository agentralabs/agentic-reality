//! Minimal MCP client that talks to the AgenticReality MCP server via stdio.
//!
//! This example spawns the MCP server as a child process, sends an initialize
//! handshake followed by a tools/list request, and prints the result.
//!
//! Prerequisites:
//!   cargo build --workspace --release
//!
//! Run with:
//!   cargo run --example mcp_client

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() {
    println!("=== AgenticReality MCP Client Example ===\n");

    // Locate the MCP server binary
    let server_bin = find_server_binary();
    println!("Using server binary: {}", server_bin);

    // Spawn the MCP server in stdio mode
    let mut child = Command::new(&server_bin)
        .arg("--mode")
        .arg("stdio")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to spawn MCP server");

    let stdin = child.stdin.as_mut().expect("failed to open stdin");
    let stdout = child.stdout.take().expect("failed to open stdout");
    let mut reader = BufReader::new(stdout);

    // -----------------------------------------------------------------------
    // Step 1: Send initialize request
    // -----------------------------------------------------------------------
    let init_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "example-client",
                "version": "0.1.0"
            }
        }
    });

    println!("Sending initialize request...");
    send_request(stdin, &init_request);

    let init_response = read_response(&mut reader);
    println!("Initialize response:");
    println!("  {}\n", serde_json::to_string_pretty(&init_response).unwrap_or_default());

    // Send initialized notification (no id, no response expected)
    let initialized_notification = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized",
        "params": {}
    });
    send_request(stdin, &initialized_notification);

    // -----------------------------------------------------------------------
    // Step 2: List available tools
    // -----------------------------------------------------------------------
    let list_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    });

    println!("Sending tools/list request...");
    send_request(stdin, &list_request);

    let list_response = read_response(&mut reader);

    // Extract tool names
    if let Some(tools) = list_response
        .get("result")
        .and_then(|r| r.get("tools"))
        .and_then(|t| t.as_array())
    {
        println!("Available tools ({}):", tools.len());
        for tool in tools {
            if let Some(name) = tool.get("name").and_then(|n| n.as_str()) {
                let desc = tool
                    .get("description")
                    .and_then(|d| d.as_str())
                    .unwrap_or("");
                println!("  {} -- {}", name, desc);
            }
        }
    } else {
        println!("Tools list response:");
        println!(
            "  {}",
            serde_json::to_string_pretty(&list_response).unwrap_or_default()
        );
    }

    // -----------------------------------------------------------------------
    // Step 3: Call reality_context tool
    // -----------------------------------------------------------------------
    let call_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "reality_context",
            "arguments": {
                "operation": "summary"
            }
        }
    });

    println!("\nSending tools/call reality_context...");
    send_request(stdin, &call_request);

    let call_response = read_response(&mut reader);
    println!("Context summary response:");
    println!(
        "  {}",
        serde_json::to_string_pretty(&call_response).unwrap_or_default()
    );

    // -----------------------------------------------------------------------
    // Clean up
    // -----------------------------------------------------------------------
    drop(stdin); // close stdin to signal EOF
    let _ = child.wait();

    println!("\n=== Done ===");
}

fn send_request(stdin: &mut impl Write, request: &serde_json::Value) {
    let json = serde_json::to_string(request).expect("failed to serialize request");
    writeln!(stdin, "{}", json).expect("failed to write to stdin");
    stdin.flush().expect("failed to flush stdin");
}

fn read_response(reader: &mut impl BufRead) -> serde_json::Value {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("failed to read response line");
    serde_json::from_str(line.trim()).unwrap_or_else(|e| {
        serde_json::json!({
            "error": format!("failed to parse response: {} -- raw: {}", e, line.trim())
        })
    })
}

fn find_server_binary() -> String {
    // Try release build first, then debug
    let candidates = [
        "target/release/agentic-reality-mcp",
        "target/debug/agentic-reality-mcp",
    ];

    for candidate in &candidates {
        if std::path::Path::new(candidate).exists() {
            return candidate.to_string();
        }
    }

    // Fall back to PATH lookup
    "agentic-reality-mcp".to_string()
}
