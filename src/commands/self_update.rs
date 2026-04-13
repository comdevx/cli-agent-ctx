/// Handler for `agent-ctx self-update`.
///
/// Checks for the latest version and provides update instructions.
use anyhow::Result;

use crate::config::defaults::GITHUB_REPO;
use crate::output::OutputMode;

/// Run the self-update command.
///
/// # Errors
/// Returns error if the GitHub API request fails.
pub async fn run(out: &OutputMode) -> Result<()> {
    let current = env!("CARGO_PKG_VERSION");
    out.info(&format!("current version: v{current}"));
    out.info("checking for updates...");

    match check_latest_version().await {
        Ok(latest) => {
            if latest.trim_start_matches('v') == current {
                out.success("already up to date");
            } else {
                out.info(&format!("latest version: v{latest}"));
                out.info(&format!(
                    "download: https://github.com/{GITHUB_REPO}/releases/latest"
                ));
                out.info("or run: cargo install agent-ctx");
            }
        }
        Err(_) => {
            out.warn("could not check for updates — are you offline?");
            out.info(&format!(
                "check manually: https://github.com/{GITHUB_REPO}/releases"
            ));
        }
    }

    Ok(())
}

async fn check_latest_version() -> Result<String> {
    let url = format!(
        "https://api.github.com/repos/{GITHUB_REPO}/releases/latest"
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "agent-ctx")
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await?;

    let json: serde_json::Value = resp.json().await?;
    let tag = json["tag_name"]
        .as_str()
        .unwrap_or("unknown")
        .trim_start_matches('v');
    Ok(tag.to_string())
}
