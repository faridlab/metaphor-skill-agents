# 1 · Ideation

**Goal:** turn a half-formed idea into a shaped concept — clear problem, a domain model with
agreed vocabulary, and the business flows the thing must support — before any code or detailed
design.

## Use these

| Name | Kind | What it does | Reach for it when… | Install |
|------|------|--------------|--------------------|---------|
| `idea-refine` | skill | Refines a vague idea into a concrete, actionable plan | The idea is still fuzzy and you need to sharpen scope and intent | `metaphor agent install idea-refine` |
| `domain-specific-expert` | skill | Transfers and documents domain knowledge | You're entering an unfamiliar problem domain and need to capture its rules | `metaphor agent install domain-specific-expert` |
| `creative-domain-architect` | skill | DDD domain modeling, ubiquitous language, reusable patterns | You're naming the core concepts and drawing aggregate/entity boundaries | `metaphor agent install creative-domain-architect` |
| `business-flow-bdd` | skill | Models each feature's business flow (actors, paths, rules, postconditions) | You want to pin down *what the feature does for the business* up front | `metaphor agent install business-flow-bdd` |
| `business-flow` | agent | Drives `business-flow-bdd` — produces flow docs and Gherkin scenarios | You want a first draft of the flows and acceptance scenarios written for you | `metaphor agent install business-flow` |
| `context-engineering` ✱ | skill | Designing context windows/prompts for coding agents | The idea itself involves wiring an AI/agent into a product surface | `metaphor agent install context-engineering` |

✱ cross-cutting — see the [index](README.md#cross-cutting-tools).

## Recommended flow

1. **Sharpen the idea.** Use `idea-refine` to convert the rough notion into a concrete plan:
   problem statement, who it's for, what's in and out of scope.
2. **Learn the domain.** If the space is unfamiliar, use `domain-specific-expert` to capture the
   rules, terms, and constraints that the design must respect.
3. **Model it.** With `creative-domain-architect`, name the core concepts and establish the
   ubiquitous language — the vocabulary every later doc, schema, and test will reuse.
4. **Capture the business flows.** Use `business-flow-bdd` (or hand it to the `business-flow`
   agent) to write each feature's flow and a first set of Given/When/Then scenarios. These become
   acceptance tests later — defining them now keeps design honest.
5. **Orient.** Run `metaphor info` to confirm which workspace/project you're in before the idea
   turns into project structure.

## Hand-off

Concept shaped and flows captured → move to [2 · Planning & Design](2-planning-and-design.md) to
turn it into a spec, interfaces, and architecture.
