//! Tests Maintainer Skill
//!
//! Comprehensive test generation and maintenance for Backbone Framework modules.
//! Implements Testee framework patterns with 5-category testing approach.
//!
//! Features:
//! - Automatic test file generation for new modules
//! - 5-category testing (Auth, Success, Validation, Business Logic, Edge Cases)
//! - Multi-layer validation (API → DB → Cache → Audit)
//! - Backbone Framework compliance
//! - Template-based code generation
//! - CI/CD integration

pub mod framework;
pub mod templates;
pub mod utils;

// Re-export main functionality
pub use framework::{TestModule, TestEndpoint};
pub use utils::ModuleAnalyzer;