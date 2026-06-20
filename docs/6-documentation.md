# 6 · Documentation

**Goal:** make the work understandable to everyone who comes after — evaluators, app developers,
maintainers, and contributors — from the framework handbook down to a single API doc.

## Use these

| Name | Kind | What it does | Reach for it when… | Install |
|------|------|--------------|--------------------|---------|
| `framework-handbook` | skill | The full handbook: philosophy, architecture, maintainer + developer guides, ADRs | You're producing or restructuring the project-level doc set | `metaphor agent install framework-handbook` |
| `technical-writer` | agent | Drives `framework-handbook` via a writer ↔ technical-manager review loop | You want the whole handbook authored at quality | `metaphor agent install technical-writer` |
| `documentation-and-adrs` | skill | Reader-first docs and architecture decision records | You're writing docs or recording a decision | `metaphor agent install documentation-and-adrs` |
| `doc-writer` | agent | README, API docs, and inline comments that actually help | You need a single README/API surface documented | `metaphor agent install doc-writer` |
| `domain-specific-expert` | skill | Domain knowledge transfer and documentation | You're capturing domain rules for future readers | `metaphor agent install domain-specific-expert` |
| `onboarding-explainer` | agent | Explains a module/service to a new engineer, outside-in | Someone needs to come up to speed on existing code | `metaphor agent install onboarding-explainer` |
| `business-flow-bdd` | skill | Business-flow docs that double as acceptance specs | You want feature behavior documented as living flows | `metaphor agent install business-flow-bdd` |

## Choosing the right writer

- **Whole handbook** (philosophy → architecture → maintainer/developer guides → ADRs) →
  `technical-writer` + `framework-handbook`.
- **One README / API reference / inline comments** → `doc-writer`. (The handbook agent
  deliberately delegates single-file work here.)
- **Explaining existing code to a person** → `onboarding-explainer`.
- **Feature behavior as living documentation** → `business-flow-bdd`.

## Recommended flow

1. **Handbook first** for new or under-documented projects: let `technical-writer` build the doc
   set from the sources of truth, using the writer ↔ manager review loop.
2. **Fill the surface.** Use `doc-writer` for the README, API reference, and inline comments; keep
   ADRs current with `documentation-and-adrs`.
3. **Capture the domain and flows** with `domain-specific-expert` and `business-flow-bdd` so the
   behavior is documented, not just the code.
4. **Generate and check.** Run `metaphor docs generate` (rustdoc + coverage check) to keep the
   generated reference honest.

## Hand-off

Documented → move to [7 · Deployment](7-deployment.md).
