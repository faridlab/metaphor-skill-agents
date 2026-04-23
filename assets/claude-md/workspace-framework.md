---
template: workspace-framework
description: Root CLAUDE.md for a metaphora-style framework workspace (builds Metaphor itself).
---

# Metaphor Framework Workspace

> Type: **framework workspace** — the repo that *builds* Metaphor (CLI, plugins, framework crates, domain modules, skeletons).
> This file orients Claude. Skills carry depth; load them on demand.

## What this is

A multi-project meta-workspace orchestrated by `metaphor.yaml` at this root. Every subdirectory listed in that manifest is an **independent project** with its own `Cargo.toml` / `package.json` / build. `metaphor` is the canonical entry point — prefer it over raw `cargo`/`npm`/`docker` whenever a matching subcommand exists.

## Golden path

```bash
metaphor info                    # which project am I in?
metaphor doctor                  # tooling + plugin health
metaphor list                    # enumerate projects
metaphor graph                   # dependency graph
```

## Rules

- **MUST** read `metaphor.yaml` before planning any cross-project change.
- **MUST** use `metaphor <cmd>` when a matching subcommand exists (`metaphor build`, `metaphor test`, `metaphor dev serve`, `metaphor agent ...`).
- **NEVER** run `cargo build` / `cargo test` from the workspace root — each project has its own `Cargo.toml`; use `metaphor build [--all]` / `metaphor test [--all] [--affected]`.
- **NEVER** hand-copy files into `.claude/skills/` — use `metaphor agent install <name>` (never lose attribution or updates).
- **NEVER** hardcode project paths in scripts — read via `metaphor show projects --json`.
- **SHOULD** use `--affected --base=main` in CI to only rebuild/retest what changed.
- Commits: conventional commits; **no Claude signatures** in commit messages.

## Project types in this workspace

| Type | Meaning | Per-type CLAUDE.md |
|------|---------|--------------------|
| `crate` | Focused Rust library (one crate, one concern) | `./CLAUDE.md` in that project |
| `backend-service` | Runnable HTTP/gRPC server (Axum + SQLx + Tokio) | `./CLAUDE.md` in that project |
| `module` | Domain module library (DDD 4-layer; schema YAML is SSoT) | `./CLAUDE.md` in that project |
| `cli-tool` | Subcommand dispatcher with plugin discovery | `./CLAUDE.md` in that project |
| `mobileapp` | Kotlin Multiplatform + Compose + Koin + offline-first | `./CLAUDE.md` in that project |

Before editing inside any project, read its `CLAUDE.md`.

## Plugins (separate binaries, subprocess-dispatched)

```
metaphor-codegen  → make, module, apps, proto, migration, seed
metaphor-schema   → schema, webapp
metaphor-dev      → dev, lint, test, docs, config, jobs
metaphor-agent    → agent (Claude Code skills + subagents + CLAUDE.md)
```

Discovery order: `$PATH` → `$METAPHOR_PLUGIN_BIN_DIR` → `~/.metaphor/bin/`.

## Common tasks

- "Run tests on changed projects" → `metaphor test --affected --parallel 4`
- "Lint all projects" → `metaphor lint check --all`
- "Generate cross-project docker-compose" → `metaphor compose generate --write`
- "Bootstrap Claude Code in every project" → `metaphor agent claude init` (this workspace + per-type CLAUDE.md everywhere)
- "Install skills globally" → `metaphor agent install --all --global`

## Key files to read before editing

- `metaphor.yaml` — authoritative project inventory and types.
- `metaphor-cli/docs/cli-reference.md` — full command surface.
- `metaphor-cli/docs/architecture.md` — why plugins are subprocess-dispatched.
- `metaphor-skill-agents/manifest.yaml` — what skills/agents/CLAUDE.md templates are shipped.

## Deeper knowledge (load on demand)

- Skill: `metaphor-cli-master` — complete CLI surface, plugins, workflows.
- Skill: `backbone-cli-master` — Backbone-specific workflows.
- Skill: `planning-and-task-breakdown` — before big cross-project changes.
- Skill: `context-engineering` — when wiring Claude into new surfaces.

## Anti-patterns

- Mixing `--all` with `--projects` (they conflict — pick one).
- Re-running failures with `--parallel` hoping they pass (parallelism never fixes a real failure).
- Editing another project's files from inside a sibling project (use `metaphor sync` to refresh).
- Bypassing `metaphor agent install` to copy skills manually.
