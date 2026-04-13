/// Handler for `agent-ctx log`.
///
/// Displays decision history.
use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;

use crate::config::defaults::{CTX_DIR_NAME, DECISIONS_FILE};
use crate::core::decisions::DecisionLog;
use crate::error::CliError;
use crate::output::OutputMode;

/// Run the log command.
///
/// # Errors
/// Returns error if not initialized.
pub fn run(
    project_dir: &Path,
    tag: Option<&str>,
    limit: usize,
    out: &OutputMode,
) -> Result<()> {
    let ctx_dir = project_dir.join(CTX_DIR_NAME);
    if !ctx_dir.exists() {
        return Err(CliError::NotInitialized.into());
    }

    let log = DecisionLog::load(&ctx_dir.join(DECISIONS_FILE))
        .unwrap_or_default();

    let decisions = if let Some(tag_filter) = tag {
        let filtered = log.filter_by_tag(tag_filter);
        let start = filtered.len().saturating_sub(limit);
        filtered[start..].to_vec()
    } else {
        log.last_n(limit).into_iter().collect()
    };

    if decisions.is_empty() {
        out.info("no decisions recorded yet — use `agent-ctx decide` to add one");
        return Ok(());
    }

    if out.json {
        let json = serde_json::to_string_pretty(&decisions)
            .context("failed to serialize decisions")?;
        out.data(&json);
    } else {
        out.data(&format!(
            "{}\n\n",
            "Decision Log".bold().underline()
        ));
        for (i, d) in decisions.iter().enumerate() {
            out.data(&format!(
                "{}. {} {} — {} ({})\n",
                i + 1,
                format!("[{}]", d.tag).blue(),
                d.message,
                d.author.dimmed(),
                d.date.dimmed(),
            ));
        }
    }

    Ok(())
}
