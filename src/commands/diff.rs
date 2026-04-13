/// Handler for `agent-ctx diff`.
///
/// Compares two snapshots and shows what changed.
use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::HashSet;
use std::path::Path;

use crate::config::defaults::{CTX_DIR_NAME, SNAPS_DIR};
use crate::core::snapshot::Snapshot;
use crate::error::CliError;
use crate::output::OutputMode;

/// Run the diff command.
///
/// # Errors
/// Returns error if snapshots are not found.
pub fn run(
    project_dir: &Path,
    snap1_id: &str,
    snap2_id: &str,
    format: &str,
    out: &OutputMode,
) -> Result<()> {
    let snaps_dir = project_dir.join(CTX_DIR_NAME).join(SNAPS_DIR);
    let snap1 = load_snap_by_id(&snaps_dir, snap1_id)?;
    let snap2 = load_snap_by_id(&snaps_dir, snap2_id)?;

    if out.json || format == "json" {
        let diff = build_diff_json(&snap1, &snap2)?;
        out.data(&diff);
    } else {
        let diff = build_diff_text(&snap1, &snap2);
        out.data(&diff);
    }

    Ok(())
}

fn load_snap_by_id(snaps_dir: &Path, id: &str) -> Result<Snapshot> {
    let entries = std::fs::read_dir(snaps_dir)
        .context("failed to read snapshots directory")?;

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with(id) || name_str.contains(id) {
            let content = std::fs::read_to_string(entry.path())
                .context("failed to read snapshot")?;
            return Snapshot::from_toml(&content);
        }
    }

    Err(CliError::SnapshotNotFound { id: id.to_string() }.into())
}

fn build_diff_text(snap1: &Snapshot, snap2: &Snapshot) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "Comparing: {} → {}\n\n",
        snap1.meta.id, snap2.meta.id
    ));

    if snap1.project.branch != snap2.project.branch {
        out.push_str(&format!(
            "{} branch: {} → {}\n",
            "~".yellow(),
            snap1.project.branch,
            snap2.project.branch
        ));
    }

    if snap1.project.commit != snap2.project.commit {
        out.push_str(&format!(
            "{} commit: {} → {}\n",
            "~".yellow(),
            snap1.project.commit,
            snap2.project.commit
        ));
    }

    // Modified files diff
    let files1: HashSet<_> = snap1.progress.modified_files.iter().collect();
    let files2: HashSet<_> = snap2.progress.modified_files.iter().collect();

    for f in files2.difference(&files1) {
        out.push_str(&format!("{} modified: {f}\n", "+".green()));
    }
    for f in files1.difference(&files2) {
        out.push_str(&format!("{} no longer modified: {f}\n", "-".red()));
    }

    // Decisions diff
    let d1_msgs: HashSet<_> = snap1.decisions.iter().map(|d| &d.message).collect();
    for d in &snap2.decisions {
        if !d1_msgs.contains(&d.message) {
            out.push_str(&format!(
                "{} new decision: {} ({})\n",
                "+".green(),
                d.message,
                d.author
            ));
        }
    }

    // New commits between snapshots
    let c1: HashSet<_> = snap1.progress.recent_commits.iter().collect();
    let new_commits: Vec<_> = snap2
        .progress
        .recent_commits
        .iter()
        .filter(|c| !c1.contains(c))
        .collect();

    if !new_commits.is_empty() {
        out.push_str(&format!(
            "\n{} new commits:\n",
            new_commits.len()
        ));
        for c in new_commits {
            out.push_str(&format!("  {c}\n"));
        }
    }

    out
}

fn build_diff_json(snap1: &Snapshot, snap2: &Snapshot) -> Result<String> {
    let diff = serde_json::json!({
        "from": snap1.meta.id,
        "to": snap2.meta.id,
        "branch_changed": snap1.project.branch != snap2.project.branch,
        "commit_changed": snap1.project.commit != snap2.project.commit,
        "from_branch": snap1.project.branch,
        "to_branch": snap2.project.branch,
        "from_commit": snap1.project.commit,
        "to_commit": snap2.project.commit,
    });
    serde_json::to_string_pretty(&diff).context("failed to serialize diff")
}
