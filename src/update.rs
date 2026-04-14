/// Background update checker — runs on every CLI startup.
///
/// Non-blocking: spawns a tokio task that checks GitHub API
/// and prints a notification to stderr if a newer version exists.
/// Never delays or blocks normal CLI execution.
use crate::config::defaults::GITHUB_REPO;

/// Spawn a background task to check for updates.
///
/// Prints to stderr if a newer version is available.
/// Silently does nothing on failure (offline, timeout, etc.).
pub fn check_in_background() {
    let current = env!("CARGO_PKG_VERSION").to_string();
    tokio::spawn(async move {
        if let Ok(latest) = fetch_latest_version().await {
            let latest_clean = latest.trim_start_matches('v');
            if latest_clean != current && is_newer(latest_clean, &current) {
                eprintln!();
                eprintln!("  ┌─────────────────────────────────────────────────┐");
                eprintln!("  │ A new version of agent-ctx is available!         │");
                eprintln!(
                    "  │ Current: v{:<10} Latest: v{:<17}│",
                    current, latest_clean
                );
                eprintln!("  │ Run `agent-ctx self-update` to see details       │");
                eprintln!("  └─────────────────────────────────────────────────┘");
            }
        }
    });
}

/// Compare two semver strings. Returns true if `latest` > `current`.
pub fn is_newer(latest: &str, current: &str) -> bool {
    let parse = |s: &str| -> (u32, u32, u32) {
        let parts: Vec<u32> = s.split('.').filter_map(|p| p.parse().ok()).collect();
        (
            parts.first().copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
            parts.get(2).copied().unwrap_or(0),
        )
    };
    parse(latest) > parse(current)
}

async fn fetch_latest_version() -> anyhow::Result<String> {
    let url = format!("https://api.github.com/repos/{GITHUB_REPO}/releases/latest");
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "agent-ctx")
        .timeout(std::time::Duration::from_secs(3))
        .send()
        .await?;

    let json: serde_json::Value = resp.json().await?;
    let tag = json["tag_name"].as_str().unwrap_or("0.0.0").to_string();
    Ok(tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_newer() {
        assert!(is_newer("0.2.0", "0.1.0"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(is_newer("0.1.1", "0.1.0"));
        assert!(!is_newer("0.1.0", "0.1.0"));
        assert!(!is_newer("0.1.0", "0.2.0"));
    }
}
