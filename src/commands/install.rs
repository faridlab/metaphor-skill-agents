//! `metaphor-agent install <name>...` — copy bundled asset into target `.claude/`.

use anyhow::{anyhow, bail, Context, Result};
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};

use crate::catalog::{self, Entry, Kind, ASSETS};
use crate::target::Target;

pub struct Options {
    pub names: Vec<String>,
    pub all: bool,
    pub category: Option<String>,
    pub global: bool,
    pub force: bool,
}

pub fn run(opts: Options) -> Result<()> {
    let target = Target::resolve(opts.global)?;
    let catalog = catalog::catalog();

    let selected: Vec<Entry> = if opts.all {
        catalog
    } else if let Some(cat) = &opts.category {
        let filtered: Vec<Entry> = catalog
            .into_iter()
            .filter(|e| &e.category == cat)
            .collect();
        if filtered.is_empty() {
            bail!("no bundled entries match category '{cat}' (try: generic, backbone, agents)");
        }
        filtered
    } else {
        if opts.names.is_empty() {
            bail!("install: pass one or more names, or use --all / --category <name>");
        }
        opts.names
            .iter()
            .map(|n| catalog::find(n))
            .collect::<Result<Vec<_>>>()?
    };

    println!(
        "{} {} entrie(s) into {}",
        "Installing".bright_green().bold(),
        selected.len(),
        target.root.display()
    );

    let mut installed = 0;
    let mut skipped = 0;
    for entry in &selected {
        match install_one(entry, &target, opts.force)? {
            Outcome::Installed => installed += 1,
            Outcome::Skipped(reason) => {
                skipped += 1;
                println!(
                    "  {} {} ({}) — {}",
                    "skip".yellow(),
                    entry.name,
                    entry.kind.as_str(),
                    reason
                );
            }
        }
    }

    println!();
    println!(
        "{} {} installed, {} skipped",
        "Done.".bright_green(),
        installed,
        skipped
    );
    Ok(())
}

enum Outcome {
    Installed,
    Skipped(String),
}

fn install_one(entry: &Entry, target: &Target, force: bool) -> Result<Outcome> {
    let dest = destination(entry, target);
    if dest.exists() && !force {
        return Ok(Outcome::Skipped(format!(
            "already exists at {} (use --force to overwrite)",
            dest.display()
        )));
    }

    match entry.kind {
        Kind::Skill => copy_dir(&entry.asset_path, &dest)?,
        Kind::Agent => copy_file(&entry.asset_path, &dest)?,
    }

    println!(
        "  {} {} ({}) → {}",
        "copy".green(),
        entry.name,
        entry.kind.as_str(),
        dest.display()
    );
    Ok(Outcome::Installed)
}

fn destination(entry: &Entry, target: &Target) -> PathBuf {
    match entry.kind {
        Kind::Skill => target.skills_dir().join(&entry.name),
        Kind::Agent => target.agents_dir().join(format!("{}.md", entry.name)),
    }
}

fn copy_dir(asset_path: &str, dest: &Path) -> Result<()> {
    let dir = ASSETS
        .get_dir(asset_path)
        .ok_or_else(|| anyhow!("embedded dir missing: {asset_path}"))?;
    if dest.exists() {
        fs::remove_dir_all(dest)
            .with_context(|| format!("removing existing {}", dest.display()))?;
    }
    fs::create_dir_all(dest).with_context(|| format!("creating {}", dest.display()))?;
    write_dir_recursive(dir, dest)?;
    Ok(())
}

fn write_dir_recursive(dir: &include_dir::Dir<'_>, dest: &Path) -> Result<()> {
    for f in dir.files() {
        let rel = f
            .path()
            .strip_prefix(dir.path())
            .unwrap_or_else(|_| f.path());
        let out = dest.join(rel);
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&out, f.contents())
            .with_context(|| format!("writing {}", out.display()))?;
    }
    for sub in dir.dirs() {
        let rel = sub
            .path()
            .strip_prefix(dir.path())
            .unwrap_or_else(|_| sub.path());
        let out = dest.join(rel);
        fs::create_dir_all(&out)?;
        write_dir_recursive(sub, &out)?;
    }
    Ok(())
}

fn copy_file(asset_path: &str, dest: &Path) -> Result<()> {
    let file = ASSETS
        .get_file(asset_path)
        .ok_or_else(|| anyhow!("embedded file missing: {asset_path}"))?;
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(dest, file.contents())
        .with_context(|| format!("writing {}", dest.display()))?;
    Ok(())
}
