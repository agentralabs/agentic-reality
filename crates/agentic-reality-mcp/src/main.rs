//! AgenticReality MCP Server binary entry point.

use clap::Parser;

#[derive(Parser)]
#[command(name = "agentic-reality-mcp", version, about = "MCP server for AgenticReality")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Start the MCP server
    Serve {
        /// Server mode: stdio or http
        #[arg(long, default_value = "stdio")]
        mode: String,
        /// HTTP port (only for http mode)
        #[arg(long, default_value = "3010")]
        port: u16,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Serve { mode, port }) => {
            eprintln!("AgenticReality MCP server starting (mode={}, port={})", mode, port);
            // MCP server implementation will be in Phase 5
        }
        None => {
            eprintln!("AgenticReality MCP server — use 'serve' subcommand");
        }
    }
}
