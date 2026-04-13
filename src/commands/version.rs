/// Handler for `agent-ctx version`.
///
/// Shows extended version and build info.
use crate::output::OutputMode;

/// Run the version command.
pub fn run(out: &OutputMode) {
    let version = env!("CARGO_PKG_VERSION");

    if out.json {
        let info = serde_json::json!({
            "name": "agent-ctx",
            "version": version,
            "target": std::env::consts::ARCH,
            "os": std::env::consts::OS,
        });
        if let Ok(json) = serde_json::to_string_pretty(&info) {
            out.data(&json);
        }
    } else {
        out.data(&format!("agent-ctx {version}\n"));
        out.data(&format!("target: {}-{}\n", std::env::consts::ARCH, std::env::consts::OS));
    }
}
