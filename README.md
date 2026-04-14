# Agent Ctx
> A DevCool developer tool · devcool.xyz

[![CI](https://github.com/comdevx/cli-agent-ctx/actions/workflows/ci.yml/badge.svg)](https://github.com/comdevx/cli-agent-ctx/actions/workflows/ci.yml)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)

Persist and share development context across AI agent sessions. No more re-explaining your project every time you start a new Claude Code, Codex, or Gemini CLI session.

## Install

### Quick Install (recommended)
```bash
curl -fsSL https://comdevx.github.io/cli-agent-ctx/install.sh | sh
```

### From source
```bash
cargo install --git https://github.com/comdevx/cli-agent-ctx
```

### From GitHub Releases
Download the binary for your platform from [Releases](https://github.com/comdevx/cli-agent-ctx/releases).

## Quick Start

```bash
# Initialize in your project
agent-ctx init

# Capture a snapshot before ending your session
agent-ctx snap -m "finished auth endpoint, starting tests"

# Next session — load context instantly
agent-ctx load

# Load in Claude Code format
agent-ctx load --for claude

# Record a team decision
agent-ctx decide "use JWT over session-based auth — stateless" --tag arch

# View decision history
agent-ctx log
```

## Usage

### Commands

| Command | Description |
|---------|-------------|
| `agent-ctx init` | Initialize `.agent-ctx/` in current project |
| `agent-ctx snap` | Capture context snapshot (git state, decisions, progress) |
| `agent-ctx load` | Load and display latest snapshot |
| `agent-ctx diff <s1> <s2>` | Compare two snapshots |
| `agent-ctx sync --to <agent>` | Export context for specific agent |
| `agent-ctx decide <msg>` | Record a design decision |
| `agent-ctx log` | Show decision history |
| `agent-ctx config` | Manage settings |
| `agent-ctx self-update` | Check for updates |

### Snapshot Flags
```bash
agent-ctx snap --author som -m "finished feature X"
agent-ctx snap --include-diff    # include git diff in snapshot
```

### Load Flags
```bash
agent-ctx load                   # latest snapshot, markdown format
agent-ctx load --format json     # JSON output
agent-ctx load --format plain    # plain text
agent-ctx load --for claude      # Claude Code optimized format
agent-ctx load --for codex       # Codex format
agent-ctx load --author som      # latest from specific author
agent-ctx load | pbcopy          # copy to clipboard
```

### Global Flags
```
--no-color          Disable colored output
--json              Output as JSON
--quiet, -q         Errors only
--verbose, -v       Extra detail
--no-update-check   Skip update check
```

## Team Workflow

```bash
# Developer A: save context before ending session
agent-ctx snap --author alice -m "auth endpoint done"
git add .agent-ctx && git commit -m "ctx: alice session snapshot"
git push

# Developer B: pick up work
git pull
agent-ctx load --author alice
# Paste into new AI agent session — full context restored
```

## Configuration

Config file: `.agent-ctx/config.toml`

```toml
config_version = 1

[defaults]
author = "your-name"
format = "markdown"
auto_snap = false

[ignore]
paths = ["target/", "node_modules/", ".git/"]
```

## Shell Completions

```bash
# Bash
agent-ctx completions bash > ~/.local/share/bash-completion/completions/agent-ctx
# Zsh
agent-ctx completions zsh > ~/.zfunc/_agent-ctx
# Fish
agent-ctx completions fish > ~/.config/fish/completions/agent-ctx.fish
```

## Development

```bash
git clone https://github.com/comdevx/cli-agent-ctx
cd cli-agent-ctx
cargo test
cargo run -- --help
```

## License

Dual-licensed under MIT and Apache 2.0. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
