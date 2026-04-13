/// Handler for `agent-ctx completions`.
///
/// Generates shell completion scripts.
use clap::CommandFactory;
use clap_complete::generate;

use crate::cli::Cli;

/// Run the completions command.
pub fn run(shell: clap_complete::Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "agent-ctx", &mut std::io::stdout());
}
