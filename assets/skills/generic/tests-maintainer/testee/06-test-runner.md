# Test Runner and Orchestration

This document details the test runner that orchestrates test execution, manages results, and provides reporting.

## Test Runner Responsibilities

1. **Parse Configuration** - Load test hierarchy and build registry
2. **Parse CLI Arguments** - Handle `--run` parameter for selective testing
3. **Check Prerequisites** - Verify API availability before API tests
4. **Execute Tests** - Run selected tests with proper setup/teardown
5. **Aggregate Results** - Collect individual results into suite results
6. **Generate Reports** - Save JSON reports and print summary

---

## Python Original

```python
class TestRunner:
    """Orchestrates API testing with SAM local environment."""

    def __init__(self):
        self.api_url = "http://127.0.0.1:3000"
        self.results_dir: Optional[Path] = None
        self.test_results: List[Dict[str, Any]] = []
        self.failed_tests = 0
        self.total_tests = 0
        self.run_parameter = None

        self.test_config = TEST_CONFIG
        self.test_scripts = self._build_test_scripts_from_config()
        self.test_registry = self._build_test_registry_from_config()

    def setup_results_directory(self) -> Path:
        """Create timestamped results directory."""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        results_dir = Path(__file__).parent.parent / "results" / timestamp
        results_dir.mkdir(parents=True, exist_ok=True)
        self.results_dir = results_dir
        return results_dir

    def check_api_requirements(self, scripts_to_run: List[str]) -> bool:
        """Check if any tests require API and if API is ready."""
        requires_api = False
        for script in scripts_to_run:
            if script in self.test_registry:
                test_config = self.test_registry[script]
                if test_config.get("requires_api", True):
                    requires_api = True
                    break

        if requires_api:
            return self.wait_for_api_ready(timeout=60)
        return True

    def wait_for_api_ready(self, timeout: int = 60) -> bool:
        """Wait for API to be ready."""
        import requests
        start_time = time.time()

        while time.time() - start_time < timeout:
            try:
                response = requests.get(f"{self.api_url}/health", timeout=5)
                if response.status_code in [200, 400, 401, 403]:
                    return True
            except (requests.ConnectionError, requests.Timeout):
                pass
            time.sleep(2)

        return False

    def parse_run_parameter(self, run_param: Optional[str]) -> List[str]:
        """Parse --run parameter and return list of test scripts."""
        if not run_param:
            return self.get_all_test_scripts()

        parts = run_param.split('.')
        scripts = []

        def traverse_config(config: Dict, path_parts: List[str]) -> List[str]:
            if not path_parts:
                return self.collect_scripts_from_config(config)

            current_part = path_parts[0]
            remaining_parts = path_parts[1:]

            if current_part in config:
                if isinstance(config[current_part], str):
                    return [config[current_part]]
                elif isinstance(config[current_part], dict):
                    return traverse_config(config[current_part], remaining_parts)

            return []

        scripts = traverse_config(self.test_scripts, parts)
        return scripts

    def run_test_script(self, script_name: str) -> Dict[str, Any]:
        """Run a single test script and return results."""
        start_time = datetime.now()

        # Get test configuration
        test_config = self.test_registry[script_name]
        module_name = test_config["module"]
        class_name = test_config["class"]
        results_subdir = test_config.get("results_subdir", script_name.replace(".py", ""))

        # Dynamically import and instantiate test
        import importlib
        test_module = importlib.import_module(module_name)
        test_class = getattr(test_module, class_name)

        # Execute test
        results_path = Path(self.results_dir) / results_subdir
        test_instance = test_class(results_dir=results_path)
        suite_result = test_instance.execute()

        # Process results
        success = suite_result.failed_count == 0
        status = "PASS" if success else "FAIL"

        end_time = datetime.now()
        duration = (end_time - start_time).total_seconds()

        return {
            "script": script_name,
            "status": status,
            "return_code": 0 if success else 1,
            "start_time": start_time.isoformat(),
            "end_time": end_time.isoformat(),
            "duration": duration,
            "test_details": {
                "total": suite_result.total_count,
                "passed": suite_result.passed_count,
                "failed": suite_result.failed_count,
                "success_rate": suite_result.success_rate
            }
        }

    def generate_aggregate_results(self):
        """Generate aggregate results JSON file."""
        total_test_cases = 0
        passed_test_cases = 0
        failed_test_cases = 0

        for test_result in self.test_results:
            if "test_details" in test_result:
                details = test_result["test_details"]
                total_test_cases += details.get("total", 0)
                passed_test_cases += details.get("passed", 0)
                failed_test_cases += details.get("failed", 0)

        aggregate = {
            "test_run": {
                "timestamp": datetime.now().isoformat(),
                "api_url": self.api_url,
                "results_directory": str(self.results_dir)
            },
            "summary": {
                "total_tests": total_test_cases,
                "passed": passed_test_cases,
                "failed": failed_test_cases,
                "success_rate": (passed_test_cases / total_test_cases * 100) if total_test_cases > 0 else 0
            },
            "test_results": self.test_results
        }

        results_file = self.results_dir / "results.json"
        with open(results_file, 'w') as f:
            json.dump(aggregate, f, indent=2)

    def run_tests(self, run_param: Optional[str] = None):
        """Main test execution flow."""
        self.run_parameter = run_param

        # Setup
        self.setup_results_directory()

        # Parse tests
        scripts_to_run = self.parse_run_parameter(run_param)
        if not scripts_to_run:
            raise RuntimeError("No test scripts to run")

        # Check API requirements
        if not self.check_api_requirements(scripts_to_run):
            raise RuntimeError("API not ready")

        # Run tests
        for script in scripts_to_run:
            result = self.run_test_script(script)
            self.test_results.append(result)

        # Generate results
        self.generate_aggregate_results()

        # Print summary
        # ...

        # Exit with appropriate code
        if self.failed_tests > 0:
            raise SystemExit(1)
```

---

## Rust Implementation

```rust
use crate::config::{TestConfig, TestRegistry, TestRegistryEntry, TEST_CONFIG};
use crate::framework::{Test, TestSuiteResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Test execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionResult {
    pub script: String,
    pub status: String,
    pub return_code: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration_seconds: f64,
    pub test_details: TestDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDetails {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub success_rate: f64,
}

/// Aggregate test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateResults {
    pub test_run: TestRunInfo,
    pub summary: TestSummary,
    pub test_results: Vec<TestExecutionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunInfo {
    pub timestamp: DateTime<Utc>,
    pub api_url: String,
    pub results_directory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub success_rate: f64,
}

/// Test factory function type
type TestFactory = Box<dyn Fn(Option<PathBuf>) -> Box<dyn Test> + Send + Sync>;

/// Test runner that orchestrates test execution
pub struct TestRunner {
    api_url: String,
    admin_api_url: String,
    results_dir: Option<PathBuf>,
    test_results: Vec<TestExecutionResult>,
    config: &'static TestConfig,
    registry: TestRegistry,
    factories: HashMap<String, TestFactory>,
}

impl TestRunner {
    pub fn new() -> Self {
        let config = &*TEST_CONFIG;
        let registry = TestRegistry::from_config(config);

        Self {
            api_url: std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string()),
            admin_api_url: std::env::var("ADMIN_API_BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:3001".to_string()),
            results_dir: None,
            test_results: Vec::new(),
            config,
            registry,
            factories: HashMap::new(),
        }
    }

    /// Register a test factory
    pub fn register_test<T, F>(&mut self, module: &str, factory: F)
    where
        T: Test + 'static,
        F: Fn(Option<PathBuf>) -> T + Send + Sync + 'static,
    {
        self.factories.insert(
            module.to_string(),
            Box::new(move |dir| Box::new(factory(dir)) as Box<dyn Test>),
        );
    }

    /// Setup timestamped results directory
    pub fn setup_results_directory(&mut self) -> PathBuf {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let results_dir = PathBuf::from("testing")
            .join("results")
            .join(&timestamp);

        std::fs::create_dir_all(&results_dir).expect("Failed to create results directory");
        self.results_dir = Some(results_dir.clone());
        results_dir
    }

    /// Check if API is ready
    pub async fn wait_for_api_ready(&self, url: &str, timeout_seconds: u64) -> bool {
        let client = reqwest::Client::new();
        let start = Instant::now();
        let timeout = Duration::from_secs(timeout_seconds);

        while start.elapsed() < timeout {
            match client.get(&format!("{}/health", url))
                .timeout(Duration::from_secs(5))
                .send()
                .await
            {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    if [200, 400, 401, 403].contains(&status) {
                        return true;
                    }
                }
                Err(_) => {}
            }

            tokio::time::sleep(Duration::from_secs(2)).await;
            print!(".");
        }

        println!();
        false
    }

    /// Check API requirements for tests
    pub async fn check_api_requirements(&self, entries: &[&TestRegistryEntry]) -> bool {
        let needs_api = entries.iter().any(|e| e.requires_api);

        if needs_api {
            println!("Checking API availability at {}...", self.api_url);
            self.wait_for_api_ready(&self.api_url, 60).await
        } else {
            true
        }
    }

    /// Parse run parameter and get matching test entries
    pub fn parse_run_parameter(&self, run_param: Option<&str>) -> Vec<&TestRegistryEntry> {
        match run_param {
            None => {
                // Run all tests
                self.registry.all_entries().collect()
            }
            Some(param) => {
                // Try exact match first
                if let Some(info) = self.config.get_test_info(param) {
                    if let Some(entry) = self.registry.get_by_module(&info.module) {
                        return vec![entry];
                    }
                }

                // Try prefix match
                self.registry
                    .all_entries()
                    .filter(|e| e.path.starts_with(param))
                    .collect()
            }
        }
    }

    /// Run a single test
    pub fn run_test(&mut self, entry: &TestRegistryEntry) -> TestExecutionResult {
        let start_time = Utc::now();
        let start_instant = Instant::now();

        println!("\nRunning test: {} ({})", entry.path, entry.struct_name);

        // Get test factory
        let factory = match self.factories.get(&entry.module) {
            Some(f) => f,
            None => {
                let end_time = Utc::now();
                let duration = start_instant.elapsed().as_secs_f64();

                return TestExecutionResult {
                    script: entry.path.clone(),
                    status: "ERROR".to_string(),
                    return_code: -1,
                    start_time,
                    end_time,
                    duration_seconds: duration,
                    test_details: TestDetails {
                        total: 0,
                        passed: 0,
                        failed: 1,
                        success_rate: 0.0,
                    },
                };
            }
        };

        // Create test instance
        let results_path = self.results_dir.as_ref().map(|d| d.join(&entry.results_subdir));
        let mut test = factory(results_path);

        // Execute test
        let suite_result = test.execute();

        let end_time = Utc::now();
        let duration = start_instant.elapsed().as_secs_f64();

        let success = suite_result.failed_count() == 0;
        let status = if success { "PASS" } else { "FAIL" };

        println!(
            "[{}] {} - {}/{} tests passed",
            status,
            entry.path,
            suite_result.passed_count(),
            suite_result.total_count()
        );

        TestExecutionResult {
            script: entry.path.clone(),
            status: status.to_string(),
            return_code: if success { 0 } else { 1 },
            start_time,
            end_time,
            duration_seconds: duration,
            test_details: TestDetails {
                total: suite_result.total_count(),
                passed: suite_result.passed_count(),
                failed: suite_result.failed_count(),
                success_rate: suite_result.success_rate(),
            },
        }
    }

    /// Generate aggregate results JSON
    pub fn generate_aggregate_results(&self) -> AggregateResults {
        let mut total = 0;
        let mut passed = 0;
        let mut failed = 0;

        for result in &self.test_results {
            total += result.test_details.total;
            passed += result.test_details.passed;
            failed += result.test_details.failed;
        }

        let success_rate = if total > 0 {
            (passed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        AggregateResults {
            test_run: TestRunInfo {
                timestamp: Utc::now(),
                api_url: self.api_url.clone(),
                results_directory: self.results_dir
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default(),
            },
            summary: TestSummary {
                total_tests: total,
                passed,
                failed,
                success_rate,
            },
            test_results: self.test_results.clone(),
        }
    }

    /// Save aggregate results to file
    pub fn save_results(&self, aggregate: &AggregateResults) -> Result<PathBuf, std::io::Error> {
        let results_dir = self.results_dir.as_ref().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Results directory not set")
        })?;

        let results_file = results_dir.join("results.json");
        let json = serde_json::to_string_pretty(aggregate)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        std::fs::write(&results_file, json)?;
        Ok(results_file)
    }

    /// Print test summary
    pub fn print_summary(&self, aggregate: &AggregateResults) {
        println!("\n{}", "=".repeat(60));
        println!("TEST SUMMARY");
        println!("{}", "=".repeat(60));
        println!("Total Tests: {}", aggregate.summary.total_tests);
        println!("Passed: {}", aggregate.summary.passed);
        println!("Failed: {}", aggregate.summary.failed);
        println!("Success Rate: {:.1}%", aggregate.summary.success_rate);
        if let Some(ref dir) = self.results_dir {
            println!("Results: {}/results.json", dir.display());
        }
        println!("{}", "=".repeat(60));
    }

    /// Main test execution flow
    pub async fn run(&mut self, run_param: Option<&str>) -> Result<(), RunnerError> {
        // Setup results directory
        self.setup_results_directory();

        // Parse run parameter
        let entries = self.parse_run_parameter(run_param);
        if entries.is_empty() {
            return Err(RunnerError::NoTests);
        }

        println!("Found {} test(s) to run", entries.len());

        // Check API requirements
        if !self.check_api_requirements(&entries).await {
            return Err(RunnerError::ApiNotReady(self.api_url.clone()));
        }

        // Run tests
        for entry in entries {
            let result = self.run_test(entry);
            self.test_results.push(result);
        }

        // Generate and save results
        let aggregate = self.generate_aggregate_results();

        if let Err(e) = self.save_results(&aggregate) {
            eprintln!("Warning: Failed to save results: {}", e);
        }

        // Print summary
        self.print_summary(&aggregate);

        // Return error if any tests failed
        if aggregate.summary.failed > 0 {
            println!("\n[FAILED] {} test(s) failed", aggregate.summary.failed);
            Err(RunnerError::TestsFailed(aggregate.summary.failed))
        } else {
            println!("\n[SUCCESS] All tests passed!");
            Ok(())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RunnerError {
    #[error("No tests found to run")]
    NoTests,

    #[error("API not ready at {0}")]
    ApiNotReady(String),

    #[error("{0} test(s) failed")]
    TestsFailed(usize),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

---

## CLI Entry Point

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "test-runner")]
#[command(about = "MyPayNow API Test Runner")]
struct Cli {
    /// Test path to run (e.g., customers.profile.api)
    #[arg(long)]
    run: Option<String>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Create runner and register tests
    let mut runner = TestRunner::new();

    // Register all test factories
    runner.register_test("biometric::register_api_test", |dir| {
        RegisterBiometricApiTest::new(dir)
    });
    runner.register_test("customers::profile_api_test", |dir| {
        CustomerProfileApiTest::new(dir)
    });
    // ... register more tests

    // Run tests
    match runner.run(cli.run.as_deref()).await {
        Ok(()) => std::process::exit(0),
        Err(RunnerError::TestsFailed(_)) => std::process::exit(1),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(2);
        }
    }
}
```

---

## Usage Examples

```bash
# Run all tests
cargo test --test integration

# Run specific test by path
cargo test --test integration -- --run customers.biometric.register.api

# Run all customer tests
cargo test --test integration -- --run customers

# Run profile tests (api and admin_api)
cargo test --test integration -- --run customers.profile
```

---

## Results Directory Structure

```
testing/results/
└── 20231212_143052/
    ├── results.json              # Aggregate results
    ├── register_biometric_api/
    │   ├── register_biometric_api.log
    │   ├── register_biometric_api_results.json
    │   ├── Unauthorized_Access.json
    │   ├── Invalid_Token.json
    │   ├── Successful_Registration.json
    │   └── ...
    ├── customer_profile_api/
    │   ├── customer_profile_api.log
    │   ├── customer_profile_api_results.json
    │   └── ...
    └── ...
```

---

## Aggregate Results Format

```json
{
  "test_run": {
    "timestamp": "2023-12-12T14:30:52.123456Z",
    "api_url": "http://127.0.0.1:3000",
    "results_directory": "testing/results/20231212_143052"
  },
  "summary": {
    "total_tests": 45,
    "passed": 43,
    "failed": 2,
    "success_rate": 95.56
  },
  "test_results": [
    {
      "script": "customers.biometric.register.api",
      "status": "PASS",
      "return_code": 0,
      "start_time": "2023-12-12T14:30:52.123456Z",
      "end_time": "2023-12-12T14:31:05.654321Z",
      "duration_seconds": 13.530865,
      "test_details": {
        "total": 8,
        "passed": 8,
        "failed": 0,
        "success_rate": 100.0
      }
    },
    {
      "script": "customers.profile.api",
      "status": "FAIL",
      "return_code": 1,
      "start_time": "2023-12-12T14:31:05.654321Z",
      "end_time": "2023-12-12T14:31:45.987654Z",
      "duration_seconds": 40.333333,
      "test_details": {
        "total": 8,
        "passed": 6,
        "failed": 2,
        "success_rate": 75.0
      }
    }
  ]
}
```
