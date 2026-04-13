/// Output formatting — converts snapshots to various text formats.
///
/// Supports markdown, JSON, plain text, and agent-specific formats.
use super::snapshot::Snapshot;

/// Format a snapshot as Markdown (default output).
pub fn to_markdown(snap: &Snapshot) -> String {
    let mut out = String::new();

    out.push_str(&format!("# Context: {}\n\n", snap.project.name));
    out.push_str(&format!(
        "**Branch:** `{}` | **Commit:** `{}` | **Author:** {}\n\n",
        snap.project.branch, snap.project.commit, snap.meta.author
    ));

    if !snap.meta.message.is_empty() {
        out.push_str(&format!("> {}\n\n", snap.meta.message));
    }

    out.push_str("## Current Progress\n\n");
    if !snap.progress.current_task.is_empty() {
        out.push_str(&format!("**Task:** {}\n\n", snap.progress.current_task));
    }

    if !snap.progress.modified_files.is_empty() {
        out.push_str("**Modified files:**\n");
        for f in &snap.progress.modified_files {
            out.push_str(&format!("- `{f}`\n"));
        }
        out.push('\n');
    }

    if !snap.progress.recent_commits.is_empty() {
        out.push_str("**Recent commits:**\n");
        for c in &snap.progress.recent_commits {
            out.push_str(&format!("- {c}\n"));
        }
        out.push('\n');
    }

    if !snap.decisions.is_empty() {
        out.push_str("## Decisions\n\n");
        for d in &snap.decisions {
            out.push_str(&format!("- **{}** ({}): {}\n", d.tag, d.author, d.message));
        }
        out.push('\n');
    }

    if !snap.issues.is_empty() {
        out.push_str("## Known Issues\n\n");
        for issue in &snap.issues {
            out.push_str(&format!("- [{}] {}\n", issue.severity, issue.description));
        }
        out.push('\n');
    }

    if !snap.notes.is_empty() {
        out.push_str(&format!("## Notes\n\n{}\n", snap.notes));
    }

    out
}

/// Format a snapshot as plain text (minimal, no markdown).
pub fn to_plain(snap: &Snapshot) -> String {
    let mut out = String::new();

    out.push_str(&format!("Project: {}\n", snap.project.name));
    out.push_str(&format!("Branch:  {}\n", snap.project.branch));
    out.push_str(&format!(
        "Commit:  {} — {}\n",
        snap.project.commit, snap.project.commit_message
    ));
    out.push_str(&format!("Author:  {}\n", snap.meta.author));
    if !snap.meta.message.is_empty() {
        out.push_str(&format!("Status:  {}\n", snap.meta.message));
    }
    out.push('\n');

    if !snap.progress.current_task.is_empty() {
        out.push_str(&format!("Task: {}\n", snap.progress.current_task));
    }

    if !snap.progress.modified_files.is_empty() {
        out.push_str(&format!(
            "Modified: {}\n",
            snap.progress.modified_files.join(", ")
        ));
    }

    if !snap.progress.recent_commits.is_empty() {
        out.push_str("\nRecent commits:\n");
        for c in &snap.progress.recent_commits {
            out.push_str(&format!("  {c}\n"));
        }
    }

    if !snap.decisions.is_empty() {
        out.push_str("\nDecisions:\n");
        for d in &snap.decisions {
            out.push_str(&format!("  [{}] {} — {}\n", d.tag, d.message, d.author));
        }
    }

    if !snap.issues.is_empty() {
        out.push_str("\nIssues:\n");
        for issue in &snap.issues {
            out.push_str(&format!("  [{}] {}\n", issue.severity, issue.description));
        }
    }

    if !snap.notes.is_empty() {
        out.push_str(&format!("\nNotes: {}\n", snap.notes));
    }

    out
}

/// Format a snapshot for Claude Code (optimized markdown).
pub fn to_claude_format(snap: &Snapshot) -> String {
    let mut out = String::new();

    out.push_str("# Project Context (for Claude Code)\n\n");
    out.push_str(&format!(
        "You are working on **{}**.\n\n",
        snap.project.name
    ));
    out.push_str(&format!(
        "- Branch: `{}`\n- Last commit: `{}` — {}\n",
        snap.project.branch, snap.project.commit, snap.project.commit_message
    ));

    if !snap.progress.current_task.is_empty() {
        out.push_str(&format!(
            "\n## Current Task\n\n{}\n",
            snap.progress.current_task
        ));
    }

    if !snap.progress.modified_files.is_empty() {
        out.push_str("\n## Recently Modified Files\n\n");
        for f in &snap.progress.modified_files {
            out.push_str(&format!("- `{f}`\n"));
        }
    }

    if !snap.decisions.is_empty() {
        out.push_str("\n## Team Decisions (follow these)\n\n");
        for d in &snap.decisions {
            out.push_str(&format!(
                "- {} (decided by {}, {})\n",
                d.message, d.author, d.date
            ));
        }
    }

    if !snap.notes.is_empty() {
        out.push_str(&format!("\n## Notes\n\n{}\n", snap.notes));
    }

    out
}

/// Format a snapshot for Codex.
pub fn to_codex_format(snap: &Snapshot) -> String {
    let mut out = String::new();

    out.push_str(&format!("Project: {}\n", snap.project.name));
    out.push_str(&format!("Branch: {}\n", snap.project.branch));
    out.push_str(&format!(
        "Last commit: {} — {}\n\n",
        snap.project.commit, snap.project.commit_message
    ));

    if !snap.progress.current_task.is_empty() {
        out.push_str(&format!("Current task: {}\n\n", snap.progress.current_task));
    }

    if !snap.decisions.is_empty() {
        out.push_str("Decisions:\n");
        for d in &snap.decisions {
            out.push_str(&format!("- {}\n", d.message));
        }
        out.push('\n');
    }

    if !snap.notes.is_empty() {
        out.push_str(&format!("Notes: {}\n", snap.notes));
    }

    out
}

/// Format a snapshot for a named agent.
///
/// Returns the formatted string, falling back to markdown for unknown agents.
pub fn to_agent_format(snap: &Snapshot, agent: &str) -> String {
    match agent.to_lowercase().as_str() {
        "claude" => to_claude_format(snap),
        "codex" => to_codex_format(snap),
        "gemini" => to_markdown(snap),
        "cursor" => to_markdown(snap),
        _ => to_markdown(snap),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::snapshot::*;

    fn sample_snapshot() -> Snapshot {
        Snapshot {
            meta: SnapshotMeta {
                id: "20260414-183000-som".to_string(),
                author: "som".to_string(),
                created_at: chrono::Utc::now(),
                message: "finished auth endpoint".to_string(),
            },
            project: ProjectInfo {
                name: "my-api".to_string(),
                branch: "feat/add-auth".to_string(),
                commit: "abc1234".to_string(),
                commit_message: "feat: add JWT token generation".to_string(),
            },
            progress: Progress {
                current_task: "Task-4: implement login".to_string(),
                modified_files: vec!["src/auth.rs".to_string()],
                recent_commits: vec!["abc1234 feat: add JWT".to_string()],
            },
            decisions: vec![Decision {
                date: "2026-04-12".to_string(),
                author: "som".to_string(),
                message: "JWT over session-based auth".to_string(),
                tag: "arch".to_string(),
            }],
            issues: vec![],
            notes: "next: token refresh".to_string(),
        }
    }

    #[test]
    fn test_markdown_contains_key_info() {
        let md = to_markdown(&sample_snapshot());
        assert!(md.contains("my-api"));
        assert!(md.contains("feat/add-auth"));
        assert!(md.contains("JWT over session-based auth"));
        assert!(md.contains("Task-4"));
    }

    #[test]
    fn test_plain_contains_key_info() {
        let plain = to_plain(&sample_snapshot());
        assert!(plain.contains("my-api"));
        assert!(plain.contains("feat/add-auth"));
    }

    #[test]
    fn test_claude_format_has_header() {
        let claude = to_claude_format(&sample_snapshot());
        assert!(claude.contains("Project Context (for Claude Code)"));
        assert!(claude.contains("Team Decisions"));
    }

    #[test]
    fn test_agent_format_fallback() {
        let snap = sample_snapshot();
        let unknown = to_agent_format(&snap, "unknown-agent");
        let md = to_markdown(&snap);
        assert_eq!(unknown, md);
    }
}
