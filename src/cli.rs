/// CLI argument definitions using clap derive API.
///
/// This file contains ONLY data structures — no logic.
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "agent-ctx")]
#[command(version, about = "Agent Ctx — AI Agent Context Manager")]
#[command(
    long_about = "Persist and share development context across AI agent sessions.\nNo more re-explaining your project every time you start a new session."
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,

    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Skip update check
    #[arg(long, global = true)]
    pub no_update_check: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initialize .agent-ctx/ in current project
    Init,

    /// Capture a context snapshot
    Snap {
        /// Author name for this snapshot
        #[arg(long)]
        author: Option<String>,

        /// Short description of current state
        #[arg(short, long)]
        message: Option<String>,

        /// Include git diff in snapshot
        #[arg(long)]
        include_diff: bool,
    },

    /// Load and display a context snapshot
    Load {
        /// Load specific snapshot by ID
        #[arg(long)]
        snap: Option<String>,

        /// Load latest from specific author
        #[arg(long)]
        author: Option<String>,

        /// Output format: markdown, json, plain
        #[arg(long, default_value = "markdown")]
        format: String,

        /// Agent-specific format: claude, codex, gemini, cursor
        #[arg(long, name = "agent")]
        r#for: Option<String>,
    },

    /// Compare two snapshots
    Diff {
        /// First snapshot ID
        snap1: String,

        /// Second snapshot ID
        snap2: String,

        /// Output format: markdown, json, plain
        #[arg(long, default_value = "markdown")]
        format: String,
    },

    /// Export context for a specific agent
    Sync {
        /// Target agent: claude, codex, gemini, cursor
        #[arg(long)]
        to: String,

        /// Snapshot to export (default: latest)
        #[arg(long)]
        snap: Option<String>,
    },

    /// Record an architectural or design decision
    Decide {
        /// Decision message
        message: String,

        /// Who made the decision
        #[arg(long)]
        author: Option<String>,

        /// Category tag (arch, deps, api, etc.)
        #[arg(long)]
        tag: Option<String>,
    },

    /// Show decision history
    Log {
        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,

        /// Show last N decisions
        #[arg(short = 'n', long, default_value = "10")]
        limit: usize,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Update to the latest version
    SelfUpdate,

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },

    /// Show version and build info
    Version,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Set a config value
    Set {
        /// Config key
        key: String,
        /// Config value
        value: String,
    },
    /// Get a config value
    Get {
        /// Config key
        key: String,
    },
    /// Reset to defaults
    Reset,
}
