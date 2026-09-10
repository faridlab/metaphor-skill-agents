---
name: chaos-engineering
description: Run the post-change chaos gate on a Metaphor workspace's dev stack — automated, repeatable fault experiments (postgres/minio loss, service kill mid-flow, pool exhaustion, secret fail-closed, maintenance gate, tenancy fence) that prove the steady state holds while a dependency fails and the stack recovers cleanly. Use whenever a business-flow-level change lands (new/changed flow, cross-module seam, new external dependency, tenancy/RLS/auth work, runtime infrastructure), when the user asks to "run the chaos gate", stress resilience, or verify recovery behavior, or when wiring a workspace's steady-state hook / adding a chaos experiment. Also covers reading verdicts, filing failures, and the safety rules (dev stack only, volumes untouchable, never run while someone else is using the stack).
---

# Chaos engineering — the post-change resilience gate

Chaos here is a **quality gate**, like tests and review: after a business-flow-level
change, fault experiments against the **dev stack** must prove the steady state
holds while a dependency fails — and that the stack recovers cleanly once the
fault is withdrawn. It is not ad-hoc breakage, not a load test, and never a
substitute for tests.

The workspace's own `docs/chaos/README.md` is the authority for its practice
(catalog, knobs, steady state); this skill is the workflow discipline. The kit
lives at `deployment/chaos/` (bash + docker compose, no host dependencies).

## 1. When the gate is mandatory

Run it when the change does ANY of:

- adds/changes a **business flow** (route + service + entity forming an
  end-user flow),
- adds/changes a **cross-module seam** (module A now calls/expects module B),
- adds a **new external dependency** (queue, cache, storage, provider),
- touches **tenancy / RLS / auth / fences**,
- touches **runtime infrastructure** (pools, workers, deploy stack, proxy,
  migration runner).

**Exempt** (running chaos on these wastes the stack): typos, copy, formatting,
comments, docs-only, tests-only, behavior-identical refactors.

## 2. Safety preconditions — non-negotiable

1. **Dev stack only.** Never the production stack.
2. **Never while someone else is using the stack.** Experiments restart
   services; a mid-flight colleague loses their state. If there is any doubt
   whether the stack is in use — ask before running.
3. **One fault at a time.** The driver is serial; never parallelize it.
4. **Volumes and caches are untouchable.** Experiments only stop/start/recreate
   services — never delete volumes, never `compose down -v`, never touch build
   caches.
5. Every experiment arms an **abort trap** that restores the stack on any exit —
   but prefer clean phases; the trap is the net, not the plan.
6. Secrets are read per-variable from the env file; never echoed.

## 3. Running the gate

```bash
# preferred: the dev plugin's forwarder (same args either way)
metaphor chaos list
metaphor chaos --dry-run all          # print every action + check, touch NOTHING
metaphor chaos postgres-loss          # one experiment, live
metaphor chaos --full all             # + business-flow canary / tenancy legs (slow)

# direct (workspaces whose plugin predates the forwarder)
bash deployment/chaos/run.sh list
bash deployment/chaos/run.sh --dry-run all
bash deployment/chaos/run.sh postgres-loss
```

Discipline:

1. **`list`** to see the catalog; **`--dry-run`** the target first when the
   stack or the kit is unfamiliar — the dry-run prints the full plan and never
   writes runlogs.
2. Run the **targeted experiment(s)** that match the change (a tenancy change →
   `tenancy-fence-under-fault`; a boot/env change → `secret-fail-closed`,
   `maintenance-mode`; a dependency change → the matching `-loss` row). `all`
   is for full gates and release trains.
3. `--full` adds the business-flow canary and tenancy-sweep legs — it needs the
   workspace's steady-state hook (§5) and takes much longer.
4. Each experiment runs baseline → inject → observe → restore → recovery. The
   **baseline must be green** before anything is injected; a red baseline
   aborts without injecting (fix the stack first).

## 4. Reading the verdict

Per experiment: `VERDICT: PASS|FAIL — <name>` plus
`rc=… passed=… failed=… skipped=…`; the run ends with `chaos_exit=0|1`.

- **PASS** → the resilience assumption is proven for this change. The runlog
  (`deployment/chaos/runlogs/YYYY-MM-DD-<experiment>.log`) is the evidence —
  it is written to be committed; include it with the change.
- **FAIL** → a defect, not a flake. File an issue in the workspace's tracker
  referencing the runlog, and treat it with normal prioritization. Do not
  "fix" the experiment to make it pass; if the expectation was genuinely
  wrong, say so in the issue and change the hypothesis, not the observation.
- **SKIP** (rc=2) → honest absence, never coverage: the target service is not
  in this compose (e.g. no control plane), a knob is unset
  (`CHAOS_SECRET_ENV_VAR`), or the steady-state hook is not installed. Report
  skips as skips; decide deliberately whether to wire the missing piece.

## 5. Wiring the steady-state hook (turn skips into signal)

A fresh workspace ships health-only steady state (`/livez` + `/readyz`) —
shallower but honest. To make the gate judge the product's real flows:

1. Copy `deployment/chaos/lib/steady-state.sh.example` →
   `deployment/chaos/lib/steady-state.sh` and commit it — it is product code.
2. Implement the hook functions as **thin wrappers over committed probe
   scripts in the app repo** (`flow_canary LABEL`, `tenancy_sweep`; both
   exit 0 = pass). The kit must never own product probe logic.
3. The hook file must only define functions — it is sourced at kit startup.
4. Re-run: `--full` legs now exercise the canary (foreground as the verdict
   leg, background during midwrite/kill faults) and the sweep at tenancy
   recovery.

## 6. Adding an experiment

Copy the closest existing experiment and keep the shape:

- Metadata header: fault, hypothesis (one sentence, judged not vibes), blast
  radius (smallest that still teaches), restore.
- Phases: `inject` (call `mark_injected` the moment the fault is live — it
  arms the abort trap), optional `during_fault`, `restore` (must undo the
  injection and wait for readiness), `recovery_checks`.
- Checks go through the `check_*` ledger (`check_eq` / `check_in` /
  `check_contains` / `check_service_running` / `check_cmd`) so verdicts parse;
  every HTTP probe uses `--max-time` (completing at all proves no hang) and
  targets `127.0.0.1` explicitly, never `localhost`.
- Recreate-class faults (compose override + `up -d`) are SLOW and restart the
  dev service — the header must say so, and §2 applies doubly.
- Add the experiment to the catalog in `run.sh` and the table in the
  workspace's `docs/chaos/README.md`.

## 7. What the kit assumes

Single-service dev compose (`<product>-service` on 127.0.0.1:3000, optional
control plane on :3210; postgres/redis/minio), `deployment/.env.dev` optional,
bash 3.2 compatible. Rebind names in `deployment/chaos/lib/common.sh` — it is
the single wiring point (`SERVICE_NAME`, `CONTROL_PLANE_SERVICE`, base URLs,
timeouts, knobs).
