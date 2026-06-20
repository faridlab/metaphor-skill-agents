# Changelog

All notable changes to `metaphor-skill-agents` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
