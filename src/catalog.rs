//! Bundled catalog of skills + agents, embedded at compile time via `include_dir`.
//!
//! The binary ships with every asset file baked in; installation copies from
//! the embedded tree to disk. No runtime asset path resolution needed.

use anyhow::{anyhow, Result};
use include_dir::{include_dir, Dir};

pub static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Skill,
    Agent,
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::Skill => "skill",
            Kind::Agent => "agent",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub kind: Kind,
    /// For skills: "generic" or "backbone". For agents: "agents".
    pub category: String,
    /// Path inside the embedded tree, e.g. "skills/generic/commit-generator".
    pub asset_path: String,
    /// One-line description scraped from the entry's frontmatter.
    pub description: String,
}

/// Enumerate every bundled skill and agent.
pub fn catalog() -> Vec<Entry> {
    let mut out = Vec::new();

    // Skills: assets/skills/{generic,backbone}/<name>/SKILL.md
    if let Some(skills) = ASSETS.get_dir("skills") {
        for cat_dir in skills.dirs() {
            let category = dir_name(cat_dir.path().to_str().unwrap_or_default());
            for skill_dir in cat_dir.dirs() {
                let name = dir_name(skill_dir.path().to_str().unwrap_or_default());
                let description = read_skill_description(skill_dir).unwrap_or_default();
                out.push(Entry {
                    name: name.to_string(),
                    kind: Kind::Skill,
                    category: category.to_string(),
                    asset_path: skill_dir.path().to_string_lossy().into_owned(),
                    description,
                });
            }
        }
    }

    // Agents: assets/agents/<name>.md
    if let Some(agents) = ASSETS.get_dir("agents") {
        for f in agents.files() {
            let path = f.path();
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let description = f
                .contents_utf8()
                .map(read_frontmatter_description)
                .unwrap_or_default();
            out.push(Entry {
                name: stem.to_string(),
                kind: Kind::Agent,
                category: "agents".to_string(),
                asset_path: path.to_string_lossy().into_owned(),
                description,
            });
        }
    }

    out.sort_by(|a, b| {
        a.kind
            .as_str()
            .cmp(b.kind.as_str())
            .then(a.category.cmp(&b.category))
            .then(a.name.cmp(&b.name))
    });
    out
}

pub fn find(name: &str) -> Result<Entry> {
    catalog()
        .into_iter()
        .find(|e| e.name == name)
        .ok_or_else(|| anyhow!("unknown skill or agent: {name}"))
}

fn dir_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

fn read_skill_description(dir: &include_dir::Dir<'_>) -> Option<String> {
    for candidate in ["SKILL.md", "skill.md", "README.md"] {
        let p = dir.path().join(candidate);
        if let Some(file) = ASSETS.get_file(&p) {
            if let Some(text) = file.contents_utf8() {
                return Some(read_frontmatter_description(text));
            }
        }
    }
    None
}

/// Parse `description:` from YAML frontmatter. Returns empty string if absent.
fn read_frontmatter_description(content: &str) -> String {
    let mut lines = content.lines();
    if lines.next() != Some("---") {
        return String::new();
    }
    for line in lines {
        if line == "---" {
            break;
        }
        if let Some(rest) = line.strip_prefix("description:") {
            return rest.trim().trim_matches('"').trim_matches('\'').to_string();
        }
    }
    String::new()
}
