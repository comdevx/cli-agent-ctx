/// Application orchestrator — matches CLI commands to handlers.
///
/// No business logic here — only dispatch.
use anyhow::Result;
use clap::CommandFactory;
use std::path::PathBuf;

use crate::cli::{Cli, Command};
use crate::commands::version::BANNER;
use crate::output::OutputMode;

/// Run the application with parsed CLI arguments.
///
/// # Errors
/// Returns error from the dispatched command handler.
pub async fn run(cli: Cli) -> Result<()> {
    let out = OutputMode::new(cli.no_color, cli.quiet, cli.json, cli.verbose);
    let project_dir = PathBuf::from(".");

    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            // No subcommand — show banner + help
            eprintln!("{BANNER}");
            let version = env!("CARGO_PKG_VERSION");
            eprintln!("    v{version} · AI Agent Context Manager · by DevCool\n");

            // Check for updates and prompt user
            if !cli.no_update_check {
                crate::commands::self_update::check_and_prompt(&out)
                    .await
                    .ok();
                eprintln!();
            }

            Cli::command().print_help()?;
            return Ok(());
        }
    };

    match command {
        Command::Init => crate::commands::init::run(&project_dir, &out),
        Command::Snap {
            author,
            message,
            include_diff,
        } => crate::commands::snap::run(
            &project_dir,
            author.as_deref(),
            message.as_deref(),
            include_diff,
            &out,
        ),
        Command::Load {
            snap,
            author,
            format,
            r#for,
        } => crate::commands::load::run(
            &project_dir,
            snap.as_deref(),
            author.as_deref(),
            &format,
            r#for.as_deref(),
            &out,
        ),
        Command::Diff {
            snap1,
            snap2,
            format,
        } => crate::commands::diff::run(&project_dir, &snap1, &snap2, &format, &out),
        Command::Sync { to, snap } => {
            crate::commands::sync::run(&project_dir, &to, snap.as_deref(), &out)
        }
        Command::Decide {
            message,
            author,
            tag,
        } => crate::commands::decide::run(
            &project_dir,
            &message,
            author.as_deref(),
            tag.as_deref(),
            &out,
        ),
        Command::Log { tag, limit } => {
            crate::commands::log::run(&project_dir, tag.as_deref(), limit, &out)
        }
        Command::Config { action } => crate::commands::config::run(&project_dir, &action, &out),
        Command::SelfUpdate => crate::commands::self_update::run(&out).await,
        Command::Completions { shell } => {
            crate::commands::completions::run(shell);
            Ok(())
        }
        Command::Version => {
            crate::commands::version::run(&out);
            Ok(())
        }
    }
}
