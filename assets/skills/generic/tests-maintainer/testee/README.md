# Testing Framework Documentation

This documentation describes the MyPayNow API testing framework and provides a comprehensive guide for implementing a similar framework in Rust.

## Documentation Index

| Document | Description |
|----------|-------------|
| [01-overview.md](./01-overview.md) | Framework architecture, goals, and design patterns |
| [02-base-framework.md](./02-base-framework.md) | Core traits: Test, ApiTest, TestResult, TestSuiteResult |
| [03-common-utilities.md](./03-common-utilities.md) | Utility managers: Setup, JWT, Redis, MongoDB, CommonUtils |
| [04-test-configuration.md](./04-test-configuration.md) | Hierarchical test configuration and registry |
| [05-api-test-patterns.md](./05-api-test-patterns.md) | Complete test examples with validation patterns |
| [06-test-runner.md](./06-test-runner.md) | Test orchestration and result aggregation |
| [07-rust-implementation.md](./07-rust-implementation.md) | Step-by-step Rust implementation guide |

## Quick Reference

### Test Lifecycle

```
execute() → setup() → run_tests() → teardown() → save_results()
```

### Test Categories per Endpoint

1. **Auth Tests**: 401 (no auth), 403 (invalid token)
2. **Success Tests**: Happy path with DB/cache validation
3. **Validation Tests**: Missing fields, invalid values → 400
4. **Business Logic**: Conflicts, not found, idempotency
5. **Edge Cases**: Boundaries, special characters

### Validation Layers

```
API Response → Response Body → Database Record → Cache Entry → Audit Log
```

### Key Patterns

- **Template Method**: Test lifecycle orchestration
- **Factory**: Dynamic test instantiation from config
- **Builder**: TestResult and request body construction
- **Composite**: Individual results → Suite results

## Source Files Reference

This documentation is based on the Python testing framework in:

| File | Purpose |
|------|---------|
| `testing/test_framework/base_test.py` | Abstract base classes |
| `testing/tests/common_utils_test.py` | Shared utilities |
| `testing/tests/config_tests.py` | Test configuration registry |
| `testing/tests/run_mypaynow_api_tests.py` | Test runner |
| `testing/tests/test_*.py` | Individual test implementations |

## Rust Crate Dependencies

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
redis = { version = "0.23", features = ["tokio-comp"] }
mongodb = "2.7"
jsonwebtoken = "9.0"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
tracing = "0.1"
clap = { version = "4.0", features = ["derive"] }
uuid = { version = "1.0", features = ["v4"] }
async-trait = "0.1"
```

## Usage

```bash
# Run all tests
cargo test --test integration

# Run specific test path
cargo test --test integration -- --run customers.biometric.register.api

# Run all tests in a domain
cargo test --test integration -- --run customers

# Run with verbose logging
RUST_LOG=debug cargo test --test integration
```
