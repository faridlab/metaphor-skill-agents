# 2 · Planning & Design

**Goal:** turn the shaped concept into a plan you can execute — a written spec, stable interfaces,
module boundaries, and an architecture, with the key decisions recorded.

## Use these

| Name | Kind | What it does | Reach for it when… | Install |
|------|------|--------------|--------------------|---------|
| `planning-and-task-breakdown` | skill | Breaks a task into a plan + ordered steps before coding | The work is more than a small change and needs sequencing | `metaphor agent install planning-and-task-breakdown` |
| `spec-driven-development` | skill | Write the spec first; code follows the spec | You want behavior agreed and written down before implementation | `metaphor agent install spec-driven-development` |
| `api-and-interface-design` | skill | Stable API/interface design; REST/GraphQL/module boundaries | You're shaping a public surface other code will depend on | `metaphor agent install api-and-interface-design` |
| `framework-architect` | skill | System-level architecture decisions, framework evolution | You're making structural choices that are expensive to reverse | `metaphor agent install framework-architect` |
| `modules-orchestrator` | skill | Cross-module coordination and boundaries | The design spans several modules and you must define their seams | `metaphor agent install modules-orchestrator` |
| `cloud-infrastructure-architect` | skill | Multi-cloud design, cost optimization, security | The design needs infrastructure/topology decisions early | `metaphor agent install cloud-infrastructure-architect` |
| `documentation-and-adrs` | skill | Reader-first docs and architecture decision records | You're making a decision worth recording so future-you knows *why* | `metaphor agent install documentation-and-adrs` |
| `framework-handbook` | skill | Authors architecture docs (C4), the "why" of tech choices | You want the architecture written up as part of the design | `metaphor agent install framework-handbook` |
| `technical-writer` | agent | Drives `framework-handbook` to produce architecture/ADR docs | You want the design documented while it's fresh | `metaphor agent install technical-writer` |
| `council` | skill | Multi-persona board that stress-tests an architecture/refactor and returns one least-downside call | You're weighing a structural choice, a refactor, or a new module/framework and want the trade-offs surfaced before you commit | `metaphor agent install council` |

> The `council` skill pulls in the `council-chair`, `council-skeptic`, and `council-steelman` agents
> for its isolated dissenting seats — install them alongside it. It also serves [5 · Review](5-review.md)
> at higher altitude (judging a built unit's maturity); here it's a *design-time* sounding board.

## Recommended flow

1. **Break it down.** `planning-and-task-breakdown` turns the concept into an ordered set of steps
   and identifies the risky ones.
2. **Write the spec.** Use `spec-driven-development` to state the intended behavior before code;
   for anything other code consumes, design the surface with `api-and-interface-design`.
3. **Decide the architecture.** Use `framework-architect` for system-level choices and
   `modules-orchestrator` to set module boundaries; reach for `cloud-infrastructure-architect` when
   topology/infra is in scope. Inspect the existing shape with `metaphor graph` and
   `metaphor show projects`.
4. **Pressure-test the big calls.** For a structural choice that's expensive to reverse, run the
   `council` skill — it adapts a multi-persona roster to the repo type and hands back one
   least-downside recommendation plus the alternatives, instead of a both-sides summary.
5. **Record the decisions.** Capture each significant trade-off as an ADR with
   `documentation-and-adrs`; let the `technical-writer` agent (via `framework-handbook`) write up
   the architecture so it's documented before it drifts.

## Hand-off

Spec, interfaces, and architecture agreed → move to [3 · Development](3-development.md) to build it.
