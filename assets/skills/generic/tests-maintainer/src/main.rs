//! Test Framework Generator for Backbone Framework
//!
//! Implements Testee patterns for comprehensive API testing:
//! - Template Method pattern for test lifecycle
//! - Builder pattern for test results
//! - Factory pattern for dynamic test instantiation
//! - Composite pattern for suite results
//!
//! Automatically generates complete test suites for Backbone modules with:
//! - 5-category testing (Auth, Success, Validation, Business Logic, Edge Cases)
//! - Multi-layer validation (API → DB → Cache → Audit)
//! - Automated test data cleanup
//! - Structured test result reporting

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

use anyhow::{Result, Context};

mod framework;
mod templates;
mod utils;

use framework::{TestEndpoint, TestModule};
use templates::{TemplateEngine, TemplateContext};
use utils::{ModuleAnalyzer, RustFormatter};

/// Main test framework generator
pub struct TestFrameworkGenerator {
    project_root: PathBuf,
    template_engine: TemplateEngine,
    module_analyzer: ModuleAnalyzer,
}

impl TestFrameworkGenerator {
    pub fn new(project_root: impl AsRef<Path>) -> Self {
        Self {
            project_root: project_root.as_ref().to_path_buf(),
            template_engine: TemplateEngine::new(),
            module_analyzer: ModuleAnalyzer::new(),
        }
    }

    /// Generate comprehensive test suite for a module
    pub fn generate_module_tests(&self, module_name: &str) -> Result<HashMap<PathBuf, String>> {
        println!("🚀 Generating comprehensive tests for module: {}", module_name);

        // Analyze module structure
        let module = self.module_analyzer
            .analyze_module(&self.project_root, module_name)
            .context("Failed to analyze module")?;

        println!("  📊 Found {} entities and {} endpoints",
            module.entities.len(),
            module.endpoints.len());

        let mut generated_files = HashMap::new();

        // 1. Generate base framework structure
        generated_files.extend(self.generate_base_framework(&module)?);

        // 2. Generate test utilities and factories
        generated_files.extend(self.generate_test_utilities(&module)?);

        // 3. Generate API tests with 5-category pattern
        generated_files.extend(self.generate_api_tests(&module)?);

        // 4. Generate integration tests
        generated_files.extend(self.generate_integration_tests(&module)?);

        // 5. Generate test runner and scripts
        generated_files.extend(self.generate_test_runners(&module)?);

        // 6. Generate CI/CD configuration
        generated_files.extend(self.generate_cicd_config(&module)?);

        println!("✅ Generated {} test files for module '{}'", generated_files.len(), module_name);
        Ok(generated_files)
    }

    /// Generate base framework files aligned with Backbone Framework patterns
    fn generate_base_framework(&self, module: &TestModule) -> Result<HashMap<PathBuf, String>> {
        let mut files = HashMap::new();

        // Create directory structure following SAPIENS pattern
        let integration_path = self.project_root
            .join("libs/modules")
            .join(&module.name)
            .join("tests/integration/framework");

        // Generate mod.rs for framework
        let framework_mod_path = integration_path.join("mod.rs");
        let context = TemplateContext::new()
            .with("module_name", &module.name)
            .with("entities", &module.entities);

        files.insert(framework_mod_path, self.template_engine.render("framework_mod.rs.j2", &context)?);

        // Generate test configuration (following existing pattern)
        let config_path = self.project_root
            .join("libs/modules")
            .join(&module.name)
            .join("tests/integration/config.rs");

        files.insert(config_path, self.template_engine.render("config.rs.j2", &context)?);

        Ok(files)
    }

    /// Generate test utilities and factories
    fn generate_test_utilities(&self, module: &TestModule) -> Result<HashMap<PathBuf, String>> {
        let mut files = HashMap::new();

        let tests_path = self.project_root
            .join("libs/modules")
            .join(&module.name)
            .join("tests");

        // Common utilities
        let utils_path = tests_path.join("utils/common_utils.rs");
        let context = TemplateContext::new()
            .with("module_name", &module.name)
            .with("entities", &module.entities);

        files.insert(utils_path, self.template_engine.render("common_utils.rs", &context)?);

        // Entity factories
        let factories_mod_path = tests_path.join("factories/mod.rs");
        files.insert(factories_mod_path, self.template_engine.render("factories_mod.rs", &context)?);

        // Individual entity factories
        for entity in &module.entities {
            let factory_path = tests_path.join(format!("factories/{}_factory.rs", entity));

            let entity_context = context.clone()
                .with("entity_name", entity)
                .with("required_fields", self.get_required_fields(entity))
                .with("optional_fields", self.get_optional_fields(entity));

            files.insert(factory_path, self.template_engine.render("entity_factory.rs", &entity_context)?;
        }

        Ok(files)
    }

    /// Generate API tests with 5-category pattern for each endpoint
    fn generate_api_tests(&self, module: &TestModule) -> Result<HashMap<PathBuf, String>> {
        let mut files = HashMap::new();

        for endpoint in &module.endpoints {
            let test_name = format!("{}_{}",
                endpoint.entity_name.to_lowercase(),
                endpoint.method.to_lowercase());

            let test_path = self.project_root
                .join("libs/modules")
                .join(&module.name)
                .join("tests/api")
                .join(format!("{}.rs", test_name));

            let context = TemplateContext::new()
                .with("endpoint", endpoint)
                .with("module_name", &module.name)
                .with("entity_name", &endpoint.entity_name)
                .with("validation_rules", &endpoint.validation_rules)
                .with("business_rules", &endpoint.business_rules)
                .with("required_fields", &endpoint.required_fields)
                .with("optional_fields", &endpoint.optional_fields);

            files.insert(test_path, self.template_engine.render("api_test.rs", &context)?;
        }

        Ok(files)
    }

    /// Generate integration tests
    fn generate_integration_tests(&self, module: &TestModule) -> Result<HashMap<PathBuf, String>> {
        let mut files = HashMap::new();

        let integration_path = self.project_root
            .join("libs/modules")
            .join(&module.name)
            .join("tests/integration");

        // Integration test module
        let mod_path = integration_path.join("mod.rs");
        let context = TemplateContext::new()
            .with("module_name", &module.name)
            .with("entities", &module.entities);

        files.insert(mod_path, self.template_engine.render("integration_mod.rs", &context)?);

        // Entity workflow tests
        for entity in &module.entities {
            let workflow_path = integration_path.join(format!("{}_workflow_tests.rs", entity));

            let context = context.clone()
                .with("entity_name", entity);

            files.insert(workflow_path, self.template_engine.render("workflow_test.rs", &context)?;
        }

        // Error handling tests
        let error_path = integration_path.join("error_handling_tests.rs");
        files.insert(error_path, self.template_engine.render("error_handling_test.rs", &context)?;

        Ok(files)
    }

    /// Generate test runners and scripts
    fn generate_test_runners(&self, module: &TestModule) -> Result<HashMap<PathBuf, String>> {
        let mut files = HashMap::new();

        // Shell test runner
        let runner_path = self.project_root
            .join("libs/modules")
            .join(&module.name)
            .join("tests/run_tests.sh");

        let context = TemplateContext::new()
            .with("module_name", &module.name)
            .with("entities", &module.entities);

        files.insert(runner_path, self.template_engine.render("test_runner.sh", &context)?);

        // Make it executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&runner_path)?.permissions();
            perms.set_mode(0o755); // rwxr-xr-x
            fs::set_permissions(&runner_path, perms)?;
        }

        Ok(files)
    }

    /// Generate CI/CD configuration
    fn generate_cicd_config(&self, module: &TestModule) -> Result<HashMap<PathBuf, String>> {
        let mut files = HashMap::new();

        let workflow_path = self.project_root
            .join(".github/workflows")
            .join(format!("{}_tests.yml", module.name));

        let context = TemplateContext::new()
            .with("module_name", &module.name)
            .with("entities", &module.entities);

        files.insert(workflow_path, self.template_engine.render("github_workflow.yml", &context)?);

        Ok(files)
    }

    /// Helper to get required fields for an entity
    fn get_required_fields(&self, entity: &str) -> Vec<&'static str> {
        match entity {
            "user" => vec!["email", "username", "password_hash"],
            "role" => vec!["name"],
            "permission" => vec!["name", "resource"],
            "session" => vec!["user_id", "device_type"],
            "profile" => vec!["user_id"],
            "audit_log" => vec!["action", "entity_type", "entity_id"],
            "mfa_device" => vec!["user_id", "device_type", "public_key"],
            "password_reset_token" => vec!["user_id", "token"],
            "user_settings" => vec!["user_id"],
            "system_settings" => vec!["key", "value"],
            _ => vec![],
        }
    }

    /// Helper to get optional fields for an entity
    fn get_optional_fields(&self, entity: &str) -> Vec<&'static str> {
        match entity {
            "user" => vec!["status", "email_verified", "failed_login_attempts", "locked_until", "last_login"],
            "role" => vec!["description", "is_default"],
            "permission" => vec!["description", "conditions"],
            "session" => vec!["ip_address", "user_agent", "expires_at"],
            "profile" => vec!["first_name", "last_name", "phone", "address", "birth_date"],
            "audit_log" => vec!["details", "user_id", "ip_address"],
            "mfa_device" => vec!["device_name", "is_active", "last_used_at"],
            "password_reset_token" => vec!["expires_at", "used_at"],
            "user_settings" => vec!["theme", "language", "timezone", "notifications"],
            "system_settings" => vec!["description", "category", "is_public"],
            _ => vec![],
        }
    }
}

fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <project_root> <module_name>", args[0]);
        eprintln!("Example: {} /path/to/project sapiens", args[0]);
        std::process::exit(1);
    }

    let project_root = &args[1];
    let module_name = &args[2];

    let generator = TestFrameworkGenerator::new(project_root);

    match generator.generate_module_tests(module_name) {
        Ok(files) => {
            println!("\n🎉 Successfully generated {} test files for module '{}'", files.len(), module_name);
            println!("\n📁 Generated files:");
            for (path, _) in files {
                println!("  - {}", path.display());
            }
        }
        Err(e) => {
            eprintln!("❌ Error generating tests: {}", e);
            std::process::exit(1);
        }
    }
}