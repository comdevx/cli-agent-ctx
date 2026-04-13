/// Handler for `agent-ctx snap`.
///
/// Captures a context snapshot of the current project state.
use anyhow::{Context, Result};
use chrono::Utc;
use std::path::Path;

use crate::config::defaults::{CTX_DIR_NAME, DECISIONS_FILE, SNAPS_DIR};
use crate::config::ProjectConfig;
use crate::core::decisions::DecisionLog;
use crate::core::git;
use crate::core::snapshot::*;
use crate::error::CliError;
use crate::output::OutputMode;

/// Run the snap command.
///
/// # Errors
/// Returns error if not initialized, not a git repo, or write fails.
pub fn run(
    project_dir: &Path,
    author: Option<&str>,
    message: Option<&str>,
    include_diff: bool,
    out: &OutputMode,
) -> Result<()> {
    let ctx_dir = project_dir.join(CTX_DIR_NAME);
    if !ctx_dir.exists() {
        return Err(CliError::NotInitialized.into());
    }

    if !git::is_git_repo(project_dir) {
        return Err(CliError::NotGitRepo.into());
    }

    out.debug("reading project config");
    let config = ProjectConfig::load(&ctx_dir.join("config.toml"))
        .unwrap_or_default();

    let author_name = author
        .map(String::from)
        .unwrap_or(config.defaults.author);

    out.debug("reading git state");
    let git_state = git::read_git_state(project_dir)
        .context("failed to read git state")?;

    let decisions = DecisionLog::load(&ctx_dir.join(DECISIONS_FILE))
        .unwrap_or_default();

    let snap_id = Snapshot::generate_id(&author_name);

    let mut notes = String::new();
    if include_diff {
        if let Ok(diff) = git::get_diff(project_dir) {
            if !diff.is_empty() {
                notes.push_str("## Diff\n\n```\n");
                notes.push_str(&diff);
                notes.push_str("\n```\n");
            }
        }
    }

    let snapshot = Snapshot {
        meta: SnapshotMeta {
            id: snap_id.clone(),
            author: author_name,
            created_at: Utc::now(),
            message: message.unwrap_or("").to_string(),
        },
        project: ProjectInfo {
            name: git_state.repo_name,
            branch: git_state.branch,
            commit: git_state.commit_hash,
            commit_message: git_state.commit_message,
        },
        progress: Progress {
            current_task: String::new(),
            modified_files: git_state.modified_files,
            recent_commits: git_state.recent_commits,
        },
        decisions: decisions.decisions,
        issues: vec![],
        notes,
    };

    let toml_content = snapshot.to_toml()?;
    let snap_path = ctx_dir.join(SNAPS_DIR).join(format!("{snap_id}.toml"));

    std::fs::write(&snap_path, &toml_content)
        .context("failed to write snapshot file")?;

    if out.json {
        let json = serde_json::to_string_pretty(&snapshot)
            .context("failed to serialize snapshot to JSON")?;
        out.data(&json);
    } else {
        out.success(&format!("snapshot saved: {snap_id}"));
        out.info(&format!("  branch: {}", snapshot.project.branch));
        out.info(&format!("  commit: {}", snapshot.project.commit));
        out.info(&format!(
            "  modified files: {}",
            snapshot.progress.modified_files.len()
        ));
        out.info(&format!(
            "  decisions: {}",
            snapshot.decisions.len()
        ));
    }

    Ok(())
}
