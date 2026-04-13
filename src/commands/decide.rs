/// Handler for `agent-ctx decide`.
///
/// Records an architectural or design decision.
use anyhow::{Context, Result};
use chrono::Utc;
use std::path::Path;

use crate::config::defaults::{CTX_DIR_NAME, DECISIONS_FILE};
use crate::config::ProjectConfig;
use crate::core::decisions::DecisionLog;
use crate::core::snapshot::Decision;
use crate::error::CliError;
use crate::output::OutputMode;

/// Run the decide command.
///
/// # Errors
/// Returns error if not initialized or write fails.
pub fn run(
    project_dir: &Path,
    message: &str,
    author: Option<&str>,
    tag: Option<&str>,
    out: &OutputMode,
) -> Result<()> {
    let ctx_dir = project_dir.join(CTX_DIR_NAME);
    if !ctx_dir.exists() {
        return Err(CliError::NotInitialized.into());
    }

    let config = ProjectConfig::load(&ctx_dir.join("config.toml")).unwrap_or_default();

    let author_name = author.map(String::from).unwrap_or(config.defaults.author);

    let decisions_path = ctx_dir.join(DECISIONS_FILE);
    let mut log = DecisionLog::load(&decisions_path).unwrap_or_default();

    let decision = Decision {
        date: Utc::now().format("%Y-%m-%d").to_string(),
        author: author_name,
        message: message.to_string(),
        tag: tag.unwrap_or("general").to_string(),
    };

    let idx = log.add(decision.clone());
    log.save(&decisions_path)
        .context("failed to save decision")?;

    if out.json {
        let json =
            serde_json::to_string_pretty(&decision).context("failed to serialize decision")?;
        out.data(&json);
    } else {
        out.success(&format!("decision #{} recorded: {}", idx + 1, message));
        out.info(&format!("  tag: {}", decision.tag));
        out.info(&format!("  author: {}", decision.author));
    }

    Ok(())
}
