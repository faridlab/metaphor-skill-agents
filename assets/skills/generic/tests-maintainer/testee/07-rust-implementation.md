# Rust Implementation Guide

This document provides a complete guide for implementing the testing framework in Rust.

## Project Structure

```
your-project/
├── Cargo.toml
├── src/
│   └── ...                        # Your main application
└── tests/
    ├── Cargo.toml                 # Test crate dependencies
    ├── integration/
    │   ├── main.rs                # Test entry point
    │   ├── lib.rs                 # Test library
    │   ├── framework/
    │   │   ├── mod.rs
    │   │   ├── base_test.rs       # Test, ApiTest traits, TestResult
    │   │   ├── test_runner.rs     # TestRunner implementation
    │   │   └── config.rs          # TestConfig, TestRegistry
    │   ├── utils/
    │   │   ├── mod.rs
    │   │   ├── setup_manager.rs   # TestSetupManager
    │   │   ├── jwt_manager.rs     # JwtTokenManager
    │   │   ├── redis_manager.rs   # RedisSessionManager
    │   │   ├── mongo_manager.rs   # MongoDbManager
    │   │   └── common_utils.rs    # CommonUtils
    │   └── tests/
    │       ├── mod.rs
    │       ├── biometric/
    │       │   ├── mod.rs
    │       │   └── register_api_test.rs
    │       ├── customer/
    │       │   ├── mod.rs
    │       │   └── profile_api_test.rs
    │       └── ...
    └── data/
        └── test_fixtures.json     # Optional test fixtures
```

## Cargo.toml for Tests

```toml
[package]
name = "integration-tests"
version = "0.1.0"
edition = "2021"

[[test]]
name = "integration"
path = "integration/main.rs"

[dependencies]
# Async runtime
tokio = { version = "1.0", features = ["full", "rt-multi-thread", "macros"] }
async-trait = "0.1"

# HTTP client
reqwest = { version = "0.11", features = ["json"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database clients
redis = { version = "0.23", features = ["tokio-comp"] }
mongodb = { version = "2.7", features = ["tokio-runtime"] }

# JWT
jsonwebtoken = "9.0"

# Time
chrono = { version = "0.4", features = ["serde"] }

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# CLI
clap = { version = "4.0", features = ["derive"] }

# UUID
uuid = { version = "1.0", features = ["v4"] }

# Base64
base64 = "0.21"

# Lazy initialization
lazy_static = "1.4"

# Optional: TOML config
toml = "0.8"
```

## Step-by-Step Implementation

### Step 1: Create Framework Module

**tests/integration/framework/mod.rs**
```rust
mod base_test;
mod test_runner;
mod config;

pub use base_test::*;
pub use test_runner::*;
pub use config::*;
```

**tests/integration/framework/base_test.rs**
```rust
// Copy the full implementation from 02-base-framework.md
// Includes: TestResult, TestSuiteResult, Test trait, ApiTest trait
```

### Step 2: Create Utils Module

**tests/integration/utils/mod.rs**
```rust
mod setup_manager;
mod jwt_manager;
mod redis_manager;
mod mongo_manager;
mod common_utils;

pub use setup_manager::*;
pub use jwt_manager::*;
pub use redis_manager::*;
pub use mongo_manager::*;
pub use common_utils::*;
```

Copy implementations from 03-common-utilities.md for each file.

### Step 3: Create Configuration

**tests/integration/framework/config.rs**
```rust
use lazy_static::lazy_static;
use std::collections::HashMap;

// Copy TestConfig, TestInfo, TestRegistry from 04-test-configuration.md

lazy_static! {
    pub static ref TEST_CONFIG: TestConfig = {
        TestConfig::builder()
            // Add all your tests here
            .add_test("customers.biometric.register.api", TestInfo {
                module: "tests::biometric::register_api_test".into(),
                struct_name: "RegisterBiometricApiTest".into(),
                results_subdir: "register_biometric_api".into(),
                requires_api: true,
            })
            .add_test("customers.profile.api", TestInfo {
                module: "tests::customer::profile_api_test".into(),
                struct_name: "CustomerProfileApiTest".into(),
                results_subdir: "customer_profile_api".into(),
                requires_api: true,
            })
            // ... add more tests
            .build()
    };
}
```

### Step 4: Create Test Implementations

**tests/integration/tests/mod.rs**
```rust
pub mod biometric;
pub mod customer;
// Add more modules as needed
```

**tests/integration/tests/biometric/mod.rs**
```rust
mod register_api_test;
pub use register_api_test::*;
```

**tests/integration/tests/biometric/register_api_test.rs**
```rust
// Copy full implementation from 05-api-test-patterns.md
```

### Step 5: Create Entry Point

**tests/integration/main.rs**
```rust
mod framework;
mod utils;
mod tests;

use clap::Parser;
use framework::{TestRunner, TEST_CONFIG};
use tests::biometric::RegisterBiometricApiTest;
use tests::customer::CustomerProfileApiTest;

#[derive(Parser)]
#[command(name = "integration-tests")]
#[command(about = "API Integration Test Runner")]
struct Cli {
    /// Test path to run (e.g., customers.profile.api)
    #[arg(long)]
    run: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    let filter = if std::env::var("RUST_LOG").is_ok() {
        tracing_subscriber::EnvFilter::from_default_env()
    } else {
        tracing_subscriber::EnvFilter::new("info")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    let cli = Cli::parse();

    // Create and configure runner
    let mut runner = TestRunner::new();

    // Register all test factories
    register_tests(&mut runner);

    // Run tests
    match runner.run(cli.run.as_deref()).await {
        Ok(()) => {
            println!("\n✅ All tests passed!");
            std::process::exit(0);
        }
        Err(framework::RunnerError::TestsFailed(count)) => {
            println!("\n❌ {} test(s) failed", count);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("\n💥 Error: {}", e);
            std::process::exit(2);
        }
    }
}

/// Register all test factories with the runner
fn register_tests(runner: &mut TestRunner) {
    // Biometric tests
    runner.register_test(
        "tests::biometric::register_api_test",
        |dir| RegisterBiometricApiTest::new(dir),
    );

    // Customer tests
    runner.register_test(
        "tests::customer::profile_api_test",
        |dir| CustomerProfileApiTest::new(dir),
    );

    // Add more test registrations here...
}
```

### Step 6: Create lib.rs for Test Library

**tests/integration/lib.rs**
```rust
pub mod framework;
pub mod utils;
pub mod tests;

// Re-export commonly used items
pub use framework::{Test, ApiTest, TestResult, TestSuiteResult, TestRunner};
pub use utils::{CommonUtils, TestSetupManager, JwtTokenManager};
```

---

## Running Tests

### Development Mode

```bash
# Run all tests
cargo test --test integration

# Run specific test path
cargo test --test integration -- --run customers.biometric.register.api

# Run with verbose output
cargo test --test integration -- --run customers -v

# Run with specific log level
RUST_LOG=debug cargo test --test integration
```

### CI/CD Integration

```yaml
# GitHub Actions example
name: Integration Tests

on: [push, pull_request]

jobs:
  integration-tests:
    runs-on: ubuntu-latest

    services:
      redis:
        image: redis:7
        ports:
          - 6379:6379

      mongodb:
        image: mongo:6
        ports:
          - 27017:27017

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Start API Server
        run: |
          cargo run --release &
          sleep 10  # Wait for server to start

      - name: Run Integration Tests
        env:
          API_BASE_URL: http://localhost:3000
          REDIS_URL: redis://localhost:6379
          MONGODB_URI: mongodb://localhost:27017
        run: cargo test --test integration -- --run all

      - name: Upload Test Results
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: test-results
          path: testing/results/
```

---

## Key Patterns Summary

### 1. Test Lifecycle

```rust
impl Test for MyApiTest {
    fn setup(&mut self) -> Result<(), TestError> {
        // 1. Initialize connections (Redis, MongoDB)
        // 2. Create auth session if needed
        // 3. Prepare test data
        Ok(())
    }

    fn run_tests(&mut self) -> Vec<TestResult> {
        // Execute all test scenarios
        vec![
            self.test_scenario_1(),
            self.test_scenario_2(),
            // ...
        ]
    }

    fn teardown(&mut self) -> Result<(), TestError> {
        // 1. Cleanup test records from database
        // 2. Delete cache entries
        // 3. Close connections
        Ok(())
    }
}
```

### 2. Test Result Pattern

```rust
fn test_something(&mut self) -> TestResult {
    // Arrange
    let request_body = self.create_request();

    // Act
    let response = self.make_request(...).await;

    // Assert
    let success = response.status_code == Some(expected);

    // Report
    TestResult::new("Test Name", success)
        .with_details(format!("Status: {:?}", response.status_code))
        .with_input("request", &request_body)
        .with_output("response", &response)
}
```

### 3. Multi-Step Validation

```rust
fn test_flow(&mut self) -> TestResult {
    // Step 1: API call
    let response = self.make_request(...).await;

    // Step 1a: Validate status
    let (ok, details, body) = self.common_utils.validate_api_response(&response, 200, None, true);
    if !ok {
        return TestResult::fail("Flow").with_details(format!("Step 1a: {}", details));
    }

    // Step 1b: Validate body
    let mismatches = self.common_utils.compare_nested_objects(&expected, body.as_ref().unwrap(), "");
    if !mismatches.is_empty() {
        return TestResult::fail("Flow").with_details(format!("Step 1b: {}", mismatches.join("; ")));
    }

    // Step 1c: Validate database
    let (db_ok, db_details, _) = self.common_utils.assert_db_record(...).await;
    if !db_ok {
        return TestResult::fail("Flow").with_details(format!("Step 1c: {}", db_details));
    }

    // Step 1d: Validate cache
    let (cache_ok, cache_details, _) = self.common_utils.assert_cache_record(...);
    if !cache_ok {
        return TestResult::fail("Flow").with_details(format!("Step 1d: {}", cache_details));
    }

    TestResult::pass("Flow").with_details("All steps passed")
}
```

### 4. Test Data Management

```rust
struct MyApiTest {
    // Track data for cleanup
    created_user_ids: Vec<String>,
    cache_keys_to_delete: Vec<String>,
}

impl MyApiTest {
    fn create_test_user(&mut self) -> String {
        let user_id = format!("test-user-{}", Uuid::new_v4());
        self.created_user_ids.push(user_id.clone());
        user_id
    }
}

impl Test for MyApiTest {
    fn teardown(&mut self) -> Result<(), TestError> {
        // Cleanup all tracked data
        for user_id in &self.created_user_ids {
            self.mongo.delete_one("users", doc! { "_id": user_id }).await?;
        }
        self.redis.delete_keys(&self.cache_keys_to_delete)?;
        Ok(())
    }
}
```

---

## Recommended Crates

| Purpose | Crate | Notes |
|---------|-------|-------|
| Async Runtime | tokio | Full features for tests |
| HTTP Client | reqwest | JSON support built-in |
| Serialization | serde, serde_json | Derive macros |
| Redis | redis | tokio-comp feature |
| MongoDB | mongodb | tokio-runtime feature |
| JWT | jsonwebtoken | HS256/RS256 support |
| Time | chrono | serde feature for serialization |
| Errors | thiserror, anyhow | Custom error types |
| Logging | tracing | Structured logging |
| CLI | clap | Derive macros |
| Config | toml | Optional, for TOML configs |
| UUID | uuid | v4 feature |
| Base64 | base64 | Standard encoding |

---

## Checklist for New Tests

When adding a new API endpoint test:

1. [ ] Create test module file (`tests/endpoint_api_test.rs`)
2. [ ] Add module to parent `mod.rs`
3. [ ] Add test to `TEST_CONFIG` in `config.rs`
4. [ ] Register factory in `main.rs`
5. [ ] Implement struct with test data constants
6. [ ] Implement helper methods for request building
7. [ ] Implement `Test` trait (setup, run_tests, teardown)
8. [ ] Implement `ApiTest` trait
9. [ ] Write test methods:
   - [ ] `test_unauthorized_access()` → 401
   - [ ] `test_invalid_token()` → 403
   - [ ] `test_successful_operation()` → expected status
   - [ ] `test_missing_required_field()` → 400
   - [ ] `test_invalid_values()` → 400
   - [ ] Edge cases as needed
10. [ ] Ensure cleanup in teardown
11. [ ] Run tests locally
12. [ ] Update documentation if needed
