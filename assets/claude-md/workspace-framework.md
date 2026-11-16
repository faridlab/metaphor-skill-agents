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
- **MUST** read and follow the target repo's own `CLAUDE.md` when working across repos — before editing in another repo, read its rules; the more local `CLAUDE.md` always wins.

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

<!-- rust-rules:start (one identical copy in every Rust repo; full text: metaphora docs/handbook/rust-rules.md) -->
## Rust rules (NS01 to NS12)

Every Rust change in this repo follows the twelve Rust rules. The full text, with the reason and
the check for each rule, is the metaphora handbook page
[`docs/handbook/rust-rules.md`](https://github.com/faridlab/metaphora/blob/main/docs/handbook/rust-rules.md).
Cite the code (for example `NS04`) in reviews and commits.

- **NS01 Types carry the meaning:** newtypes and enums for ids, money and states; parse input once, at the boundary.
- **NS02 Never panic on input:** no `unwrap`, `expect`, indexing or unchecked arithmetic on request, file, env, row or message data; return a typed error.
- **NS03 Errors are typed, never swallowed:** `thiserror` in libraries, `anyhow` only in binaries; no `let _ =`, `.ok()` or `unwrap_or_default()` that hides a failure.
- **NS04 Ownership is designed, not appeased:** never add `clone`, `to_vec`, `to_owned`, `Rc`/`Arc`, `RefCell`/`Cell`, `Box`, indices, a split, a lifetime or `unsafe` only to make it compile. `Arc` for state shared across tasks is design, not appeasement.
- **NS05 Unsafe stays in one place:** `#![forbid(unsafe_code)]` by default; any exception lives in one module with `// SAFETY:` on every block.
- **NS06 Pure core, effects at the edge:** domain logic does no database, network, clock or env access; the edge passes those in.
- **NS07 Structure:** one concept per module, private or `pub(crate)` by default, dependencies point inward, generated code and hand code (`*.ext.rs`) kept apart.
- **NS08 Comments and naming:** domain names, comments say why, every public item documented, no session jargon.
- **NS09 Reuse before writing:** search std, current dependencies and the framework first; promote shared code upstream, never copy it.
- **NS10 Dependencies:** each new crate justified, unused default features off, `cargo deny` clean.
- **NS11 Proof, tests, speed:** a fix starts with a test that fails on the old code; a speed claim comes with a measurement.
- **NS12 The CI gate:** `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings -W clippy::pedantic`, `cargo test` and `cargo deny check` all pass; every `#[allow]` says why. Adopted repo by repo: until this repo runs it, reviewers enforce it by hand.
<!-- rust-rules:end -->
