//! `metaphor-agent claude ...` — install CLAUDE.md templates per project type.
//!
//! Templates live at `assets/claude-md/<name>.md` (embedded via `include_dir!`).
//! The command reads the nearest `metaphor.yaml`, detects framework vs consumer
//! workspace shape, and writes a `CLAUDE.md` at the workspace root plus one at
//! each project path matching its declared `type`.

use anyhow::{anyhow, bail, Context, Result};
use colored::*;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::catalog::ASSETS;

pub struct Options {
    /// Single-project mode: write CLAUDE.md at this path, using the template
    /// that matches the project type (detected from metaphor.yaml) or an
    /// explicit `template` override.
    pub path: Option<PathBuf>,

    /// Force-install this template name at CWD (skips metaphor.yaml entirely).
    pub template: Option<String>,

    /// Overwrite existing CLAUDE.md files.
    pub force: bool,
}

pub fn run_init(opts: Options) -> Result<()> {
    // Direct "install a specific template at CWD" mode.
    if let Some(template) = opts.template {
        let target_dir = opts.path.unwrap_or_else(|| std::env::current_dir().unwrap());
        let dest = target_dir.join("CLAUDE.md");
        install_template(&template, &dest, opts.force)?;
        summary(1, 0);
        return Ok(());
    }

    // Single-project mode: `--path <dir>` with no template → infer type from manifest.
    if let Some(ref target_dir) = opts.path {
        let (manifest, workspace_root) = find_and_load_manifest(target_dir)?;
        let project = manifest.current_project(&workspace_root, target_dir).ok_or_else(|| {
            anyhow!(
                "no project in metaphor.yaml covers {} — pass --template <name> to force",
                target_dir.display()
            )
        })?;
        let template = template_for_type(&project.project_type);
        let dest = target_dir.join("CLAUDE.md");
        install_template(template, &dest, opts.force)?;
        summary(1, 0);
        return Ok(());
    }

    // Default: workspace-wide init.
    let cwd = std::env::current_dir()?;
    let (manifest, workspace_root) = find_and_load_manifest(&cwd)?;

    let workspace_template = workspace_template_for(&manifest);
    println!(
        "{} workspace ({})",
        "Detected".bright_green().bold(),
        workspace_template.cyan()
    );

    let mut installed = 0u32;
    let mut skipped = 0u32;

    // 1. Workspace root CLAUDE.md.
    let root_dest = workspace_root.join("CLAUDE.md");
    match install_template(workspace_template, &root_dest, opts.force)? {
        Outcome::Installed => installed += 1,
        Outcome::Skipped(r) => {
            println!("  {} {} — {}", "skip".yellow(), root_dest.display(), r);
            skipped += 1;
        }
    }

    // 2. Per-project CLAUDE.md files.
    for project in &manifest.projects {
        // Skip upstream clones in consumer workspaces — they'll be wiped by
        // `metaphor sync`, and CLAUDE.md is the upstream repo's responsibility.
        if project.remote.is_some() {
            println!(
                "  {} {} ({}) — upstream clone, CLAUDE.md belongs in its source repo",
                "skip".yellow(),
                project.name,
                project.project_type
            );
            skipped += 1;
            continue;
        }
        let resolved = project.resolved_path(&workspace_root);
        if !resolved.exists() {
            println!(
                "  {} {} ({}) — directory not present (run `metaphor sync`?)",
                "skip".yellow(),
                project.name,
                project.project_type
            );
            skipped += 1;
            continue;
        }
        let template = template_for_type(&project.project_type);
        let dest = resolved.join("CLAUDE.md");
        match install_template(template, &dest, opts.force)? {
            Outcome::Installed => installed += 1,
            Outcome::Skipped(r) => {
                println!("  {} {} — {}", "skip".yellow(), dest.display(), r);
                skipped += 1;
            }
        }
    }

    summary(installed, skipped);
    Ok(())
}

pub fn run_list() -> Result<()> {
    let templates = enumerate_templates()?;
    println!("{}", "Workspace templates:".bright_cyan().bold());
    for t in templates.iter().filter(|t| t.kind == Kind::Workspace) {
        println!("  {}  — {}", t.name.bright_white(), t.description.dimmed());
    }
    println!();
    println!("{}", "Type templates:".bright_cyan().bold());
    for t in templates.iter().filter(|t| t.kind == Kind::Type) {
        println!("  {}  — {}", t.name.bright_white(), t.description.dimmed());
    }
    Ok(())
}

pub fn run_update() -> Result<()> {
    // `update` is `init --force` — reapply templates everywhere.
    run_init(Options {
        path: None,
        template: None,
        force: true,
    })
}

// ── Template installation ───────────────────────────────────────────────

enum Outcome {
    Installed,
    Skipped(String),
}

fn install_template(template: &str, dest: &Path, force: bool) -> Result<Outcome> {
    if dest.exists() && !force {
        return Ok(Outcome::Skipped(
            "already exists (use --force to overwrite)".to_string(),
        ));
    }
    let asset_path = format!("claude-md/{template}.md");
    let file = ASSETS
        .get_file(&asset_path)
        .ok_or_else(|| anyhow!("embedded template missing: {asset_path}"))?;
    let body = strip_frontmatter(file.contents_utf8().unwrap_or_default());
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    fs::write(dest, body).with_context(|| format!("writing {}", dest.display()))?;
    println!(
        "  {} {} → {}",
        "copy".green(),
        template.bright_white(),
        dest.display()
    );
    Ok(Outcome::Installed)
}

/// Strip the YAML frontmatter block from a template so the written CLAUDE.md
/// starts at the `# Heading` line. Frontmatter is internal metadata (template
/// name, description) — not useful once installed on disk.
fn strip_frontmatter(content: &str) -> String {
    if !content.starts_with("---\n") {
        return content.to_string();
    }
    let rest = &content[4..];
    if let Some(end) = rest.find("\n---\n") {
        return rest[end + 5..].trim_start_matches('\n').to_string();
    }
    content.to_string()
}

fn summary(installed: u32, skipped: u32) {
    println!();
    println!(
        "{} {} installed, {} skipped",
        "Done.".bright_green(),
        installed,
        skipped
    );
}

// ── Template enumeration ────────────────────────────────────────────────

#[derive(PartialEq, Eq)]
enum Kind {
    Workspace,
    Type,
}

struct Template {
    name: String,
    description: String,
    kind: Kind,
}

fn enumerate_templates() -> Result<Vec<Template>> {
    let dir = ASSETS
        .get_dir("claude-md")
        .ok_or_else(|| anyhow!("embedded claude-md/ directory missing"))?;
    let mut out = Vec::new();
    for f in dir.files() {
        let Some(stem) = f.path().file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if f.path().extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let description = f
            .contents_utf8()
            .map(read_frontmatter_description)
            .unwrap_or_default();
        let kind = if stem.starts_with("workspace-") {
            Kind::Workspace
        } else {
            Kind::Type
        };
        out.push(Template {
            name: stem.to_string(),
            description,
            kind,
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

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

// ── metaphor.yaml parsing ───────────────────────────────────────────────
//
// We parse only the fields we need. Full schema lives in
// `metaphor-cli/crates/metaphor-workspace` but this plugin is standalone —
// duplicating the minimum keeps us decoupled.

#[derive(Debug, Deserialize)]
struct Manifest {
    #[serde(default)]
    projects: Vec<Project>,
}

#[derive(Debug, Deserialize)]
struct Project {
    #[allow(dead_code)]
    name: String,
    #[serde(rename = "type")]
    project_type: String,
    path: String,
    #[serde(default)]
    remote: Option<String>,
}

impl Manifest {
    fn has_any_remote(&self) -> bool {
        self.projects.iter().any(|p| p.remote.is_some())
    }

    fn current_project(&self, workspace_root: &Path, cwd: &Path) -> Option<&Project> {
        let mut best: Option<(usize, &Project)> = None;
        for p in &self.projects {
            let root = p.resolved_path(workspace_root);
            if cwd.starts_with(&root) {
                let depth = root.components().count();
                match best {
                    Some((d, _)) if d >= depth => {}
                    _ => best = Some((depth, p)),
                }
            }
        }
        best.map(|(_, p)| p)
    }
}

impl Project {
    fn resolved_path(&self, workspace_root: &Path) -> PathBuf {
        let p = PathBuf::from(&self.path);
        let joined = if p.is_absolute() {
            p
        } else {
            workspace_root.join(p)
        };
        joined
            .components()
            .filter(|c| !matches!(c, std::path::Component::CurDir))
            .collect()
    }
}

fn find_and_load_manifest(start: &Path) -> Result<(Manifest, PathBuf)> {
    let mut dir = start.to_path_buf();
    loop {
        let candidate = dir.join("metaphor.yaml");
        if candidate.exists() {
            let raw = fs::read_to_string(&candidate)
                .with_context(|| format!("reading {}", candidate.display()))?;
            let manifest: Manifest =
                serde_yaml::from_str(&raw).context("parsing metaphor.yaml")?;
            return Ok((manifest, dir));
        }
        if !dir.pop() {
            break;
        }
    }
    bail!(
        "metaphor.yaml not found from {} upward — pass --template <name> for single-file install",
        start.display()
    );
}

fn workspace_template_for(manifest: &Manifest) -> &'static str {
    if manifest.has_any_remote() {
        "workspace-consumer"
    } else {
        "workspace-framework"
    }
}

fn template_for_type(project_type: &str) -> &'static str {
    match project_type {
        "crate" => "type-crate",
        "backend-service" => "type-backend-service",
        "module" => "type-module",
        "cli-tool" => "type-cli-tool",
        "mobileapp" => "type-mobileapp",
        // Unsupported-so-far types fall back to the closest shape.
        "webservice" => "type-backend-service",
        "webapp" | "desktopapp" => "type-mobileapp",
        _ => "type-crate",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frontmatter_stripped() {
        let input = "---\nname: foo\ndescription: bar\n---\n\n# Heading\n\nBody\n";
        let out = strip_frontmatter(input);
        assert!(out.starts_with("# Heading"), "got: {out}");
    }

    #[test]
    fn frontmatter_absent_is_noop() {
        let input = "# Heading\nBody\n";
        assert_eq!(strip_frontmatter(input), input);
    }

    #[test]
    fn consumer_detected_by_remote() {
        let yaml = "\
projects:
  - name: lib
    type: crate
    path: ./modules/lib
    remote: https://x.example/lib
  - name: svc
    type: backend-service
    path: ./apps/svc
";
        let m: Manifest = serde_yaml::from_str(yaml).unwrap();
        assert!(m.has_any_remote());
        assert_eq!(workspace_template_for(&m), "workspace-consumer");
    }

    #[test]
    fn framework_detected_without_remote() {
        let yaml = "\
projects:
  - name: lib
    type: crate
    path: ./lib
  - name: svc
    type: backend-service
    path: ./svc
";
        let m: Manifest = serde_yaml::from_str(yaml).unwrap();
        assert!(!m.has_any_remote());
        assert_eq!(workspace_template_for(&m), "workspace-framework");
    }

    #[test]
    fn type_mapping() {
        assert_eq!(template_for_type("module"), "type-module");
        assert_eq!(template_for_type("backend-service"), "type-backend-service");
        assert_eq!(template_for_type("mobileapp"), "type-mobileapp");
        assert_eq!(template_for_type("cli-tool"), "type-cli-tool");
        assert_eq!(template_for_type("crate"), "type-crate");
        // Fallback for unseen types.
        assert_eq!(template_for_type("infra"), "type-crate");
    }
}
