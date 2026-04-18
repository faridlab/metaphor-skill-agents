//! Metaphor Skill Agents plugin — bundled Claude Code skills & subagents.
//!
//! Exposes commands to install, list, inspect, update, and remove skills/agents
//! into a project's `.claude/` directory (or the user's `~/.claude/`).

pub mod catalog;
pub mod commands;
pub mod target;
