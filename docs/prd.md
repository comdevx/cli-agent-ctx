# Agent Ctx — AI Agent Context Manager CLI
> A DevCool developer tool · devcool.xyz

## Problem
Developers using AI coding agents (Claude Code, Codex, Gemini CLI, Cursor) lose all context every time they start a new session, switch agents, or hand off work to a teammate. Re-explaining project state, decisions, and progress takes 5-10 minutes per session. With context windows filling up and multi-agent workflows becoming the norm, this context loss is the #1 productivity drain for AI-assisted development.

## Target Developer
Individual developers and small-to-medium teams (2-10 people) who use AI coding agents daily. They switch between sessions frequently, sometimes use multiple agents, and collaborate on shared codebases. They already use git and are comfortable with CLI tools.

## Commands/Features

1. **snap** — Capture a context snapshot of the current project state. Reads git status, recent commits, modified files, branch info, and any manual annotations (decisions, notes, current task). Saves to `.agent-ctx/` directory in the project.

2. **load** — Output the latest (or specified) context snapshot in a format that AI agents can consume. Designed to be pasted into a new agent session or piped directly. Supports multiple output formats (markdown, json, plain text).

3. **diff** — Compare two context snapshots side-by-side. Shows what changed: new commits, task progress, new decisions, file changes. Useful for understanding what happened between sessions.

4. **sync** — Export context in agent-specific formats. Converts the universal snapshot into formats optimized for specific agents (Claude Code markdown, Codex instructions, etc.).

5. **decide** — Record an architectural or design decision with rationale. Stored in shared context so all team members and future sessions see it. Decisions persist across snapshots.

6. **config** — Manage Agent Ctx settings: default author name, auto-snap on git commit, output format preferences, ignored paths.

## Success Metrics (30-day targets)
- 500 GitHub stars
- 200 unique installs
- 10 contributors

## Out of Scope (v1)
- Cloud sync / hosted backend (git is the transport layer)
- Real-time collaboration (async via git only)
- GUI / web interface
- Automatic agent detection (manual format selection)
- Plugin system
- Integration with specific IDE extensions
