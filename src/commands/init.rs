/// Handler for `agent-ctx init`.
///
/// Creates the `.agent-ctx/` directory structure in the current project.
use anyhow::{Context, Result};
use std::path::Path;

use crate::config::defaults::{CONFIG_FILE, CTX_DIR_NAME, DECISIONS_FILE, SNAPS_DIR};
use crate::config::ProjectConfig;
use crate::core::decisions::DecisionLog;
use crate::error::CliError;
use crate::output::OutputMode;

/// Run the init command.
///
/// # Errors
/// Returns error if the directory already exists or cannot be created.
pub fn run(project_dir: &Path, out: &OutputMode) -> Result<()> {
    let ctx_dir = project_dir.join(CTX_DIR_NAME);

    if ctx_dir.exists() {
        return Err(CliError::AlreadyInitialized { path: ctx_dir }.into());
    }

    out.debug("creating .agent-ctx/ directory structure");

    std::fs::create_dir_all(ctx_dir.join(SNAPS_DIR))
        .context("failed to create snapshots directory")?;

    let config = ProjectConfig::default();
    config
        .save(&ctx_dir.join(CONFIG_FILE))
        .context("failed to write config")?;

    let decisions = DecisionLog::default();
    decisions
        .save(&ctx_dir.join(DECISIONS_FILE))
        .context("failed to write decisions")?;

    out.success(&format!(
        "initialized .agent-ctx/ in {}",
        project_dir.display()
    ));
    out.info("run `agent-ctx snap` to capture your first context snapshot");

    Ok(())
}
