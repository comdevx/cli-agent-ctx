/// Handler for `agent-ctx self-update`.
///
/// Checks for updates and automatically downloads + installs the new binary.
use anyhow::{Context, Result};
use std::io::Write;

use crate::config::defaults::GITHUB_REPO;
use crate::output::OutputMode;

/// Run the self-update command.
///
/// # Errors
/// Returns error if the update check or download fails.
pub async fn run(out: &OutputMode) -> Result<()> {
    let current = env!("CARGO_PKG_VERSION");
    out.info(&format!("current version: v{current}"));
    out.info("checking for updates...");

    let latest = check_latest_version()
        .await
        .context("failed to check for updates — are you offline?")?;

    if latest == current {
        out.success("already up to date");
        return Ok(());
    }

    out.info(&format!("new version available: v{latest}"));
    perform_update(&latest, out).await
}

/// Check for updates and prompt user. Called from banner (bare command).
///
/// # Errors
/// Returns error if update check fails.
pub async fn check_and_prompt(out: &OutputMode) -> Result<()> {
    let current = env!("CARGO_PKG_VERSION");

    let latest = match check_latest_version().await {
        Ok(v) => v,
        Err(_) => return Ok(()), // silently skip if offline
    };

    if latest == current || !crate::update::is_newer(&latest, current) {
        return Ok(());
    }

    eprintln!();
    eprintln!("  ┌─────────────────────────────────────────────────┐");
    eprintln!(
        "  │ New version available: v{:<10} (current: v{}) │",
        latest, current
    );
    eprintln!("  └─────────────────────────────────────────────────┘");
    eprint!("  Update now? [Y/n] ");
    std::io::stderr().flush().ok();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    let answer = input.trim().to_lowercase();

    if answer.is_empty() || answer == "y" || answer == "yes" {
        perform_update(&latest, out).await?;
    } else {
        out.info("skipped — run `agent-ctx self-update` anytime to update");
    }

    Ok(())
}

async fn perform_update(version: &str, out: &OutputMode) -> Result<()> {
    let target = detect_target();
    let ext = if cfg!(target_os = "windows") {
        "zip"
    } else {
        "tar.gz"
    };
    let archive_name = format!("agent-ctx-v{version}-{target}.{ext}");
    let url =
        format!("https://github.com/{GITHUB_REPO}/releases/download/v{version}/{archive_name}");

    out.info(&format!("downloading {archive_name}..."));

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "agent-ctx")
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .await
        .context("failed to download update")?;

    if !resp.status().is_success() {
        anyhow::bail!(
            "download failed: HTTP {} — binary may not exist for your platform ({target})",
            resp.status()
        );
    }

    let bytes = resp.bytes().await.context("failed to read download")?;

    let current_exe = std::env::current_exe().context("failed to determine current binary path")?;
    let tmp_dir = tempfile::tempdir().context("failed to create temp directory")?;
    let archive_path = tmp_dir.path().join(&archive_name);

    std::fs::write(&archive_path, &bytes).context("failed to write archive")?;

    // Extract
    out.info("extracting...");
    if cfg!(target_os = "windows") {
        // On Windows, use powershell to extract zip
        std::process::Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "Expand-Archive -Path '{}' -DestinationPath '{}'",
                    archive_path.display(),
                    tmp_dir.path().display()
                ),
            ])
            .output()
            .context("failed to extract zip")?;
    } else {
        std::process::Command::new("tar")
            .args([
                "-xzf",
                &archive_path.to_string_lossy(),
                "-C",
                &tmp_dir.path().to_string_lossy(),
            ])
            .output()
            .context("failed to extract tar.gz")?;
    }

    let new_binary = tmp_dir.path().join("agent-ctx");
    if !new_binary.exists() {
        anyhow::bail!("extracted binary not found in archive");
    }

    // Replace current binary
    out.info("installing...");
    let backup_path = current_exe.with_extension("old");

    // Move current → backup, then move new → current
    if backup_path.exists() {
        std::fs::remove_file(&backup_path).ok();
    }
    std::fs::rename(&current_exe, &backup_path)
        .context("failed to backup current binary — do you need sudo?")?;

    match std::fs::copy(&new_binary, &current_exe) {
        Ok(_) => {
            // Set executable permission on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&current_exe, std::fs::Permissions::from_mode(0o755)).ok();
            }
            // Remove backup
            std::fs::remove_file(&backup_path).ok();
            out.success(&format!("updated to v{version}"));
        }
        Err(e) => {
            // Rollback
            std::fs::rename(&backup_path, &current_exe).ok();
            anyhow::bail!("failed to install new binary: {e}");
        }
    }

    Ok(())
}

fn detect_target() -> String {
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "unknown"
    };

    let os = if cfg!(target_os = "linux") {
        "unknown-linux-gnu"
    } else if cfg!(target_os = "macos") {
        "apple-darwin"
    } else if cfg!(target_os = "windows") {
        "pc-windows-msvc"
    } else {
        "unknown"
    };

    format!("{arch}-{os}")
}

async fn check_latest_version() -> Result<String> {
    let url = format!("https://api.github.com/repos/{GITHUB_REPO}/releases/latest");
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
        .unwrap_or("0.0.0")
        .trim_start_matches('v');
    Ok(tag.to_string())
}
