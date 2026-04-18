//! `metaphor-agent update [<name>...]` — refresh installed copies.
//!
//! Without names, refreshes every currently-installed entry.

use anyhow::Result;
use colored::*;

use crate::catalog::{self, Kind};
use crate::commands::install::{self, Options as InstallOptions};
use crate::target::Target;

pub struct Options {
    pub names: Vec<String>,
    pub global: bool,
}

pub fn run(opts: Options) -> Result<()> {
    let target = Target::resolve(opts.global)?;
    let targets = if opts.names.is_empty() {
        catalog::catalog()
            .into_iter()
            .filter(|e| is_installed(&target, e))
            .map(|e| e.name)
            .collect()
    } else {
        opts.names
    };

    if targets.is_empty() {
        println!("{}", "Nothing installed to update.".yellow());
        return Ok(());
    }

    println!("{} {} entrie(s)", "Updating".bright_green().bold(), targets.len());
    install::run(InstallOptions {
        names: targets,
        all: false,
        category: None,
        global: opts.global,
        force: true,
    })
}

fn is_installed(target: &Target, entry: &catalog::Entry) -> bool {
    match entry.kind {
        Kind::Skill => target.skills_dir().join(&entry.name).exists(),
        Kind::Agent => target.agents_dir().join(format!("{}.md", entry.name)).exists(),
    }
}
