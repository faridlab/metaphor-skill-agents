//! Framework structures for test generation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an API endpoint for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestEndpoint {
    pub path: String,
    pub method: String,
    pub entity_name: String,
    pub module_name: String,
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub validation_rules: HashMap<String, String>,
    pub business_rules: Vec<String>,
}

/// Represents a module for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestModule {
    pub name: String,
    pub entities: Vec<String>,
    pub endpoints: Vec<TestEndpoint>,
    pub has_database: bool,
    pub has_cache: bool,
    pub has_auth: bool,
}

impl TestModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            entities: Vec::new(),
            endpoints: Vec::new(),
            has_database: true,
            has_cache: true,
            has_auth: true,
        }
    }

    pub fn with_entity(mut self, entity: String) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn with_endpoint(mut self, endpoint: TestEndpoint) -> Self {
        self.endpoints.push(endpoint);
        self
    }

    pub fn without_database(mut self) -> Self {
        self.has_database = false;
        self
    }

    pub fn without_cache(mut self) -> Self {
        self.has_cache = false;
        self
    }

    pub fn without_auth(mut self) -> Self {
        self.has_auth = false;
        self
    }
}