/// Snapshot data model and serialization.
///
/// A snapshot captures the full context of a project at a point in time.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A complete context snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub meta: SnapshotMeta,
    pub project: ProjectInfo,
    pub progress: Progress,
    pub decisions: Vec<Decision>,
    pub issues: Vec<Issue>,
    pub notes: String,
}

/// Snapshot metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMeta {
    pub id: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
}

/// Project git state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub branch: String,
    pub commit: String,
    pub commit_message: String,
}

/// Current progress state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    pub current_task: String,
    pub modified_files: Vec<String>,
    pub recent_commits: Vec<String>,
}

/// A design or architecture decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub date: String,
    pub author: String,
    pub message: String,
    pub tag: String,
}

/// A known issue or concern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub severity: String,
    pub description: String,
}

impl Snapshot {
    /// Generate a snapshot ID from timestamp and author.
    pub fn generate_id(author: &str) -> String {
        let now = Utc::now();
        format!("{}-{}", now.format("%Y%m%d-%H%M%S"), author)
    }

    /// Serialize snapshot to TOML string.
    ///
    /// # Errors
    /// Returns error if serialization fails.
    pub fn to_toml(&self) -> anyhow::Result<String> {
        toml::to_string_pretty(self).map_err(|e| anyhow::anyhow!("failed to serialize snapshot: {e}"))
    }

    /// Deserialize snapshot from TOML string.
    ///
    /// # Errors
    /// Returns error if the TOML is malformed or missing required fields.
    pub fn from_toml(s: &str) -> anyhow::Result<Self> {
        toml::from_str(s).map_err(|e| anyhow::anyhow!("failed to parse snapshot: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot() -> Snapshot {
        Snapshot {
            meta: SnapshotMeta {
                id: "20260414-183000-som".to_string(),
                author: "som".to_string(),
                created_at: Utc::now(),
                message: "finished auth endpoint".to_string(),
            },
            project: ProjectInfo {
                name: "my-api".to_string(),
                branch: "feat/add-auth".to_string(),
                commit: "abc1234".to_string(),
                commit_message: "feat: add JWT token generation".to_string(),
            },
            progress: Progress {
                current_task: "Task-4: implement login endpoint".to_string(),
                modified_files: vec!["src/auth.rs".to_string()],
                recent_commits: vec!["abc1234 feat: add JWT token generation".to_string()],
            },
            decisions: vec![Decision {
                date: "2026-04-12".to_string(),
                author: "som".to_string(),
                message: "JWT over session-based auth".to_string(),
                tag: "arch".to_string(),
            }],
            issues: vec![],
            notes: "next: implement token refresh".to_string(),
        }
    }

    #[test]
    fn test_snapshot_roundtrip_toml() {
        let snap = sample_snapshot();
        let toml_str = snap.to_toml().unwrap();
        let parsed = Snapshot::from_toml(&toml_str).unwrap();
        assert_eq!(parsed.meta.id, snap.meta.id);
        assert_eq!(parsed.project.branch, snap.project.branch);
        assert_eq!(parsed.decisions.len(), 1);
    }

    #[test]
    fn test_generate_id_format() {
        let id = Snapshot::generate_id("testuser");
        assert!(id.ends_with("-testuser"));
        assert!(id.len() > 20);
    }
}
