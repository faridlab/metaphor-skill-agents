//! `metaphor-agent init` — bootstrap a `.claude/` directory in the target.

use anyhow::{Context, Result};
use colored::*;
use std::fs;

use crate::target::Target;

pub struct Options {
    pub global: bool,
}

pub fn run(opts: Options) -> Result<()> {
    let target = Target::resolve(opts.global)?;

    for dir in [target.skills_dir(), target.agents_dir()] {
        fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
        println!("  {} {}", "ok".green(), dir.display());
    }

    let settings = target.root.join("settings.json");
    if !settings.exists() {
        fs::write(&settings, "{\n  \"$schema\": \"https://json.schemastore.org/claude-code-settings.json\"\n}\n")
            .with_context(|| format!("writing {}", settings.display()))?;
        println!("  {} {}", "ok".green(), settings.display());
    }

    println!();
    println!(
        "{} initialized at {}",
        ".claude/".bright_green(),
        target.root.display()
    );
    println!("Next: {} to see available skills and agents.", "metaphor agent list".bright_cyan());
    Ok(())
}
