/// Configuration management for Agent Ctx.
///
/// Handles both project-level config (.agent-ctx/config.toml)
/// and global config (~/.config/agent-ctx/config.toml).
pub mod defaults;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Project-level configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub config_version: u32,
    pub defaults: DefaultsConfig,
    pub ignore: IgnoreConfig,
}

/// Default values for commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
    pub author: String,
    pub format: String,
    pub auto_snap: bool,
}

/// Paths to ignore when collecting context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IgnoreConfig {
    pub paths: Vec<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            config_version: 1,
            defaults: DefaultsConfig {
                author: whoami::username(),
                format: "markdown".to_string(),
                auto_snap: false,
            },
            ignore: IgnoreConfig {
                paths: vec![
                    "target/".to_string(),
                    "node_modules/".to_string(),
                    ".git/".to_string(),
                ],
            },
        }
    }
}

impl ProjectConfig {
    /// Load config from a TOML file.
    ///
    /// # Errors
    /// Returns error if the file cannot be read or parsed.
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).context("failed to read config file")?;
        toml::from_str(&content).context("failed to parse config file")
    }

    /// Save config to a TOML file.
    ///
    /// # Errors
    /// Returns error if serialization or file writing fails.
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("failed to serialize config")?;
        std::fs::write(path, content).context("failed to write config file")
    }

    /// Get a config value by key.
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "author" => Some(self.defaults.author.clone()),
            "format" => Some(self.defaults.format.clone()),
            "auto_snap" => Some(self.defaults.auto_snap.to_string()),
            _ => None,
        }
    }

    /// Set a config value by key.
    ///
    /// # Errors
    /// Returns error if the key is not recognized.
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "author" => self.defaults.author = value.to_string(),
            "format" => {
                if !["markdown", "json", "plain"].contains(&value) {
                    anyhow::bail!("invalid format: {value} — must be markdown, json, or plain");
                }
                self.defaults.format = value.to_string();
            }
            "auto_snap" => {
                self.defaults.auto_snap =
                    value.parse().context("auto_snap must be true or false")?;
            }
            _ => anyhow::bail!("unknown config key: {key}"),
        }
        Ok(())
    }
}

/// Find the `.agent-ctx/` directory by walking up from the given path.
#[allow(dead_code)] // will be used for nested directory support
pub fn find_ctx_dir(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        let candidate = current.join(".agent-ctx");
        if candidate.is_dir() {
            return Some(candidate);
        }
        if !current.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ProjectConfig::default();
        assert_eq!(config.config_version, 1);
        assert_eq!(config.defaults.format, "markdown");
        assert!(!config.defaults.auto_snap);
    }

    #[test]
    fn test_set_get() {
        let mut config = ProjectConfig::default();
        config.set("author", "nid").unwrap();
        assert_eq!(config.get("author").unwrap(), "nid");

        config.set("format", "json").unwrap();
        assert_eq!(config.get("format").unwrap(), "json");
    }

    #[test]
    fn test_set_invalid_format() {
        let mut config = ProjectConfig::default();
        assert!(config.set("format", "xml").is_err());
    }

    #[test]
    fn test_set_unknown_key() {
        let mut config = ProjectConfig::default();
        assert!(config.set("nonexistent", "value").is_err());
    }

    #[test]
    fn test_roundtrip_toml() {
        let config = ProjectConfig::default();
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        config.save(&path).unwrap();
        let loaded = ProjectConfig::load(&path).unwrap();
        assert_eq!(loaded.defaults.format, config.defaults.format);
    }
}
