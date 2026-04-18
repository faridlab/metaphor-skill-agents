//! `metaphor-agent remove <name>...` — delete installed copies from target `.claude/`.

use anyhow::{bail, Context, Result};
use colored::*;
use std::fs;

use crate::catalog::{self, Kind};
use crate::target::Target;

pub struct Options {
    pub names: Vec<String>,
    pub global: bool,
}

pub fn run(opts: Options) -> Result<()> {
    if opts.names.is_empty() {
        bail!("remove: pass one or more names");
    }
    let target = Target::resolve(opts.global)?;
    for name in &opts.names {
        let entry = catalog::find(name)?;
        let path = match entry.kind {
            Kind::Skill => target.skills_dir().join(&entry.name),
            Kind::Agent => target.agents_dir().join(format!("{}.md", entry.name)),
        };
        if !path.exists() {
            println!(
                "  {} {} — not installed at {}",
                "skip".yellow(),
                name,
                path.display()
            );
            continue;
        }
        if path.is_dir() {
            fs::remove_dir_all(&path)
                .with_context(|| format!("removing {}", path.display()))?;
        } else {
            fs::remove_file(&path)
                .with_context(|| format!("removing {}", path.display()))?;
        }
        println!("  {} {} → {}", "removed".green(), name, path.display());
    }
    Ok(())
}
