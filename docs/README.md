# Which skill/agent, when — a lifecycle guide

This guide maps every bundled **skill** and **agent** onto the stages of building software, so you
can reach for the right one for the goal in front of you — from a half-formed idea to a system you
maintain in production.

## How to use it

1. Find the phase you're in below.
2. Open that phase's file — it lists the skills/agents for the job, what each is for, and when to
   reach for it.
3. Install the ones you need:

```bash
metaphor agent install <name>            # into this project's ./.claude/
metaphor agent install <name> --global   # into ~/.claude/ (every project)
metaphor agent install --all             # everything (38 skills + 13 agents)
```

New to skills? Install `using-agent-skills` first — it's the meta-skill for invoking and composing
the rest.

> **Skills vs agents.** A *skill* is a knowledge base Claude loads on demand to think well about a
> domain. An *agent* is a subagent you delegate a scoped job to (it has its own tools/model). Many
> phases pair them: a skill teaches the approach, an agent does the work.

## Lifecycle map

| Phase | Goal | Primary skills | Primary agents | Key `metaphor` commands |
|-------|------|----------------|----------------|-------------------------|
| [1 · Ideation](1-ideation.md) | Turn an idea into a shaped, modeled concept | `idea-refine`, `domain-specific-expert`, `creative-domain-architect`, `business-flow-bdd` | `business-flow` | `metaphor info` |
| [2 · Planning & Design](2-planning-and-design.md) | Decide the spec, interfaces, and architecture | `planning-and-task-breakdown`, `spec-driven-development`, `api-and-interface-design`, `framework-architect`, `modules-orchestrator`, `cloud-infrastructure-architect`, `documentation-and-adrs`, `council` | `technical-writer`, `council-chair`, `council-skeptic`, `council-steelman` | `metaphor graph`, `metaphor show projects` |
| [3 · Development](3-development.md) | Build it in small, verified steps | `metaphor-cli-master`, `backbone-schema-maintainer`, `custom-logic-specialist`, `incremental-implementation`, `source-driven-development`, `test-driven-development`, `code-simplification`, `frontend-ui-engineering`, `crate-maintainer`, `apps-maintainer`, `database-migration-specialist`, `backbone-cli-master` | `refactorer` | `metaphor build`, `metaphor dev serve`, `metaphor make` |
| [4 · Testing](4-testing.md) | Prove it works and stays fast | `test-driven-development`, `business-flow-bdd`, `browser-testing-with-devtools`, `debugging-and-error-recovery`, `performance-optimization` | `test-writer`, `business-flow`, `debugger`, `perf-analyzer` | `metaphor test --affected` |
| [5 · Review](5-review.md) | Catch defects before merge | `code-review-and-quality`, `security-and-hardening`, `code-simplification`, `council` | `code-reviewer`, `security-auditor`, `perf-analyzer`, `council-chair`, `council-skeptic`, `council-steelman` | `metaphor lint check` |
| [6 · Documentation](6-documentation.md) | Make it understandable to others | `framework-handbook`, `documentation-and-adrs`, `domain-specific-expert`, `business-flow-bdd` | `technical-writer`, `doc-writer`, `onboarding-explainer` | `metaphor docs generate` |
| [7 · Deployment](7-deployment.md) | Ship it safely and repeatably | `shipping-and-launch`, `ci-cd-and-automation`, `cloud-infrastructure-architect` | — | `metaphor compose generate`, `metaphor build --all` |
| [8 · Maintenance](8-maintenance.md) | Keep it healthy and evolving | `deprecation-and-migration`, `database-migration-specialist`, `crate-maintainer`, `apps-maintainer`, `performance-optimization`, `security-and-hardening`, `debugging-and-error-recovery` | `debugger`, `perf-analyzer`, `security-auditor`, `refactorer`, `onboarding-explainer` | `metaphor doctor`, `metaphor test --affected --base=main` |

## Cross-cutting tools

These serve **every** phase — install them once and lean on them throughout rather than tying them
to a single stage:

| Tool | Kind | Why it's everywhere |
|------|------|---------------------|
| `using-agent-skills` | skill | How to invoke and compose all the others. Start here. |
| `context-engineering` | skill | Designing what Claude sees — useful any time you wire an AI into a surface. |
| `metaphor-cli-master` | skill | `metaphor` is the entry point for build/test/dev/deploy across every phase. |
| `git-workflow-and-versioning` | skill | Branching and versioning underlie development through release. |
| `grouped-commits` / `commit-generator` | skills | Committing well happens continuously. `grouped-commits` splits by functionality with why-focused messages; `commit-generator` does conventional-commit formatting + changelog automation. |
| CLAUDE.md templates | orientation | `metaphor agent claude init` writes per-project-type `CLAUDE.md` so Claude knows what kind of project it's in on every turn. See the [root README](../README.md#claudemd-orientation-per-project-type). |

## The whole flow, in one breath

Refine the idea (`idea-refine`) and model its domain + business flows
(`creative-domain-architect`, `business-flow-bdd`) → break it down and design the spec, APIs, and
architecture (`planning-and-task-breakdown`, `spec-driven-development`, `api-and-interface-design`,
`framework-architect`) → build incrementally against the schema SSoT (`incremental-implementation`,
`backbone-schema-maintainer`, `custom-logic-specialist`) → test and debug (`test-writer`,
`business-flow`, `debugger`, `perf-analyzer`) → review for quality and security (`code-reviewer`,
`security-auditor`) → document it (`technical-writer`, `doc-writer`) → ship it
(`shipping-and-launch`, `ci-cd-and-automation`) → and keep it healthy (`deprecation-and-migration`,
`apps-maintainer`, and the debug/perf/security agents on call).
