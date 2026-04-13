# Agent Ctx — CLI Specification

## Command Tree
```
agent-ctx
├── snap                    Capture context snapshot
│   ├── --author <name>     Author name for this snapshot
│   ├── --message, -m <msg> Short description of current state
│   └── --include-diff      Include git diff in snapshot
├── load                    Load and display context snapshot
│   ├── --snap <id>         Load specific snapshot (default: latest)
│   ├── --author <name>     Load latest from specific author
│   ├── --format <fmt>      Output format: markdown|json|plain (default: markdown)
│   └── --for <agent>       Agent-specific format: claude|codex|gemini|cursor
├── diff <snap1> <snap2>    Compare two snapshots
│   └── --format <fmt>      Output format: markdown|json|plain
├── sync                    Export context for specific agent
│   ├── --to <agent>        Target agent: claude|codex|gemini|cursor
│   └── --snap <id>         Snapshot to export (default: latest)
├── decide <message>        Record a decision
│   ├── --author <name>     Who made the decision
│   └── --tag <tag>         Category tag (arch, deps, api, etc.)
├── log                     Show decision history
│   ├── --tag <tag>         Filter by tag
│   └── --limit, -n <N>     Show last N decisions (default: 10)
├── config                  Manage configuration
│   ├── set <key> <value>   Set a config value
│   ├── get <key>           Get a config value
│   └── reset               Reset to defaults
├── init                    Initialize .agent-ctx/ in current project
├── self-update             Update to latest version
├── completions <shell>     Generate shell completions
│   └── <shell>             bash|zsh|fish|powershell
└── version                 Show version and build info
```

## Global Flags
```
--help, -h          Show help for any command
--version, -V       Show version
--no-color          Disable colored output
--json              Output as JSON
--quiet, -q         Suppress non-error output
--verbose, -v       Enable verbose output
--no-update-check   Skip update check
```

## Exit Codes
```
0   Success
1   General error (runtime failure)
2   Usage error (bad arguments)
130 Interrupted (Ctrl+C)
```

## Context Directory Structure
```
.agent-ctx/
├── config.toml             Local project config
├── decisions.toml          Team decisions log
├── snaps/
│   ├── 20260414-183000-som.toml
│   ├── 20260414-190000-nid.toml
│   └── latest -> 20260414-190000-nid.toml
└── shared.toml             Shared team context (pinned notes)
```

## Snapshot Format (TOML)
```toml
[meta]
id = "20260414-183000-som"
author = "som"
created_at = "2026-04-14T18:30:00Z"
message = "finished auth endpoint, starting tests"

[project]
name = "my-api"
branch = "feat/add-auth"
commit = "abc1234"
commit_message = "feat: add JWT token generation"

[progress]
current_task = "Task-4: implement login endpoint"
completion = "60%"
modified_files = ["src/auth.rs", "src/routes/login.rs", "tests/auth_test.rs"]
recent_commits = [
    "abc1234 feat: add JWT token generation",
    "def5678 feat: add user model",
    "ghi9012 chore: add auth dependencies",
]

[decisions]
# Decisions are also stored in decisions.toml for persistence
entries = [
    { date = "2026-04-12", author = "som", message = "JWT over session-based auth — stateless, scales better" },
    { date = "2026-04-14", author = "nid", message = "PostgreSQL over MongoDB — need JOIN for user-role queries" },
]

[issues]
items = [
    { severity = "minor", description = "flaky test in auth_test.rs:42" },
]

[notes]
content = "next: implement token refresh + write integration tests"
```

## Config File
```toml
# .agent-ctx/config.toml
config_version = 1

[defaults]
author = "som"
format = "markdown"         # default output format
auto_snap = false           # snap on every git commit

[ignore]
paths = ["target/", "node_modules/", ".git/"]
```

## Error Message Format
```
error: {what went wrong}
  -> {context}
  -> {how to fix}
```

## Example Usage
```bash
# Initialize in a project
agent-ctx init

# Take a snapshot before ending session
agent-ctx snap -m "finished auth, starting tests"

# Next session — load context
agent-ctx load

# Load for a specific agent
agent-ctx load --for claude

# Record a team decision
agent-ctx decide "use Redis for token blacklist — need fast lookups" --tag arch

# View decision history
agent-ctx log

# Teammate picks up work
agent-ctx load --author som

# Compare what changed overnight
agent-ctx diff 20260414-som 20260415-nid

# Pipe to clipboard for pasting into agent
agent-ctx load | pbcopy
```
