# 3 · Development

**Goal:** build the thing in small, verified steps — against the schema source-of-truth, inside
the established module boundaries, committed cleanly as you go.

## Use these

| Name | Kind | What it does | Reach for it when… | Install |
|------|------|--------------|--------------------|---------|
| `metaphor-cli-master` ✱ | skill | Treat `metaphor` as the first-class build/dev entry point | Always, inside a Metaphor workspace | `metaphor agent install metaphor-cli-master` |
| `backbone-cli-master` | skill | Master 20+ Backbone CLI commands and workflows | You're building on the Backbone framework | `metaphor agent install backbone-cli-master` |
| `backbone-schema-maintainer` | skill | Backbone DSL + 20+ generators, DDD + Clean Arch | You're changing a module's schema YAML (the SSoT) | `metaphor agent install backbone-schema-maintainer` |
| `custom-logic-specialist` | skill | The `// <<< CUSTOM` pattern; regeneration-safe edits | You're adding hand-written logic to generated code | `metaphor agent install custom-logic-specialist` |
| `incremental-implementation` | skill | Ship in small verified steps, not big-bang changes | Any non-trivial build — keep each step provable | `metaphor agent install incremental-implementation` |
| `source-driven-development` | skill | Work from the source of truth; avoid drifting copies | You might otherwise hand-edit generated/derived artifacts | `metaphor agent install source-driven-development` |
| `test-driven-development` | skill | Red/green/refactor loop | You want tests to drive the implementation | `metaphor agent install test-driven-development` |
| `code-simplification` | skill | Reduce complexity, remove dead code, collapse abstractions | A change is getting tangled and needs simplifying | `metaphor agent install code-simplification` |
| `frontend-ui-engineering` | skill | Frontend/UI engineering patterns | You're building the UI layer | `metaphor agent install frontend-ui-engineering` |
| `crate-maintainer` | skill | Shared crate versioning and integration | You're working inside a `crate` project | `metaphor agent install crate-maintainer` |
| `apps-maintainer` | skill | App lifecycle and module integration | You're wiring modules into a runnable app | `metaphor agent install apps-maintainer` |
| `database-migration-specialist` | skill | PostgreSQL migrations via Backbone tooling | The change needs a schema/data migration | `metaphor agent install database-migration-specialist` |
| `refactorer` | agent | Targeted, behavior-preserving refactors in small steps | You need to restructure code without changing behavior | `metaphor agent install refactorer` |

Cross-cutting here too: `using-agent-skills`, `git-workflow-and-versioning`, and
`grouped-commits` / `commit-generator` for committing as you build. See the
[index](README.md#cross-cutting-tools).

## Recommended flow

1. **Start from the SSoT.** If a module is involved, change the **schema YAML** with
   `backbone-schema-maintainer`, then regenerate via `metaphor make` / the relevant codegen
   command — don't hand-edit generated files (`source-driven-development`).
2. **Add custom logic safely.** Put hand-written code only inside `// <<< CUSTOM` markers using
   `custom-logic-specialist`, so the next regeneration doesn't wipe it.
3. **Build in small steps.** Follow `incremental-implementation` (and `test-driven-development`
   when tests should lead). Run `metaphor build` and `metaphor dev serve` to keep a tight loop.
4. **Keep it clean.** Use `code-simplification` and the `refactorer` agent to keep each step
   tidy; commit each finished unit with `grouped-commits` (why-focused, grouped by functionality).
5. **Project-type specifics.** Lean on `crate-maintainer`, `apps-maintainer`,
   `frontend-ui-engineering`, or `database-migration-specialist` depending on what you're building.

## Hand-off

Feature built and committed → move to [4 · Testing](4-testing.md) to prove it works.
