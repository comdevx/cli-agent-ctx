/// Git integration — reads git state without modifying it.
///
/// All functions are read-only. No git writes happen here.
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Information about the current git state.
#[derive(Debug, Clone)]
pub struct GitState {
    pub branch: String,
    pub commit_hash: String,
    pub commit_message: String,
    pub modified_files: Vec<String>,
    pub recent_commits: Vec<String>,
    pub repo_name: String,
}

/// Check if the current directory is inside a git repository.
pub fn is_git_repo(path: &Path) -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(path)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Read the current git state from the working directory.
///
/// # Errors
/// Returns error if git commands fail or the directory is not a git repo.
pub fn read_git_state(path: &Path) -> Result<GitState> {
    let branch = run_git(path, &["rev-parse", "--abbrev-ref", "HEAD"])
        .context("failed to get current branch")?;

    let commit_hash = run_git(path, &["rev-parse", "--short", "HEAD"])
        .context("failed to get commit hash")?;

    let commit_message = run_git(path, &["log", "-1", "--pretty=%s"])
        .context("failed to get commit message")?;

    let modified_raw = run_git(path, &["diff", "--name-only", "HEAD"])
        .unwrap_or_default();
    let staged_raw = run_git(path, &["diff", "--name-only", "--cached"])
        .unwrap_or_default();

    let mut modified_files: Vec<String> = modified_raw
        .lines()
        .chain(staged_raw.lines())
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect();
    modified_files.sort();
    modified_files.dedup();

    let recent_raw = run_git(path, &["log", "--oneline", "-10"])
        .unwrap_or_default();
    let recent_commits: Vec<String> = recent_raw
        .lines()
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect();

    let repo_name = run_git(path, &["rev-parse", "--show-toplevel"])
        .unwrap_or_default();
    let repo_name = Path::new(repo_name.trim())
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    Ok(GitState {
        branch,
        commit_hash,
        commit_message,
        modified_files,
        recent_commits,
        repo_name,
    })
}

/// Get the git diff output for including in snapshots.
///
/// # Errors
/// Returns error if git diff command fails.
pub fn get_diff(path: &Path) -> Result<String> {
    run_git(path, &["diff", "HEAD"]).context("failed to get git diff")
}

fn run_git(path: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(path)
        .output()
        .context("failed to execute git")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git {} failed: {}", args.join(" "), stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn init_test_repo() -> TempDir {
        let dir = TempDir::new().unwrap();
        Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        std::fs::write(dir.path().join("README.md"), "# Test").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        dir
    }

    #[test]
    fn test_is_git_repo() {
        let dir = init_test_repo();
        assert!(is_git_repo(dir.path()));
    }

    #[test]
    fn test_not_git_repo() {
        let dir = TempDir::new().unwrap();
        assert!(!is_git_repo(dir.path()));
    }

    #[test]
    fn test_read_git_state() {
        let dir = init_test_repo();
        let state = read_git_state(dir.path()).unwrap();
        assert!(!state.commit_hash.is_empty());
        assert!(!state.branch.is_empty());
        assert_eq!(state.commit_message, "init");
    }
}
