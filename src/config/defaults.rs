/// Default constants for Agent Ctx.
///
/// Directory name for agent context data.
pub const CTX_DIR_NAME: &str = ".agent-ctx";

/// Subdirectory for snapshots.
pub const SNAPS_DIR: &str = "snaps";

/// Config file name.
pub const CONFIG_FILE: &str = "config.toml";

/// Decisions file name.
pub const DECISIONS_FILE: &str = "decisions.toml";

/// Maximum number of recent commits to capture.
#[allow(dead_code)] // used in future snapshot filtering
pub const MAX_RECENT_COMMITS: usize = 10;

/// GitHub repository for update checks.
pub const GITHUB_REPO: &str = "comdevx/cli-agent-ctx";
