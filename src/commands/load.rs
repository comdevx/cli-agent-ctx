/// Handler for `agent-ctx load`.
///
/// Loads and displays a context snapshot in the specified format.
use anyhow::{Context, Result};
use std::path::Path;

use crate::config::defaults::{CTX_DIR_NAME, SNAPS_DIR};
use crate::core::formatter;
use crate::core::snapshot::Snapshot;
use crate::error::CliError;
use crate::output::OutputMode;

/// Run the load command.
///
/// # Errors
/// Returns error if not initialized or no snapshots found.
pub fn run(
    project_dir: &Path,
    snap_id: Option<&str>,
    author: Option<&str>,
    format: &str,
    agent: Option<&str>,
    out: &OutputMode,
) -> Result<()> {
    let ctx_dir = project_dir.join(CTX_DIR_NAME);
    if !ctx_dir.exists() {
        return Err(CliError::NotInitialized.into());
    }

    let snap = find_snapshot(&ctx_dir.join(SNAPS_DIR), snap_id, author)?;

    let output = if let Some(agent_name) = agent {
        formatter::to_agent_format(&snap, agent_name)
    } else {
        match format {
            "json" => serde_json::to_string_pretty(&snap)
                .context("failed to serialize to JSON")?,
            "plain" => formatter::to_plain(&snap),
            _ => formatter::to_markdown(&snap),
        }
    };

    out.data(&output);
    Ok(())
}

/// Find a snapshot by ID, author, or latest.
fn find_snapshot(
    snaps_dir: &Path,
    snap_id: Option<&str>,
    author: Option<&str>,
) -> Result<Snapshot> {
    if !snaps_dir.exists() {
        return Err(CliError::NoSnapshots.into());
    }

    let mut entries: Vec<_> = std::fs::read_dir(snaps_dir)
        .context("failed to read snapshots directory")?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "toml")
        })
        .collect();

    if entries.is_empty() {
        return Err(CliError::NoSnapshots.into());
    }

    // Sort by filename descending (newest first)
    entries.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    if let Some(id) = snap_id {
        // Find by exact or partial ID match
        for entry in &entries {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with(id) || name_str.contains(id) {
                let content = std::fs::read_to_string(entry.path())
                    .context("failed to read snapshot")?;
                return Snapshot::from_toml(&content);
            }
        }
        return Err(CliError::SnapshotNotFound {
            id: id.to_string(),
        }
        .into());
    }

    if let Some(author_name) = author {
        // Find latest snapshot by author
        for entry in &entries {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.contains(author_name) {
                let content = std::fs::read_to_string(entry.path())
                    .context("failed to read snapshot")?;
                return Snapshot::from_toml(&content);
            }
        }
        return Err(CliError::SnapshotNotFound {
            id: format!("by author: {author_name}"),
        }
        .into());
    }

    // Default: latest snapshot
    let latest = &entries[0];
    let content = std::fs::read_to_string(latest.path())
        .context("failed to read latest snapshot")?;
    Snapshot::from_toml(&content)
}
