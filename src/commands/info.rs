//! `metaphor-agent info <name>` — show an entry's frontmatter + file layout.

use anyhow::Result;
use colored::*;

use crate::catalog::{self, Kind, ASSETS};

pub fn run(name: &str) -> Result<()> {
    let entry = catalog::find(name)?;
    println!("{} {}", "Name:".bold(), entry.name);
    println!("{} {}", "Kind:".bold(), entry.kind.as_str());
    println!("{} {}", "Category:".bold(), entry.category);
    if !entry.description.is_empty() {
        println!("{} {}", "Description:".bold(), entry.description);
    }
    println!("{} {}", "Asset path:".bold(), entry.asset_path);
    println!();

    match entry.kind {
        Kind::Skill => {
            println!("{}", "Files:".bold());
            if let Some(dir) = ASSETS.get_dir(&entry.asset_path) {
                print_dir(dir, 0);
            }
        }
        Kind::Agent => {
            println!("{}", "Contents (first 40 lines):".bold());
            if let Some(file) = ASSETS.get_file(&entry.asset_path) {
                if let Some(text) = file.contents_utf8() {
                    for line in text.lines().take(40) {
                        println!("  {}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn print_dir(dir: &include_dir::Dir<'_>, depth: usize) {
    let indent = "  ".repeat(depth + 1);
    for f in dir.files() {
        let name = f
            .path()
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        println!("{}{}", indent, name);
    }
    for sub in dir.dirs() {
        let name = sub
            .path()
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        println!("{}{}/", indent, name.bright_blue());
        print_dir(sub, depth + 1);
    }
}
