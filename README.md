# metaphor-skill-agents

Bundled Claude Code **skills** and **subagents**, installable into any project with one command.

Ships as a Metaphor CLI plugin (`metaphor agent ...`) or a standalone binary (`metaphor-agent ...`).

## Quick install

```bash
# 1. Install the plugin binary (downloads from GitHub releases → ~/.metaphor/bin/)
metaphor plugin add metaphor-agent@latest

# 2. Go into any project
cd /path/to/your-project

# 3. (optional) bootstrap .claude/ scaffolding
metaphor agent init

# 4. Install skills/agents
metaphor agent list                                      # browse what's available
metaphor agent install --all                             # everything (38 skills + 8 agents)
metaphor agent install --category generic                # only portable skills
metaphor agent install --category community              # the addyosmani/agent-skills set
metaphor agent install commit-generator code-reviewer    # pick specific ones

# Install into ~/.claude/ (available in every project, not just this one)
metaphor agent install --all --global
```

Also available: `metaphor agent info <name>`, `metaphor agent update`, `metaphor agent remove <name>`.
The `skill` alias works too: `metaphor agent skill commit-generator`.

## What's inside

- **39 skills** across four categories:
  - `metaphor/` (1) — `metaphor-cli-master`: teaches Claude to treat the `metaphor` CLI as a first-class workspace tool (commands, plugin model, `metaphor.yaml`, when to prefer it over raw `cargo`/`npm`/`docker`).
  - `generic/` (8) — portable Backbone-adjacent skills: commit-generator, reviewer-code-quality, devops-automation-expert, deployment-orchestrator, security-deployment-specialist, cloud-infrastructure-architect, tests-maintainer, domain-specific-expert
  - `backbone/` (9) — tied to the Backbone/Rust framework: backbone-cli-master, backbone-schema-maintainer, framework-architect, custom-logic-specialist, database-migration-specialist, crate-maintainer, apps-maintainer, creative-domain-architect, modules-orchestrator
  - `community/` (21) — curated production engineering skills from [addyosmani/agent-skills](https://github.com/addyosmani/agent-skills) (MIT). Covers API design, TDD/spec-driven, debugging, CI/CD, security hardening, planning, docs/ADRs, and more. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
- **8 subagents**: code-reviewer, test-writer, refactorer, doc-writer, perf-analyzer, security-auditor, debugger, onboarding-explainer. Three of them (code-reviewer, security-auditor, test-writer) are merged versions combining the original drafts with structure from `addyosmani/agent-skills`.

All assets are embedded into the binary at compile time via `include_dir!`. Installation is a simple copy — no network, no registry.

## Install the plugin binary

```bash
cd metaphor-skill-agents
cargo build --release
# copy target/release/metaphor-agent into $PATH or ~/.metaphor/bin/
```

Once registered with `metaphor-cli`'s `KNOWN_PLUGINS`, it's invokable as:

```bash
metaphor agent list
metaphor agent install code-reviewer
```

## Usage

### From inside a project

```bash
# Bootstrap .claude/ scaffolding
metaphor agent init

# List what's bundled
metaphor agent list
metaphor agent list --category generic
metaphor agent list --installed

# Install into ./.claude/
metaphor agent install commit-generator
metaphor agent install code-reviewer test-writer
metaphor agent install --category generic
metaphor agent install --all
metaphor agent install existing-name --force

# Inspect an entry
metaphor agent info framework-architect

# Update
metaphor agent update                 # refresh everything installed
metaphor agent update commit-generator

# Uninstall
metaphor agent remove commit-generator
```

### Install globally (~/.claude/)

Add `--global` to any command:

```bash
metaphor agent install --all --global
metaphor agent list --global
```

### Natural alias: `metaphor agent skill <name>`

`skill` is an alias of `install` — the phrase the user asked for:

```bash
metaphor agent skill commit-generator
```

## Layout

```
metaphor-skill-agents/
├── Cargo.toml                       # metaphor-agent binary + metaphor_skill_agents lib
├── manifest.yaml                    # top-level catalog
├── src/
│   ├── main.rs                      # clap dispatcher
│   ├── lib.rs
│   ├── catalog.rs                   # include_dir asset scan + frontmatter parse
│   ├── target.rs                    # resolve project vs --global
│   └── commands/
│       ├── install.rs
│       ├── list.rs
│       ├── info.rs
│       ├── remove.rs
│       ├── update.rs
│       └── init.rs
└── assets/
    ├── skills/
    │   ├── generic/<name>/SKILL.md
    │   └── backbone/<name>/SKILL.md
    └── agents/<name>.md
```

## Adding your own skill / agent

1. Drop the skill directory into `assets/skills/<category>/<name>/` (must contain `SKILL.md` with YAML frontmatter: `name`, `description`).
2. Or drop an agent markdown into `assets/agents/<name>.md` (frontmatter: `name`, `description`, optional `tools`, `model`).
3. Append it to `manifest.yaml`.
4. `cargo build --release` — the `include_dir!` macro rebakes the assets.

## How install works

- Skills are directories: copied recursively into `<target>/.claude/skills/<name>/`.
- Agents are single files: copied to `<target>/.claude/agents/<name>.md`.
- Existing targets are preserved unless `--force` is passed.
- `update` is `install --force` over entries that are already present.
