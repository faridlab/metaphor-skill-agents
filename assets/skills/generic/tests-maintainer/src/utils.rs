//! Utility modules for test framework generation

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

use anyhow::{Result, Context};

use crate::framework::{TestModule, TestEndpoint};

pub struct ModuleAnalyzer {
    // Could add configuration here
}

impl ModuleAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Analyze a module to extract entities and endpoints
    pub fn analyze_module(&self, project_root: &Path, module_name: &str) -> Result<TestModule> {
        let module_path = project_root
            .join("libs/modules")
            .join(module_name);

        if !module_path.exists() {
            anyhow::bail!("Module {} not found at {:?}", module_name, module_path);
        }

        println!("  🔍 Analyzing module: {}", module_name);

        let mut module = TestModule::new(module_name.to_string());

        // Find entities
        let entities = self.find_entities(&module_path)?;
        println!("    📋 Found {} entities: {:?}", entities.len(), entities);
        for entity in entities {
            module = module.with_entity(entity);
        }

        // Find endpoints from handlers
        let endpoints = self.find_endpoints(&module_path, module_name)?;
        println!("    🌐 Found {} endpoints", endpoints.len());
        for endpoint in endpoints {
            module = module.with_endpoint(endpoint);
        }

        // Determine module capabilities
        let has_database = module_path.join("src/infrastructure/persistence").exists();
        let has_cache = module_path.join("src/infrastructure/cache").exists();
        let has_auth = self.has_authentication(&module_path);

        if !has_database {
            module = module.without_database();
        }
        if !has_cache {
            module = module.without_cache();
        }
        if !has_auth {
            module = module.without_auth();
        }

        Ok(module)
    }

    /// Find entity definitions in the module
    fn find_entities(&self, module_path: &Path) -> Result<Vec<String>> {
        let mut entities = Vec::new();
        let entity_dir = module_path.join("src/domain/entity");

        if entity_dir.exists() {
            for entry in fs::read_dir(&entity_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        // Remove common suffixes
                        let entity_name = file_stem
                            .replace("_entity", "")
                            .replace("_rs", "")
                            .replace(".rs", "");

                        if !entity_name.is_empty() && entity_name != "mod" {
                            entities.push(entity_name);
                        }
                    }
                }
            }
        }

        Ok(entities)
    }

    /// Find endpoints from handler files
    fn find_endpoints(&self, module_path: &Path, module_name: &str) -> Result<Vec<TestEndpoint>> {
        let mut endpoints = Vec::new();
        let handler_dir = module_path.join("src/presentation/http");

        if !handler_dir.exists() {
            return Ok(endpoints);
        }

        for entry in fs::read_dir(&handler_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                    // Extract entity name from handler name
                    if file_stem.ends_with("_handler") {
                        let entity_name = file_stem.replace("_handler", "").to_lowercase();
                        let entity_singular = entity_name.trim_end_matches('s');

                        // Generate standard CRUD endpoints for this entity
                        endpoints.extend(self.generate_crud_endpoints(entity_singular, module_name));
                    }
                }
            }
        }

        Ok(endpoints)
    }

    /// Generate standard CRUD endpoints for an entity
    fn generate_crud_endpoints(&self, entity_name: &str, module_name: &str) -> Vec<TestEndpoint> {
        let base_path = format!("/api/v1/{}s", entity_name);
        let mut endpoints = Vec::new();

        // List endpoints
        endpoints.push(TestEndpoint {
            path: base_path.clone(),
            method: "GET".to_string(),
            entity_name: entity_name.to_string(),
            module_name: module_name.to_string(),
            required_fields: vec![],
            optional_fields: vec!["page".to_string(), "limit".to_string(), "filter".to_string()],
            validation_rules: {
                let mut rules = HashMap::new();
                rules.insert("page".to_string(), "positive_integer".to_string());
                rules.insert("limit".to_string(), "positive_integer".to_string());
                rules
            },
            business_rules: vec![],
        });

        // Create endpoint
        endpoints.push(TestEndpoint {
            path: base_path.clone(),
            method: "POST".to_string(),
            entity_name: entity_name.to_string(),
            module_name: module_name.to_string(),
            required_fields: self.get_required_fields_for_entity(entity_name),
            optional_fields: self.get_optional_fields_for_entity(entity_name),
            validation_rules: self.get_validation_rules_for_entity(entity_name),
            business_rules: self.get_business_rules_for_entity(entity_name),
        });

        // Get by ID endpoint
        endpoints.push(TestEndpoint {
            path: format!("{}/{{id}}", base_path.clone()),
            method: "GET".to_string(),
            entity_name: entity_name.to_string(),
            module_name: module_name.to_string(),
            required_fields: vec!["id".to_string()],
            optional_fields: vec![],
            validation_rules: HashMap::new(),
            business_rules: vec![],
        });

        // Update endpoint
        endpoints.push(TestEndpoint {
            path: format!("{}/{{id}}", base_path.clone()),
            method: "PUT".to_string(),
            entity_name: entity_name.to_string(),
            module_name: module_name.to_string(),
            required_fields: vec!["id".to_string()],
            optional_fields: self.get_optional_fields_for_entity(entity_name),
            validation_rules: self.get_validation_rules_for_entity(entity_name),
            business_rules: vec![],
        });

        // Delete endpoint
        endpoints.push(TestEndpoint {
            path: format!("{}/{{id}}", base_path),
            method: "DELETE".to_string(),
            entity_name: entity_name.to_string(),
            module_name: module_name.to_string(),
            required_fields: vec!["id".to_string()],
            optional_fields: vec![],
            validation_rules: HashMap::new(),
            business_rules: vec![],
        });

        endpoints
    }

    /// Check if module has authentication
    fn has_authentication(&self, module_path: &Path) -> bool {
        let auth_files = [
            "src/presentation/http/middleware/auth.rs",
            "src/application/services/auth_service.rs",
            "src/domain/entity/user.rs",
        ];

        for auth_file in &auth_files {
            if module_path.join(auth_file).exists() {
                return true;
            }
        }

        false
    }

    /// Get required fields for an entity
    fn get_required_fields_for_entity(&self, entity_name: &str) -> Vec<String> {
        match entity_name {
            "user" => vec!["email".to_string(), "username".to_string(), "password_hash".to_string()],
            "role" => vec!["name".to_string()],
            "permission" => vec!["name".to_string(), "resource".to_string()],
            "session" => vec!["user_id".to_string(), "device_type".to_string()],
            "profile" => vec!["user_id".to_string()],
            "audit_log" => vec!["action".to_string(), "entity_type".to_string(), "entity_id".to_string()],
            "mfa_device" => vec!["user_id".to_string(), "device_type".to_string(), "public_key".to_string()],
            "password_reset_token" => vec!["user_id".to_string(), "token".to_string()],
            "user_settings" => vec!["user_id".to_string()],
            "system_settings" => vec!["key".to_string(), "value".to_string()],
            _ => vec![],
        }
    }

    /// Get optional fields for an entity
    fn get_optional_fields_for_entity(&self, entity_name: &str) -> Vec<String> {
        match entity_name {
            "user" => vec![
                "status".to_string(),
                "email_verified".to_string(),
                "failed_login_attempts".to_string(),
                "locked_until".to_string(),
                "last_login".to_string(),
            ],
            "role" => vec!["description".to_string(), "is_default".to_string()],
            "permission" => vec!["description".to_string(), "conditions".to_string()],
            "session" => vec!["ip_address".to_string(), "user_agent".to_string(), "expires_at".to_string()],
            "profile" => vec![
                "first_name".to_string(),
                "last_name".to_string(),
                "phone".to_string(),
                "address".to_string(),
                "birth_date".to_string(),
            ],
            "audit_log" => vec!["details".to_string(), "user_id".to_string(), "ip_address".to_string()],
            "mfa_device" => vec![
                "device_name".to_string(),
                "is_active".to_string(),
                "last_used_at".to_string(),
            ],
            "password_reset_token" => vec!["expires_at".to_string(), "used_at".to_string()],
            "user_settings" => vec![
                "theme".to_string(),
                "language".to_string(),
                "timezone".to_string(),
                "notifications".to_string(),
            ],
            "system_settings" => vec!["description".to_string(), "category".to_string(), "is_public".to_string()],
            _ => vec![],
        }
    }

    /// Get validation rules for an entity
    fn get_validation_rules_for_entity(&self, entity_name: &str) -> HashMap<String, String> {
        let mut rules = HashMap::new();

        match entity_name {
            "user" => {
                rules.insert("email".to_string(), "valid_email_format".to_string());
                rules.insert("username".to_string(), "alphanumeric_underscores_min_3_max_50".to_string());
                rules.insert("password_hash".to_string(), "min_60_chars_argon2_format".to_string());
            }
            "role" => {
                rules.insert("name".to_string(), "alphanumeric_spaces_min_2_max_100_unique".to_string());
            }
            "permission" => {
                rules.insert("name".to_string(), "alphanumeric_underscores_min_2_max_100".to_string());
                rules.insert("resource".to_string(), "resource_format".to_string());
            }
            _ => {}
        }

        rules
    }

    /// Get business rules for an entity
    fn get_business_rules_for_entity(&self, entity_name: &str) -> Vec<String> {
        match entity_name {
            "user" => vec![
                "email_must_be_unique".to_string(),
                "username_must_be_unique".to_string(),
                "password_must_be_hashed".to_string(),
                "failed_login_attempts_limit".to_string(),
                "account_lock_after_max_attempts".to_string(),
            ],
            "role" => vec![
                "role_name_must_be_unique".to_string(),
                "default_role_cannot_be_deleted".to_string(),
                "cannot_delete_role_with_assigned_users".to_string(),
            ],
            "session" => vec![
                "one_active_session_per_user".to_string(),
                "session_expires_after_inactivity".to_string(),
                "max_sessions_per_user".to_string(),
            ],
            _ => vec![],
        }
    }
}

/// Rust code formatter
pub struct RustFormatter;

impl RustFormatter {
    pub fn format_code(code: &str) -> Result<String> {
        // For now, return as-is
        // In a real implementation, you'd use rustfmt
        Ok(code.to_string())
    }
}