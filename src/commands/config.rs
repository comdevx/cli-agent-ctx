/// Handler for `agent-ctx config`.
///
/// Manages project-level configuration.
use anyhow::{Context, Result};
use std::path::Path;

use crate::cli::ConfigAction;
use crate::config::defaults::{CONFIG_FILE, CTX_DIR_NAME};
use crate::config::ProjectConfig;
use crate::error::CliError;
use crate::output::OutputMode;

/// Run the config command.
///
/// # Errors
/// Returns error if not initialized or config operation fails.
pub fn run(project_dir: &Path, action: &ConfigAction, out: &OutputMode) -> Result<()> {
    let ctx_dir = project_dir.join(CTX_DIR_NAME);
    if !ctx_dir.exists() {
        return Err(CliError::NotInitialized.into());
    }

    let config_path = ctx_dir.join(CONFIG_FILE);

    match action {
        ConfigAction::Get { key } => {
            let config = ProjectConfig::load(&config_path)?;
            match config.get(key) {
                Some(value) => out.data(&format!("{value}\n")),
                None => {
                    anyhow::bail!("unknown config key: {key}");
                }
            }
        }
        ConfigAction::Set { key, value } => {
            let mut config = ProjectConfig::load(&config_path)?;
            config
                .set(key, value)
                .context("failed to set config value")?;
            config.save(&config_path).context("failed to save config")?;
            out.success(&format!("set {key} = {value}"));
        }
        ConfigAction::Reset => {
            let config = ProjectConfig::default();
            config.save(&config_path).context("failed to save config")?;
            out.success("config reset to defaults");
        }
    }

    Ok(())
}
