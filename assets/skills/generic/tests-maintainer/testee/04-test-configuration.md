# Test Configuration and Registry

This document details the hierarchical test configuration system that enables organized test discovery and execution.

## Configuration Structure

The test configuration follows a hierarchical structure:
```
Domain → Feature → SubFeature → TestType → TestConfig
```

### Python Original

```python
# config_tests.py
TEST_CONFIG = {
    "authorizer": {                          # Domain
        "api": {                             # Feature
            "testApi": {                     # TestType
                "module": "test_api_authorizer_using_api",
                "class": "ApiAuthorizerTest",
                "results_subdir": "api_authorizer"
            }
        },
        "captcha": {
            "testApi": {
                "module": "test_captcha_authorizer_using_api",
                "class": "CaptchaAuthorizerApiTest",
                "results_subdir": "captcha_authorizer_api"
            }
        }
    },
    "customers": {                           # Domain
        "register": {                        # Feature
            "api": {                         # TestType
                "module": "test_register_api",
                "class": "RegisterApiTest",
                "results_subdir": "register_api"
            }
        },
        "profile": {
            "api": {
                "module": "test_customer_profile_api",
                "class": "CustomerProfileApiTest",
                "results_subdir": "customer_profile_api"
            },
            "admin_api": {
                "module": "test_admin_customer_profile",
                "class": "AdminCustomerProfileApiTest",
                "results_subdir": "admin_customer_profile_api"
            }
        },
        "biometric": {                       # SubFeature
            "register": {                    # Feature
                "api": {                     # TestType
                    "module": "test_register_biometric_api",
                    "class": "RegisterBiometricApiTest",
                    "results_subdir": "register_biometric_api"
                }
            }
        },
        "loans": {
            "get_loan": {
                "admin_api": {
                    "module": "test_admin_get_loan",
                    "class": "AdminGetLoanApiTest",
                    "results_subdir": "admin_get_loan_api"
                }
            },
            "get_loans": {
                "admin_api": {
                    "module": "test_admin_get_loans",
                    "class": "AdminGetLoansApiTest",
                    "results_subdir": "admin_get_loans_api"
                }
            }
        }
    },
    "otp": {
        "email": {
            "api": {
                "module": "test_email_verification_api",
                "class": "EmailVerificationApiTest",
                "results_subdir": "email_otp_verification_api"
            }
        },
        "mobile": {
            "api": {
                "module": "test_mobile_verification_api",
                "class": "MobileVerificationApiTest",
                "results_subdir": "mobile_otp_verification_api"
            }
        }
    },
    "services": {
        "audit": {
            "log": {
                "integration": {
                    "module": "test_audit_log_service",
                    "class": "AuditLogServiceTest",
                    "results_subdir": "audit_log_service",
                    "requires_api": False    # Service test, no API needed
                }
            }
        }
    }
}
```

### Key Properties

Each test configuration (leaf node) contains:

| Property | Type | Description |
|----------|------|-------------|
| `module` | String | Python module name (without .py) |
| `class` | String | Test class name to instantiate |
| `results_subdir` | String | Subdirectory for test results |
| `requires_api` | Boolean | Whether test needs API running (default: True) |

---

## Rust Implementation

### Configuration Types

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Individual test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestInfo {
    /// Module path (e.g., "biometric::register_api_test")
    pub module: String,

    /// Test struct name (e.g., "RegisterBiometricApiTest")
    #[serde(rename = "struct")]
    pub struct_name: String,

    /// Subdirectory for results
    pub results_subdir: String,

    /// Whether test requires API to be running
    #[serde(default = "default_requires_api")]
    pub requires_api: bool,
}

fn default_requires_api() -> bool {
    true
}

/// Hierarchical test configuration
/// Can be either a branch (nested config) or a leaf (test info)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestConfigNode {
    /// Branch node containing nested configurations
    Branch(HashMap<String, TestConfigNode>),

    /// Leaf node containing test info
    Leaf(TestInfo),
}

/// Root test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    #[serde(flatten)]
    pub domains: HashMap<String, TestConfigNode>,
}
```

### Configuration Builder

```rust
impl TestConfig {
    /// Create a new test configuration builder
    pub fn builder() -> TestConfigBuilder {
        TestConfigBuilder::new()
    }

    /// Load configuration from TOML file
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: TestConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Get all test paths (dot notation)
    pub fn get_all_test_paths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        self.collect_paths(&self.domains, "", &mut paths);
        paths
    }

    fn collect_paths(
        &self,
        node: &HashMap<String, TestConfigNode>,
        prefix: &str,
        paths: &mut Vec<String>,
    ) {
        for (key, value) in node {
            let current_path = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };

            match value {
                TestConfigNode::Branch(children) => {
                    self.collect_paths(children, &current_path, paths);
                }
                TestConfigNode::Leaf(_) => {
                    paths.push(current_path);
                }
            }
        }
    }

    /// Get test info by path (e.g., "customers.biometric.register.api")
    pub fn get_test_info(&self, path: &str) -> Option<&TestInfo> {
        let parts: Vec<&str> = path.split('.').collect();
        self.traverse_path(&self.domains, &parts)
    }

    fn traverse_path<'a>(
        &'a self,
        node: &'a HashMap<String, TestConfigNode>,
        parts: &[&str],
    ) -> Option<&'a TestInfo> {
        if parts.is_empty() {
            return None;
        }

        let current = parts[0];
        let remaining = &parts[1..];

        match node.get(current)? {
            TestConfigNode::Leaf(info) if remaining.is_empty() => Some(info),
            TestConfigNode::Branch(children) => self.traverse_path(children, remaining),
            _ => None,
        }
    }

    /// Get all tests matching a partial path
    pub fn get_tests_by_prefix(&self, prefix: &str) -> Vec<(&str, &TestInfo)> {
        let mut results = Vec::new();

        for path in self.get_all_test_paths() {
            if path.starts_with(prefix) {
                if let Some(info) = self.get_test_info(&path) {
                    results.push((path.as_str(), info));
                }
            }
        }

        results
    }
}

/// Builder for constructing test configuration
pub struct TestConfigBuilder {
    domains: HashMap<String, TestConfigNode>,
}

impl TestConfigBuilder {
    pub fn new() -> Self {
        Self {
            domains: HashMap::new(),
        }
    }

    /// Add a test at the specified path
    pub fn add_test(mut self, path: &str, info: TestInfo) -> Self {
        let parts: Vec<&str> = path.split('.').collect();
        self.insert_at_path(&mut self.domains, &parts, info);
        self
    }

    fn insert_at_path(
        &self,
        node: &mut HashMap<String, TestConfigNode>,
        parts: &[&str],
        info: TestInfo,
    ) {
        if parts.is_empty() {
            return;
        }

        let current = parts[0].to_string();
        let remaining = &parts[1..];

        if remaining.is_empty() {
            // Insert leaf node
            node.insert(current, TestConfigNode::Leaf(info));
        } else {
            // Get or create branch node
            let branch = node
                .entry(current)
                .or_insert_with(|| TestConfigNode::Branch(HashMap::new()));

            if let TestConfigNode::Branch(children) = branch {
                self.insert_at_path(children, remaining, info);
            }
        }
    }

    pub fn build(self) -> TestConfig {
        TestConfig {
            domains: self.domains,
        }
    }
}
```

### TOML Configuration File (Alternative)

```toml
# tests/config.toml

[authorizer.api.testApi]
module = "authorizer::api_test"
struct = "ApiAuthorizerTest"
results_subdir = "api_authorizer"

[authorizer.captcha.testApi]
module = "authorizer::captcha_test"
struct = "CaptchaAuthorizerApiTest"
results_subdir = "captcha_authorizer_api"

[customers.register.api]
module = "customers::register_api_test"
struct = "RegisterApiTest"
results_subdir = "register_api"

[customers.profile.api]
module = "customers::profile_api_test"
struct = "CustomerProfileApiTest"
results_subdir = "customer_profile_api"

[customers.profile.admin_api]
module = "customers::admin_profile_test"
struct = "AdminCustomerProfileApiTest"
results_subdir = "admin_customer_profile_api"

[customers.biometric.register.api]
module = "biometric::register_api_test"
struct = "RegisterBiometricApiTest"
results_subdir = "register_biometric_api"

[services.audit.log.integration]
module = "services::audit_log_test"
struct = "AuditLogServiceTest"
results_subdir = "audit_log_service"
requires_api = false
```

### Static Configuration (Compile-Time)

For Rust, you may prefer compile-time configuration using macros or const:

```rust
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TEST_CONFIG: TestConfig = {
        TestConfig::builder()
            // Authorizer tests
            .add_test("authorizer.api.testApi", TestInfo {
                module: "authorizer::api_test".into(),
                struct_name: "ApiAuthorizerTest".into(),
                results_subdir: "api_authorizer".into(),
                requires_api: true,
            })

            // Customer tests
            .add_test("customers.register.api", TestInfo {
                module: "customers::register_api_test".into(),
                struct_name: "RegisterApiTest".into(),
                results_subdir: "register_api".into(),
                requires_api: true,
            })
            .add_test("customers.profile.api", TestInfo {
                module: "customers::profile_api_test".into(),
                struct_name: "CustomerProfileApiTest".into(),
                results_subdir: "customer_profile_api".into(),
                requires_api: true,
            })
            .add_test("customers.biometric.register.api", TestInfo {
                module: "biometric::register_api_test".into(),
                struct_name: "RegisterBiometricApiTest".into(),
                results_subdir: "register_biometric_api".into(),
                requires_api: true,
            })

            // Service tests (no API required)
            .add_test("services.audit.log.integration", TestInfo {
                module: "services::audit_log_test".into(),
                struct_name: "AuditLogServiceTest".into(),
                results_subdir: "audit_log_service".into(),
                requires_api: false,
            })

            .build()
    };
}
```

---

## Test Registry

The test registry provides a flat lookup table for script/module to test info mapping.

### Python Original

```python
class TestRunner:
    def _build_test_registry_from_config(self) -> Dict:
        """Build the test registry from unified config."""
        registry = {}

        def extract_test_info(config_node: Dict):
            for key, value in config_node.items():
                if isinstance(value, dict):
                    if "module" in value:
                        # Leaf node with test info
                        script_name = f"{value['module']}.py"
                        registry[script_name] = {
                            "module": value["module"],
                            "class": value["class"],
                            "results_subdir": value["results_subdir"],
                            "requires_api": value.get("requires_api", True)
                        }
                    else:
                        # Branch node, recurse
                        extract_test_info(value)

        extract_test_info(self.test_config)
        return registry
```

### Rust Implementation

```rust
use std::collections::HashMap;

/// Test registry for fast lookup by module name
pub struct TestRegistry {
    entries: HashMap<String, TestRegistryEntry>,
}

#[derive(Debug, Clone)]
pub struct TestRegistryEntry {
    pub module: String,
    pub struct_name: String,
    pub results_subdir: String,
    pub requires_api: bool,
    pub path: String,  // Full hierarchical path
}

impl TestRegistry {
    /// Build registry from test configuration
    pub fn from_config(config: &TestConfig) -> Self {
        let mut entries = HashMap::new();

        fn extract_entries(
            node: &HashMap<String, TestConfigNode>,
            prefix: &str,
            entries: &mut HashMap<String, TestRegistryEntry>,
        ) {
            for (key, value) in node {
                let current_path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };

                match value {
                    TestConfigNode::Branch(children) => {
                        extract_entries(children, &current_path, entries);
                    }
                    TestConfigNode::Leaf(info) => {
                        entries.insert(
                            info.module.clone(),
                            TestRegistryEntry {
                                module: info.module.clone(),
                                struct_name: info.struct_name.clone(),
                                results_subdir: info.results_subdir.clone(),
                                requires_api: info.requires_api,
                                path: current_path,
                            },
                        );
                    }
                }
            }
        }

        extract_entries(&config.domains, "", &mut entries);

        Self { entries }
    }

    /// Get entry by module name
    pub fn get_by_module(&self, module: &str) -> Option<&TestRegistryEntry> {
        self.entries.get(module)
    }

    /// Get all entries
    pub fn all_entries(&self) -> impl Iterator<Item = &TestRegistryEntry> {
        self.entries.values()
    }

    /// Get entries that require API
    pub fn api_tests(&self) -> impl Iterator<Item = &TestRegistryEntry> {
        self.entries.values().filter(|e| e.requires_api)
    }

    /// Get entries that don't require API
    pub fn non_api_tests(&self) -> impl Iterator<Item = &TestRegistryEntry> {
        self.entries.values().filter(|e| !e.requires_api)
    }
}
```

---

## Usage Examples

### Running Specific Tests by Path

```rust
// Get single test
let info = TEST_CONFIG.get_test_info("customers.biometric.register.api");

// Get all tests under a domain
let customer_tests = TEST_CONFIG.get_tests_by_prefix("customers");

// Get all API tests
let api_tests: Vec<_> = TestRegistry::from_config(&TEST_CONFIG)
    .api_tests()
    .collect();
```

### Command Line Integration

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Test path to run (e.g., customers.profile.api)
    #[arg(long)]
    run: Option<String>,
}

fn main() {
    let args = Args::parse();

    let tests_to_run = match args.run {
        Some(path) => {
            // Run specific test(s)
            if let Some(info) = TEST_CONFIG.get_test_info(&path) {
                vec![info]
            } else {
                // Try as prefix
                TEST_CONFIG.get_tests_by_prefix(&path)
                    .into_iter()
                    .map(|(_, info)| info)
                    .collect()
            }
        }
        None => {
            // Run all tests
            TEST_CONFIG.get_all_test_paths()
                .iter()
                .filter_map(|p| TEST_CONFIG.get_test_info(p))
                .collect()
        }
    };

    // Execute tests...
}
```

### Example CLI Usage

```bash
# Run all tests
cargo test --test integration

# Run specific test
cargo test --test integration -- --run customers.biometric.register.api

# Run all customer tests
cargo test --test integration -- --run customers

# Run all profile tests (both api and admin_api)
cargo test --test integration -- --run customers.profile
```
