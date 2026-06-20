# 4 · Testing

**Goal:** prove the feature does what it should — at the unit, business-flow, and UI levels —
diagnose what doesn't, and confirm it stays fast.

## Use these

| Name | Kind | What it does | Reach for it when… | Install |
|------|------|--------------|--------------------|---------|
| `test-driven-development` | skill | Red/green/refactor; tests pin behavior | You want tests to capture each behavior precisely | `metaphor agent install test-driven-development` |
| `test-writer` | agent | Writes tests matching the project's existing conventions | You need unit/integration tests authored for a target | `metaphor agent install test-writer` |
| `business-flow-bdd` | skill | Turns business flows into Gherkin acceptance specs | You want feature behavior verified at the business level | `metaphor agent install business-flow-bdd` |
| `business-flow` | agent | Authors `.feature` scenarios and runs them as acceptance tests | You want the flows from ideation wired up and executed | `metaphor agent install business-flow` |
| `browser-testing-with-devtools` | skill | Browser-based testing using DevTools | The behavior lives in a browser/UI | `metaphor agent install browser-testing-with-devtools` |
| `debugging-and-error-recovery` | skill | Root-cause analysis, reproduction, recovery | A test fails and you need to find out *why* | `metaphor agent install debugging-and-error-recovery` |
| `debugger` | agent | Root-causes failures with reproduction, proof, and a verified fix | You want a failure diagnosed and fixed end-to-end | `metaphor agent install debugger` |
| `performance-optimization` | skill | Measure-first perf improvements; algorithmic over micro | Something is slow and you must improve it without guessing | `metaphor agent install performance-optimization` |
| `perf-analyzer` | agent | Hot-path and allocation analysis, impact-ranked fixes | You need the slow paths found and ranked | `metaphor agent install perf-analyzer` |

## Recommended flow

1. **Cover behavior.** Drive units with `test-driven-development`; hand the `test-writer` agent a
   target to fill coverage gaps in the project's existing style.
2. **Verify business flows.** Use `business-flow` (over `business-flow-bdd`) to turn the flows from
   [ideation](1-ideation.md) into runnable Given/When/Then acceptance specs.
3. **Test the UI** with `browser-testing-with-devtools` when behavior is browser-side.
4. **Run them** with `metaphor test --affected` to retest only what changed.
5. **Diagnose failures.** Use the `debugger` agent (backed by `debugging-and-error-recovery`) to
   reproduce, prove the root cause, and verify the fix — never paper over it.
6. **Check speed.** Run `perf-analyzer` (backed by `performance-optimization`) on hot paths;
   measure before and after.

## Hand-off

Green and fast → move to [5 · Review](5-review.md) before merging.
