/// Handler for `agent-ctx sync`.
///
/// Exports context in agent-specific formats.
use anyhow::Result;
use std::path::Path;

use crate::output::OutputMode;

/// Run the sync command.
///
/// # Errors
/// Returns error if snapshot loading fails.
pub fn run(
    project_dir: &Path,
    to_agent: &str,
    snap_id: Option<&str>,
    out: &OutputMode,
) -> Result<()> {
    // Reuse load with agent format
    crate::commands::load::run(project_dir, snap_id, None, "markdown", Some(to_agent), out)
}
