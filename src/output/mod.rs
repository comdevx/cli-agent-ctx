/// Terminal output abstraction.
///
/// Provides colored, formatted output that respects --no-color,
/// --quiet, --json, and NO_COLOR environment variable.
use colored::Colorize;

/// Output settings derived from CLI flags.
pub struct OutputMode {
    #[allow(dead_code)] // tracked for future use in conditional formatting
    pub color: bool,
    pub quiet: bool,
    pub json: bool,
    pub verbose: bool,
}

impl OutputMode {
    /// Create output mode from CLI flags.
    pub fn new(no_color: bool, quiet: bool, json: bool, verbose: bool) -> Self {
        let color = !no_color && std::env::var("NO_COLOR").is_err();
        if !color {
            colored::control::set_override(false);
        }
        Self {
            color,
            quiet,
            json,
            verbose,
        }
    }

    /// Print a success message.
    pub fn success(&self, msg: &str) {
        if self.quiet {
            return;
        }
        eprintln!("{} {}", "✓".green().bold(), msg);
    }

    /// Print an info message.
    pub fn info(&self, msg: &str) {
        if self.quiet {
            return;
        }
        eprintln!("{} {}", "ℹ".blue().bold(), msg);
    }

    /// Print a warning message.
    pub fn warn(&self, msg: &str) {
        if self.quiet {
            return;
        }
        eprintln!("{} {}", "⚠".yellow().bold(), msg);
    }

    /// Print data to stdout (for piping).
    pub fn data(&self, content: &str) {
        print!("{content}");
    }

    /// Print verbose/debug info.
    pub fn debug(&self, msg: &str) {
        if self.verbose && !self.quiet {
            eprintln!("{} {}", "…".dimmed(), msg.dimmed());
        }
    }
}
