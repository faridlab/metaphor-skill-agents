# Changelog

All notable changes to `metaphor-skill-agents` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0]

### Added
- **`orchestrate-agents` skill** (`generic/`) — the multi-agent fleet pattern for large tasks: a
  project-manager agent runs first (priority order across tracks, a GO/NO-GO gate per track,
  guidance strings handed to the workers), the worker tracks then run as concurrent pipelines
  (spec→implementation chains, live verification legs, read-only recons — gated tracks are skipped
  by the script, not by judgment), and a steelman→skeptic→chair council closes the run against the
  workers' actual reports. The skill carries the construction rules (structured output schemas on
  every agent, gates as data, pipeline over barrier), the house-rules template every agent prompt
  must embed (read the repo's CLAUDE.md first; workers never commit, tag, or push — the
  orchestrator serializes review and keeps the user in the loop for anything outbound), and the
  orchestration discipline for the main conversation (background launch, no polling,
  journal inspection on suspicious results, never fabricating a pending agent's output). Catalog
  totals are now 38 skills and 13 subagents (docs already carried 38; the on-disk count catches up).

## [0.3.4]

### Changed
- **`grouped-commits` skill now requires self-contained, timeless messages.** Commit messages must
  decode on their own for a reader with no access to the planning context that produced them — no
  sprint/wave/pass labels, internal spec IDs, meeting references, or scorekeeping counts. The
  skill's description, message rules, and anti-patterns all carry the rule with good/bad examples,
  and the manifest summary reflects it. Rationale: messages are read years later by engineers and
  sessions that were not in the room; vocabulary defined only inside a planning document goes stale
  the moment that document's window closes.

## [0.3.3]

### Added
- **Ten council persona files** (`council/personas/`) — `dx-ergonomics`, `framework-author`,
  `api-extensibility`, `versioning-discipline`, `ddd-bounded-context`, `contract-seat`,
  `operational-realist`, `tester`, `ux`, and `devopssec`. These complete the persona registry: every
  context seat and conditional guest that `composition.yaml` wires for the `cli`, `framework`, `module`,
  and `service` repo types now ships as a file. Together with the `root`-context seats added in 0.3.2,
  the council now seats *every* roster from shipped persona files instead of regenerating any seat from
  the schema at run time. Each follows the genotype contract in `references/persona-schema.md`
  (lineage / lens / primary_attack / summon / engagement). Catalog totals are unchanged at 38 skills and
  13 subagents — these are seats within the existing `council` skill.

## [0.3.2]

### Added
- **Four council persona files** (`council/personas/`) — `systems-interaction`, `platform-engineer`,
  `strategist`, and the invited `domain-expert` seat. These materialize the `root`-context roster (and
  the `domain-expert` guest invited at the root/module/service levels) that `composition.yaml` already
  referenced, so the common "run the council at the metaphor root" path now seats from shipped persona
  files instead of regenerating them from the schema on every run. Each follows the genotype contract in
  `references/persona-schema.md` (lineage / lens / primary_attack / summon / engagement). Catalog totals
  are unchanged at 38 skills and 13 subagents — these are seats within the existing `council` skill.

## [0.3.0]

### Added
- **`council` skill** (`metaphor/`) — a context-aware, multi-persona architecture review board. Reads
  `metaphor.toml` to resolve the repo type (cli/framework/module/service/root) and scope, then
  auto-composes a roster (standing seats + context seats + conditionally invited guests) bounded to a
  single focus lens. Runs the dissenting seats as isolated subagents and ends with the Chair's one
  least-downside recommendation plus ranked alternatives — never a both-sides summary. Persists each
  run's report verbatim to `docs/council/<date>-<repo-type>-<unit>-<focus>.md` so the decision and the
  conditions it was made under survive the session. Ships the persona registry (`personas/*.yaml`), the
  roster rules (`composition.yaml`), the report contract and persona schema (`references/`), and a
  `metaphor.toml.example`.
- **`council-chair`, `council-skeptic`, `council-steelman` agents** — the council's isolated
  dissenting seats: the Chair synthesizes findings into one owned call, the Skeptic hunts the
  load-bearing assumption, the Steelman builds the strongest honest case before the critique. Derived
  from the persona registry (genotype → phenotype).

Catalog totals are now 38 skills and 13 subagents.

## [0.2.0]

### Added
- **`framework-handbook` skill** (`metaphor/`) — author the full "big framework" documentation
  set: philosophy, background, technology rationale, C4 architecture, maintainer guide,
  app-developer guide, contribution guide, glossary, and ADRs. Uses a writer ↔ technical-manager
  review loop with an explicit rubric, the Diátaxis lens, and Metaphor SSoT sourcing. Ships skeleton
  templates (`architecture.md`, `maintainer-guide.md`, `developer-guide.md`, `adr-NNNN.md`).
- **`technical-writer` agent** — drives the `framework-handbook` skill at handbook altitude;
  defers single-file README / API-reference / inline-comment work to `doc-writer`.
- **`business-flow-bdd` skill** (`metaphor/`) — model each feature's business flow (actors,
  preconditions, paths, business rules, postconditions) and turn it into executable BDD/Gherkin
  acceptance specs wired to the project's test harness. Ships `feature.feature` and cucumber-rs
  `steps.rs` templates.
- **`business-flow` agent** — drives the `business-flow-bdd` skill in author mode (document flows +
  scenarios) and test mode (wire step definitions and run them via `metaphor test`).
- **`grouped-commits` skill** (`generic/`) — commit changes split into focused commits grouped by
  functionality (small related files together, large files isolated), each with a one-line message
  that states *why* the change was made; no signatures, no context-free filler messages.
  Complements the *what*-focused `commit-generator`.

Catalog totals are now 37 skills and 10 subagents.

## [0.1.3]

### Changed
- Removed four `generic/` skills that duplicated community skills (code-review, ci-cd, shipping,
  security); their Backbone-specific guidance was preserved in the corresponding community skills
  (`code-review-and-quality`, `test-driven-development`) before removal.
- Removed the generic `tests-maintainer` skill (merged into community `test-driven-development`).
- Synced `Cargo.lock` with the version bump.

## [0.1.2]

### Added
- `agent claude` subcommand to install `CLAUDE.md` orientation files.
- `CLAUDE.md` templates for the workspace and each project type, registered in the manifest.
- Crate `CLAUDE.md` for Claude orientation.

## [0.1.1]

### Added
- `metaphor-cli-master` skill, registered in the manifest and README.

## [0.1.0]

### Added
- Initial release: bundled Claude Code skills and subagents installable into any project via
  `metaphor agent ...` or the standalone `metaphor-agent` binary. Assets embedded at compile time
  via `include_dir!`; `list` / `install` / `info` / `update` / `remove` / `init` commands.
