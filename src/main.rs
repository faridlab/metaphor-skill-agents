//! Metaphor Skill Agents plugin — binary entry point.
//!
//! Binary: `metaphor-agent`
//! Subcommand registered in metaphor-cli: `agent`

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

use metaphor_skill_agents::commands::{info, init, install, list, remove, update};

#[derive(Parser)]
#[command(
    name = "metaphor-agent",
    version,
    about = "Install Claude Code skills and subagents into any project",
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Target the user-global ~/.claude/ instead of the project's .claude/
    #[arg(long, global = true)]
    global: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Top-level passthrough namespace: `metaphor agent <subcommand>`.
    ///
    /// Lets metaphor-cli invoke us with a leading "agent" arg (matching its
    /// `dispatch_plugin("metaphor-agent", Some("agent"), ...)`) while still
    /// allowing the binary to be used standalone.
    Agent {
        #[command(subcommand)]
        action: AgentAction,
    },

    /// Install one or more skills/agents into the target `.claude/`
    Install(InstallArgs),

    /// List bundled and installed skills/agents
    List(ListArgs),

    /// Show frontmatter and file layout for a single entry
    Info { name: String },

    /// Remove installed skills/agents from the target `.claude/`
    Remove(RemoveArgs),

    /// Refresh installed copies from the bundled catalog
    Update(UpdateArgs),

    /// Create `.claude/` scaffolding (skills/, agents/, settings.json)
    Init,
}

#[derive(Subcommand)]
enum AgentAction {
    Install(InstallArgs),
    List(ListArgs),
    Info { name: String },
    Remove(RemoveArgs),
    Update(UpdateArgs),
    Init,
    /// Alias of `install` — reads most naturally as `metaphor agent skill <name>`
    Skill(InstallArgs),
}

#[derive(clap::Args)]
struct InstallArgs {
    /// Skill or agent names (positional, repeatable)
    names: Vec<String>,

    /// Install every bundled skill + agent
    #[arg(long)]
    all: bool,

    /// Install only entries in this category: generic | backbone | agents
    #[arg(long)]
    category: Option<String>,

    /// Overwrite existing files
    #[arg(long)]
    force: bool,
}

#[derive(clap::Args)]
struct ListArgs {
    /// Filter by category: generic | backbone | agents
    #[arg(long)]
    category: Option<String>,

    /// Only show entries already installed in the target
    #[arg(long)]
    installed: bool,
}

#[derive(clap::Args)]
struct RemoveArgs {
    names: Vec<String>,
}

#[derive(clap::Args)]
struct UpdateArgs {
    /// Entries to update. If omitted, refreshes every installed entry.
    names: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("{}", "⚡ Metaphor Agent".bright_green().bold());
    println!();

    let global = cli.global;
    match cli.command {
        Command::Agent { action } => dispatch(action, global),
        Command::Install(a) => install::run(build_install(a, global)),
        Command::List(a) => list::run(build_list(a, global)),
        Command::Info { name } => info::run(&name),
        Command::Remove(a) => remove::run(build_remove(a, global)),
        Command::Update(a) => update::run(build_update(a, global)),
        Command::Init => init::run(init::Options { global }),
    }
}

fn dispatch(action: AgentAction, global: bool) -> Result<()> {
    match action {
        AgentAction::Install(a) | AgentAction::Skill(a) => install::run(build_install(a, global)),
        AgentAction::List(a) => list::run(build_list(a, global)),
        AgentAction::Info { name } => info::run(&name),
        AgentAction::Remove(a) => remove::run(build_remove(a, global)),
        AgentAction::Update(a) => update::run(build_update(a, global)),
        AgentAction::Init => init::run(init::Options { global }),
    }
}

fn build_install(a: InstallArgs, global: bool) -> install::Options {
    install::Options {
        names: a.names,
        all: a.all,
        category: a.category,
        global,
        force: a.force,
    }
}

fn build_list(a: ListArgs, global: bool) -> list::Options {
    list::Options {
        category: a.category,
        installed_only: a.installed,
        global,
    }
}

fn build_remove(a: RemoveArgs, global: bool) -> remove::Options {
    remove::Options {
        names: a.names,
        global,
    }
}

fn build_update(a: UpdateArgs, global: bool) -> update::Options {
    update::Options {
        names: a.names,
        global,
    }
}
