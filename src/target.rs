//! Resolves the `.claude/` directory we write into.

use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub struct Target {
    pub root: PathBuf,
    pub global: bool,
}

impl Target {
    /// `--global` → `~/.claude/`; otherwise → `<cwd>/.claude/`.
    pub fn resolve(global: bool) -> Result<Self> {
        let root = if global {
            let home = dirs::home_dir().ok_or_else(|| anyhow!("cannot resolve home directory"))?;
            home.join(".claude")
        } else {
            std::env::current_dir()?.join(".claude")
        };
        Ok(Target { root, global })
    }

    pub fn skills_dir(&self) -> PathBuf {
        self.root.join("skills")
    }

    pub fn agents_dir(&self) -> PathBuf {
        self.root.join("agents")
    }
}
