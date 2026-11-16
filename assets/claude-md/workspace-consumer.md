---
template: workspace-consumer
description: Root CLAUDE.md for a product workspace that consumes Metaphor via remote modules pinned in metaphor.lock.
---

# Metaphor Consumer Workspace

> Type: **consumer workspace** — a product repo that *uses* Metaphor. Upstream framework crates / modules are declared in `metaphor.yaml` (`remote:` + `ref:`), resolved to SHAs in `metaphor.lock`, synced into `modules/` by `metaphor sync`. Apps live under `apps/`.
> This file orients Claude; depth lives in referenced docs — load on demand.

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
- **MUST** treat `modules/` as read-only upstream clones — **NEVER** edit inside `modules/*` (wiped on next sync). Fix upstream, re-pin, re-sync.
- **MUST** pin refs in `metaphor.yaml`; check in `metaphor.lock`; **NEVER** hand-edit the lock — regenerate via `metaphor sync --update` and commit both together.
- **MUST** declare every hand-written, non-schema-derived file under `apps/<app>/metaphor.codegen.yaml` → `user_owned:`, and **NEVER** edit a generated file: hand code lives in a `<name>.ext.rs` extension file beside it (ADR-0031 in metaphora's handbook) — `mod.ext.rs` for a directory, `lib.ext.rs` for the module struct and builder. Modules on the extension layout carry no CUSTOM zones, regeneration writes generated files whole, and `metaphor schema doctor` fails on any generated file that differs from fresh output.
- **NEVER** copy files between apps — promote shared code to an upstream module.
- **MUST** declare every required runtime env var in the matching `.env.*.example` template (the example is the contract; stacks fail-fast on missing vars — no silent defaults).
- **MUST** use `{$VAR}` parse-time placeholders for Caddy site addresses, never `{env.VAR}`.
- **SHOULD** use `metaphor dev` / `metaphor test` / `metaphor build` over raw `cargo` / `gradle`.
- **MUST** run the chaos gate after business-flow-level changes (new/changed flow, cross-module seam, new external dependency, tenancy/RLS/auth, runtime infra) — see `docs/chaos/README.md` where the workspace defines one. Typos, cosmetics, docs-only, tests-only, and behavior-identical refactors are exempt.
- **MUST** read the target repo's own `CLAUDE.md` before working in any repo (each app, each module clone, sibling repos). The more-local `CLAUDE.md` always wins; don't carry assumptions across repo boundaries. Protocol: `docs/agent/cross-repo.md` where present.

## Workspace shape

```
metaphor.yaml / metaphor.lock   # projects[] + resolved SHAs (check in together)
modules/                        # upstream clones — READ-ONLY
apps/                           # one project per app; each carries its own CLAUDE.md
deployment/                     # compose.dev.yaml / compose.yaml, proxy, observability, backups
docs/                           # runbooks; chaos/ and agent/ where the workspace ships them
metaphor.deploy.yaml            # envs, services, image tags
```

## Load on demand — read before the matching task (where the workspace ships them)

| Read | Before |
|---|---|
| `docs/agent/per-app.md` | touching any app (types, stacks, ports, quirks) |
| `docs/agent/tasks.md` | common tasks (update upstream, add app, migrations, deploy) |
| `docs/agent/anti-patterns.md` | writing code/config — the full list with consequences |
| `docs/agent/cross-repo.md` | crossing into the framework tree or sibling product repos |
| `docs/chaos/README.md` | the chaos gate: criteria, catalog, how to run |
| `docs/LOCAL-DEVELOPMENT.md` · `docs/PRODUCTION-DEPLOYMENT.md` · `docs/RELEASE-RUNBOOK.md` | running stacks / shipping |
| `metaphor.deploy.yaml`, `deployment/compose.*.yaml` | deploy / stack work |
| Skill `metaphor-cli-master` · `backbone-cli-master` · `modules-orchestrator` | CLI depth, Backbone workflows, module composition |

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
