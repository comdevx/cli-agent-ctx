/// Error types for Agent Ctx CLI.
///
/// Uses `thiserror` for structured error types with helpful messages.
/// Application code uses `anyhow::Result` for convenience.
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("not initialized — run `agent-ctx init` first")]
    NotInitialized,

    #[error("already initialized in {path}")]
    AlreadyInitialized { path: PathBuf },

    #[error("snapshot not found: {id}")]
    SnapshotNotFound { id: String },

    #[error("no snapshots found — run `agent-ctx snap` first")]
    NoSnapshots,

    #[error("not a git repository")]
    NotGitRepo,

    #[error("config not found at {path}")]
    ConfigNotFound { path: PathBuf },

    #[error("invalid argument: {message}")]
    InvalidArgument { message: String },

    #[error("git command failed: {message}")]
    GitError { message: String },

    #[error("failed to write snapshot: {message}")]
    WriteError { message: String },
}
