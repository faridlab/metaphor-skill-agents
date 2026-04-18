//! `metaphor-agent list` — enumerate bundled and installed entries.

use anyhow::Result;
use colored::*;

use crate::catalog::{self, Kind};
use crate::target::Target;

pub struct Options {
    pub category: Option<String>,
    pub installed_only: bool,
    pub global: bool,
}

pub fn run(opts: Options) -> Result<()> {
    let target = Target::resolve(opts.global)?;
    let catalog = catalog::catalog();

    let filtered: Vec<_> = catalog
        .into_iter()
        .filter(|e| match &opts.category {
            Some(c) => &e.category == c,
            None => true,
        })
        .collect();

    println!(
        "{} {}",
        "Target:".bold(),
        target.root.display()
    );
    println!();

    let mut last_section = String::new();
    for entry in &filtered {
        let section = match entry.kind {
            Kind::Skill => format!("Skills ({})", entry.category),
            Kind::Agent => "Agents".to_string(),
        };
        if section != last_section {
            println!("{}", section.bright_cyan().bold());
            last_section = section;
        }
        let is_installed = installed(&target, entry);
        if opts.installed_only && !is_installed {
            continue;
        }
        let marker = if is_installed { "✓".green() } else { "·".dimmed() };
        let desc = if entry.description.is_empty() {
            "".to_string()
        } else {
            format!("  — {}", entry.description.dimmed())
        };
        println!("  {} {}{}", marker, entry.name, desc);
    }
    println!();
    Ok(())
}

fn installed(target: &Target, entry: &crate::catalog::Entry) -> bool {
    match entry.kind {
        Kind::Skill => target.skills_dir().join(&entry.name).exists(),
        Kind::Agent => target.agents_dir().join(format!("{}.md", entry.name)).exists(),
    }
}
