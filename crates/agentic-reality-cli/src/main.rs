//! AgenticReality CLI binary.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "areal", version, about = "AgenticReality CLI — existential grounding for AI agents")]
struct Cli {
    /// Output format: text, json, or table
    #[arg(long, default_value = "text")]
    format: String,

    /// Enable verbose output
    #[arg(long)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage deployment soul
    Soul {
        #[command(subcommand)]
        action: SoulAction,
    },
    /// View environment state
    Env {
        #[command(subcommand)]
        action: EnvAction,
    },
    /// View resource body
    Resource {
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Start MCP server
    Serve {
        /// Server mode
        #[arg(long, default_value = "stdio")]
        mode: String,
    },
    /// Show version info
    Info,
}

#[derive(Subcommand)]
enum SoulAction {
    /// Show deployment soul
    Show,
    /// Show soul vitals
    Vitals,
}

#[derive(Subcommand)]
enum EnvAction {
    /// Show environment
    Show,
    /// Show environment mood
    Mood,
}

#[derive(Subcommand)]
enum ResourceAction {
    /// Show resource body
    Show,
    /// Show mind (memory) capacity
    Mind,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Info) | None => {
            println!("areal v{}", env!("CARGO_PKG_VERSION"));
            println!("AgenticReality — existential grounding for AI agents");
            println!("Sister #10 \"The Ground\" — Agentra Labs ecosystem");
        }
        Some(Commands::Soul { action }) => match action {
            SoulAction::Show => println!("Deployment soul not initialized"),
            SoulAction::Vitals => println!("No vitals available"),
        },
        Some(Commands::Env { action }) => match action {
            EnvAction::Show => println!("Environment not sensed"),
            EnvAction::Mood => println!("No mood data"),
        },
        Some(Commands::Resource { action }) => match action {
            ResourceAction::Show => println!("Resource body not initialized"),
            ResourceAction::Mind => println!("Mind capacity unknown"),
        },
        Some(Commands::Serve { mode }) => {
            println!("Starting MCP server (mode={})", mode);
        }
    }
}
