---
template: workspace-consumer
description: Root CLAUDE.md for a product workspace that consumes Metaphor via remote modules pinned in metaphor.lock.
---

# Metaphor Consumer Workspace

> Type: **consumer workspace** — a product repo that *uses* Metaphor. Upstream framework crates / modules are pulled in as pinned git dependencies via `metaphor.yaml` + `metaphor.lock`.
> This file orients Claude. Skills carry depth; load them on demand.

## What this is

A multi-app product. Upstream dependencies (e.g. `backbone-framework`, `backbone-<domain>`, `pheromone`) are declared in `metaphor.yaml` with `remote:` + `ref:`, resolved to commit SHAs in `metaphor.lock`, and synced into `modules/` on disk by `metaphor sync`. Apps live under `apps/` and consume modules via path or git dependency.

## Golden path

```bash
metaphor doctor                  # tooling + upstream health
metaphor sync                    # clone/update remote modules to pinned refs
metaphor info                    # where am I?
metaphor migration run-all       # bring DBs up to date
metaphor dev serve               # run the current app
```

## Rules

- **MUST** run `metaphor sync` after pulling changes that touch `metaphor.yaml` or `metaphor.lock`.
- **MUST** treat `modules/` as read-only clones of upstream repos — **NEVER** edit files inside `modules/*`. Fix upstream and re-sync.
- **MUST** pin versions in `metaphor.yaml` (`ref: v1.2.0` or commit SHA) for reproducibility.
- **MUST** check `metaphor.lock` into git.
- **NEVER** edit `metaphor.lock` by hand — regenerate via `metaphor sync --update`.
- **NEVER** copy files between apps — if they share code, promote to an upstream module instead.
- **SHOULD** use `metaphor dev` / `metaphor test` / `metaphor build` over raw `cargo` / `gradle`.

## Workspace shape

```
./
├── metaphor.yaml          # projects[]: apps + remote modules
├── metaphor.lock          # resolved commit SHAs (check in)
├── modules/               # ← upstream clones (read-only; managed by sync)
│   └── <backbone-*>/
├── apps/                  # ← your product code
│   └── <app-name>/        # each has its own CLAUDE.md
└── deployment/            # docker-compose, k8s, CI (if present)
```

## `metaphor.yaml` for consumers

```yaml
version: 1
projects:
  - name: backbone-framework
    type: crate
    path: ./modules/backbone-framework
    remote: https://github.com/faridlab/backbone-framework
    ref: main                              # tag/branch/SHA

  - name: my-service
    type: backend-service
    path: ./apps/my-service
    depends_on: [backbone-framework]

  - name: my-mobile
    type: mobileapp
    path: ./apps/my-mobile
```

Presence of any `remote:` entry = this is a **consumer workspace** (as opposed to a framework workspace that defines upstream).

## Per-app orientation

Each app inside `apps/` has its own `CLAUDE.md` matching its project type. Read that before editing the app:

- `backend-service` → Axum + SQLx + modules composition
- `mobileapp` → Kotlin Multiplatform + Compose + offline-first sync
- `webapp` → (add when present)

## Common tasks

- "Update to newer upstream" → edit `ref:` in `metaphor.yaml`, then `metaphor sync --update && metaphor migration run-all && metaphor test --affected`.
- "Add a new app" → `metaphor apps create <name> --type <backend-service|mobileapp|...>` (wires into `metaphor.yaml` automatically).
- "Run migrations across all DBs" → `metaphor migration run-all`.
- "Test only what I changed" → `metaphor test --affected --base=main`.
- "Refresh Claude Code setup" → `metaphor agent claude update` (re-apply CLAUDE.md templates).

## Key files to read before editing

- `metaphor.yaml` — what upstream we track, what apps exist.
- `metaphor.lock` — the SHAs actually in use.
- `apps/<current-app>/CLAUDE.md` — per-app rules.
- `deployment/` — how the product ships (if present).

## Deeper knowledge (load on demand)

- Skill: `metaphor-cli-master` — full CLI surface.
- Skill: `backbone-cli-master` — Backbone-specific workflows.
- Skill: `backbone-modules-orchestrator` — composing modules into a service.
- Skill: `source-driven-development` — staying aligned with upstream source of truth.

## Anti-patterns

- Editing `modules/*` in-place (changes are wiped on next `sync`; fix upstream).
- Committing `metaphor.lock` changes without also updating `metaphor.yaml` (lock drift).
- Copy-paste between apps (creates drift; extract to a shared module instead).
- Running `cargo build` inside `modules/*` (may pollute upstream working tree — use `metaphor build` which builds in the right place).
