/// Decision management — persistent team decisions.
///
/// Decisions are stored in `.agent-ctx/decisions.toml` and persist
/// across snapshots. They represent team-level architectural choices.
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::snapshot::Decision;

/// Container for all decisions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DecisionLog {
    pub decisions: Vec<Decision>,
}

impl DecisionLog {
    /// Load decisions from a TOML file.
    ///
    /// # Errors
    /// Returns error if the file cannot be read or parsed.
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path).context("failed to read decisions file")?;
        toml::from_str(&content).context("failed to parse decisions file")
    }

    /// Save decisions to a TOML file.
    ///
    /// # Errors
    /// Returns error if serialization or file writing fails.
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("failed to serialize decisions")?;
        std::fs::write(path, content).context("failed to write decisions file")
    }

    /// Add a new decision and return its index.
    pub fn add(&mut self, decision: Decision) -> usize {
        self.decisions.push(decision);
        self.decisions.len() - 1
    }

    /// Filter decisions by tag.
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&Decision> {
        self.decisions.iter().filter(|d| d.tag == tag).collect()
    }

    /// Get the last N decisions.
    pub fn last_n(&self, n: usize) -> Vec<&Decision> {
        let start = self.decisions.len().saturating_sub(n);
        self.decisions[start..].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_filter() {
        let mut log = DecisionLog::default();
        log.add(Decision {
            date: "2026-04-14".to_string(),
            author: "som".to_string(),
            message: "use JWT".to_string(),
            tag: "arch".to_string(),
        });
        log.add(Decision {
            date: "2026-04-14".to_string(),
            author: "nid".to_string(),
            message: "use PostgreSQL".to_string(),
            tag: "deps".to_string(),
        });

        assert_eq!(log.decisions.len(), 2);
        assert_eq!(log.filter_by_tag("arch").len(), 1);
        assert_eq!(log.filter_by_tag("deps").len(), 1);
        assert_eq!(log.filter_by_tag("none").len(), 0);
    }

    #[test]
    fn test_last_n() {
        let mut log = DecisionLog::default();
        for i in 0..5 {
            log.add(Decision {
                date: format!("2026-04-{:02}", 10 + i),
                author: "test".to_string(),
                message: format!("decision {i}"),
                tag: "arch".to_string(),
            });
        }
        assert_eq!(log.last_n(3).len(), 3);
        assert_eq!(log.last_n(10).len(), 5);
    }

    #[test]
    fn test_roundtrip_toml() {
        let mut log = DecisionLog::default();
        log.add(Decision {
            date: "2026-04-14".to_string(),
            author: "som".to_string(),
            message: "use JWT".to_string(),
            tag: "arch".to_string(),
        });

        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("decisions.toml");
        log.save(&path).unwrap();

        let loaded = DecisionLog::load(&path).unwrap();
        assert_eq!(loaded.decisions.len(), 1);
        assert_eq!(loaded.decisions[0].message, "use JWT");
    }
}
