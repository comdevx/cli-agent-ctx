use clap::Parser;

mod app;
mod cli;
mod commands;
mod config;
mod core;
mod error;
mod output;
mod update;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cli = cli::Cli::parse();

    // Background update check — non-blocking, never delays CLI
    if !cli.no_update_check {
        update::check_in_background();
    }

    if let Err(err) = app::run(cli).await {
        let code = if err.downcast_ref::<crate::error::CliError>().is_some() {
            match err.downcast_ref::<crate::error::CliError>() {
                Some(crate::error::CliError::InvalidArgument { .. }) => 2,
                _ => 1,
            }
        } else {
            1
        };
        eprintln!("error: {err:#}");
        std::process::exit(code);
    }
}
