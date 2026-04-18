# Base Test Framework

This document details the core abstractions: Test trait, ApiTest trait, and result containers.

## TestResult - Individual Test Result Container

Captures the outcome of a single test scenario.

### Python Original

```python
class TestResult:
    def __init__(self, test_name: str, success: bool, details: str = "",
                 duration: float = 0.0, input: Dict = None, output: Dict = None):
        self.test_name = test_name
        self.success = success
        self.details = details
        self.duration = duration
        self.input = input or {}
        self.output = output or {}
        self.timestamp = datetime.now()

    def to_dict(self) -> Dict[str, Any]:
        return {
            "test_name": self.test_name,
            "success": self.success,
            "status": "PASS" if self.success else "FAIL",
            "details": self.details,
            "duration_seconds": self.duration,
            "timestamp": self.timestamp.isoformat(),
            "input": self.input,
            "output": self.output
        }
```

### Rust Implementation

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_name: String,
    pub success: bool,
    pub details: String,
    pub duration_seconds: f64,
    pub timestamp: DateTime<Utc>,
    pub input: HashMap<String, Value>,
    pub output: HashMap<String, Value>,
}

impl TestResult {
    pub fn new(test_name: impl Into<String>, success: bool) -> Self {
        Self {
            test_name: test_name.into(),
            success,
            details: String::new(),
            duration_seconds: 0.0,
            timestamp: Utc::now(),
            input: HashMap::new(),
            output: HashMap::new(),
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = details.into();
        self
    }

    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration_seconds = duration;
        self
    }

    pub fn with_input(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.input.insert(key.into(), serde_json::to_value(value).unwrap_or(Value::Null));
        self
    }

    pub fn with_output(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.output.insert(key.into(), serde_json::to_value(value).unwrap_or(Value::Null));
        self
    }

    pub fn status(&self) -> &str {
        if self.success { "PASS" } else { "FAIL" }
    }
}

// Builder pattern for creating test results
impl TestResult {
    pub fn pass(test_name: impl Into<String>) -> Self {
        Self::new(test_name, true)
    }

    pub fn fail(test_name: impl Into<String>) -> Self {
        Self::new(test_name, false)
    }
}
```

### Usage Example

```rust
// Success case
let result = TestResult::pass("Successful Registration")
    .with_details("Status: 204; Record verified in database")
    .with_duration(0.250)
    .with_input("endpoint", "/biometrics/register")
    .with_input("method", "POST")
    .with_output("status_code", 204);

// Failure case
let result = TestResult::fail("Missing Required Field")
    .with_details("Status: 400 (expected 400)")
    .with_input("missing_field", "publicKey")
    .with_output("error_code", "INVALID_REQUEST");
```

---

## TestSuiteResult - Aggregated Suite Results

Collects multiple `TestResult` instances and provides summary statistics.

### Python Original

```python
class TestSuiteResult:
    def __init__(self, suite_name: str):
        self.suite_name = suite_name
        self.start_time = datetime.now()
        self.end_time: Optional[datetime] = None
        self.results: List[TestResult] = []

    def add_result(self, result: TestResult):
        self.results.append(result)

    def finish(self):
        self.end_time = datetime.now()

    @property
    def total_duration(self) -> float:
        if self.end_time:
            return (self.end_time - self.start_time).total_seconds()
        return 0.0

    @property
    def passed_count(self) -> int:
        return sum(1 for r in self.results if r.success)

    @property
    def failed_count(self) -> int:
        return sum(1 for r in self.results if not r.success)

    @property
    def total_count(self) -> int:
        return len(self.results)

    @property
    def success_rate(self) -> float:
        if self.total_count == 0:
            return 0.0
        return (self.passed_count / self.total_count) * 100
```

### Rust Implementation

```rust
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteResult {
    pub suite_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub results: Vec<TestResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub success_rate: f64,
}

impl TestSuiteResult {
    pub fn new(suite_name: impl Into<String>) -> Self {
        Self {
            suite_name: suite_name.into(),
            start_time: Utc::now(),
            end_time: None,
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: TestResult) {
        self.results.push(result);
    }

    pub fn finish(&mut self) {
        self.end_time = Some(Utc::now());
    }

    pub fn total_duration(&self) -> Duration {
        match self.end_time {
            Some(end) => end - self.start_time,
            None => Duration::zero(),
        }
    }

    pub fn total_duration_seconds(&self) -> f64 {
        self.total_duration().num_milliseconds() as f64 / 1000.0
    }

    pub fn passed_count(&self) -> usize {
        self.results.iter().filter(|r| r.success).count()
    }

    pub fn failed_count(&self) -> usize {
        self.results.iter().filter(|r| !r.success).count()
    }

    pub fn total_count(&self) -> usize {
        self.results.len()
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_count() == 0 {
            return 0.0;
        }
        (self.passed_count() as f64 / self.total_count() as f64) * 100.0
    }

    pub fn summary(&self) -> TestSuiteSummary {
        TestSuiteSummary {
            total: self.total_count(),
            passed: self.passed_count(),
            failed: self.failed_count(),
            success_rate: self.success_rate(),
        }
    }
}
```

---

## Test Trait - Abstract Base Contract

Defines the lifecycle contract that all tests must implement.

### Python Original

```python
class Test(ABC):
    def __init__(self, name: str, results_dir: Optional[Path] = None):
        self.name = name
        self.results_dir = results_dir
        self.logger = self._setup_logger()
        self.suite_result = TestSuiteResult(name)

    @abstractmethod
    def setup(self) -> None:
        """Setup method called before test execution."""
        pass

    @abstractmethod
    def teardown(self) -> None:
        """Teardown method called after test execution."""
        pass

    @abstractmethod
    def run_tests(self) -> List[TestResult]:
        """Execute all tests and return results."""
        pass

    def execute(self) -> TestSuiteResult:
        """Main execution method - the contract interface."""
        self.logger.info(f"Starting test suite: {self.name}")

        try:
            self.logger.info("Running setup...")
            self.setup()

            self.logger.info("Executing tests...")
            results = self.run_tests()

            for result in results:
                self.suite_result.add_result(result)

        except Exception as e:
            self.logger.error(f"Test execution failed: {e}")
            error_result = TestResult(
                test_name=f"{self.name}_execution_error",
                success=False,
                details=f"Test execution failed: {str(e)}"
            )
            self.suite_result.add_result(error_result)

        finally:
            try:
                self.logger.info("Running teardown...")
                self.teardown()
            except Exception as e:
                self.logger.error(f"Teardown failed: {e}")

            self.suite_result.finish()

        if self.results_dir:
            self._save_results()

        return self.suite_result
```

### Rust Implementation

```rust
use std::path::PathBuf;
use tracing::{info, error};
use anyhow::Result;

/// Test lifecycle errors
#[derive(Debug, thiserror::Error)]
pub enum TestError {
    #[error("Setup failed: {0}")]
    SetupError(String),

    #[error("Teardown failed: {0}")]
    TeardownError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Abstract test trait - all tests must implement this
pub trait Test {
    /// Get the test suite name
    fn name(&self) -> &str;

    /// Get optional results directory
    fn results_dir(&self) -> Option<&PathBuf>;

    /// Setup method called before test execution
    fn setup(&mut self) -> Result<(), TestError>;

    /// Execute all test scenarios and return results
    fn run_tests(&mut self) -> Vec<TestResult>;

    /// Teardown method called after test execution
    fn teardown(&mut self) -> Result<(), TestError>;

    /// Main execution method - orchestrates the test lifecycle
    /// Default implementation provided
    fn execute(&mut self) -> TestSuiteResult {
        let mut suite_result = TestSuiteResult::new(self.name());

        info!("Starting test suite: {}", self.name());

        // Setup phase
        match self.setup() {
            Ok(()) => info!("Setup completed successfully"),
            Err(e) => {
                error!("Setup failed: {}", e);
                suite_result.add_result(
                    TestResult::fail(format!("{}_setup_error", self.name()))
                        .with_details(format!("Setup failed: {}", e))
                );
                suite_result.finish();
                return suite_result;
            }
        }

        // Execute tests
        info!("Executing tests...");
        let results = self.run_tests();

        for result in results {
            suite_result.add_result(result);
        }

        // Teardown phase (always runs)
        info!("Running teardown...");
        if let Err(e) = self.teardown() {
            error!("Teardown failed: {}", e);
            // Don't add teardown failure as test result, just log it
        }

        suite_result.finish();

        // Save results if directory provided
        if let Some(results_dir) = self.results_dir() {
            if let Err(e) = self.save_results(&suite_result, results_dir) {
                error!("Failed to save results: {}", e);
            }
        }

        info!(
            "Test suite completed: {}/{} passed",
            suite_result.passed_count(),
            suite_result.total_count()
        );

        suite_result
    }

    /// Save test results to JSON files
    fn save_results(&self, suite_result: &TestSuiteResult, results_dir: &PathBuf) -> Result<(), TestError> {
        std::fs::create_dir_all(results_dir)?;

        // Save main results file
        let results_file = results_dir.join(format!("{}_results.json", self.name()));
        let json = serde_json::to_string_pretty(suite_result)
            .map_err(|e| TestError::ExecutionError(e.to_string()))?;
        std::fs::write(&results_file, json)?;

        // Save individual test case files
        for result in &suite_result.results {
            let test_file_name = result.test_name
                .replace(' ', "_")
                .replace('-', "_")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '_')
                .collect::<String>();

            let test_file = results_dir.join(format!("{}.json", test_file_name));
            let json = serde_json::to_string_pretty(result)
                .map_err(|e| TestError::ExecutionError(e.to_string()))?;
            std::fs::write(&test_file, json)?;
        }

        info!("Results saved to: {:?}", results_file);
        Ok(())
    }
}
```

---

## ApiTest Trait - HTTP Testing Extension

Extends `Test` with HTTP request capabilities.

### Python Original

```python
class ApiTest(Test):
    def __init__(self, name: str, api_base_url: str, results_dir: Optional[Path] = None):
        super().__init__(name, results_dir)
        self.api_base_url = api_base_url
        self.session = None

    def setup(self) -> None:
        """Setup HTTP session for API testing."""
        self.session = requests.Session()
        headers = {
            'Content-Type': 'application/json',
            'User-Agent': f'MPN-Test-{self.name}/1.0'
        }
        # Add X-Origin-Verify header for local testing
        x_origin_verify = os.getenv('X_ORIGIN_VERIFY')
        if x_origin_verify:
            headers['X-Origin-Verify'] = x_origin_verify
        self.session.headers.update(headers)

    def teardown(self) -> None:
        """Cleanup HTTP session."""
        if self.session:
            self.session.close()

    def make_request(self, method: str, endpoint: str, **kwargs) -> Dict[str, Any]:
        """Make HTTP request to API endpoint."""
        url = f"{self.api_base_url}{endpoint}"

        try:
            response = self.session.request(method, url, timeout=60, **kwargs)

            return {
                "success": True,
                "status_code": response.status_code,
                "headers": dict(response.headers),
                "body": response.text,
                "url": url
            }
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "url": url
            }
```

### Rust Implementation

```rust
use reqwest::{Client, Method, header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT}};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

/// API response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub status_code: Option<u16>,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub url: String,
    pub error: Option<String>,
}

impl ApiResponse {
    pub fn parse_body<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.body)
    }

    pub fn parse_body_value(&self) -> Result<Value, serde_json::Error> {
        if self.body.is_empty() {
            Ok(Value::Null)
        } else {
            serde_json::from_str(&self.body)
        }
    }
}

/// Configuration for API tests
#[derive(Debug, Clone)]
pub struct ApiTestConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub x_origin_verify: Option<String>,
}

impl Default for ApiTestConfig {
    fn default() -> Self {
        Self {
            base_url: std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string()),
            timeout_seconds: 60,
            x_origin_verify: std::env::var("X_ORIGIN_VERIFY").ok(),
        }
    }
}

/// API test trait - extends Test with HTTP capabilities
#[async_trait::async_trait]
pub trait ApiTest: Test {
    /// Get the API base URL
    fn api_base_url(&self) -> &str;

    /// Get the HTTP client
    fn client(&self) -> &Client;

    /// Make an HTTP request to an API endpoint
    async fn make_request(
        &self,
        method: Method,
        endpoint: &str,
        headers: Option<HeaderMap>,
        body: Option<Value>,
    ) -> ApiResponse {
        let url = format!("{}{}", self.api_base_url(), endpoint);

        let mut request = self.client()
            .request(method, &url)
            .timeout(Duration::from_secs(60));

        if let Some(hdrs) = headers {
            request = request.headers(hdrs);
        }

        if let Some(json_body) = body {
            request = request.json(&json_body);
        }

        match request.send().await {
            Ok(response) => {
                let status_code = response.status().as_u16();
                let headers: HashMap<String, String> = response
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect();
                let body = response.text().await.unwrap_or_default();

                ApiResponse {
                    success: true,
                    status_code: Some(status_code),
                    headers,
                    body,
                    url,
                    error: None,
                }
            }
            Err(e) => ApiResponse {
                success: false,
                status_code: None,
                headers: HashMap::new(),
                body: String::new(),
                url,
                error: Some(e.to_string()),
            },
        }
    }

    /// Create default headers for requests
    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&format!("Test-{}/1.0", self.name())).unwrap()
        );

        if let Ok(origin_verify) = std::env::var("X_ORIGIN_VERIFY") {
            if let Ok(value) = HeaderValue::from_str(&origin_verify) {
                headers.insert("X-Origin-Verify", value);
            }
        }

        headers
    }

    /// Create headers with authorization
    fn auth_headers(&self, token: &str) -> HeaderMap {
        let mut headers = self.default_headers();
        if let Ok(value) = HeaderValue::from_str(&format!("Bearer {}", token)) {
            headers.insert("Authorization", value);
        }
        headers
    }
}

/// Base implementation struct for API tests
pub struct BaseApiTest {
    pub name: String,
    pub api_base_url: String,
    pub client: Client,
    pub results_dir: Option<PathBuf>,
}

impl BaseApiTest {
    pub fn new(name: impl Into<String>, config: ApiTestConfig) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(ref origin_verify) = config.x_origin_verify {
            if let Ok(value) = HeaderValue::from_str(origin_verify) {
                default_headers.insert("X-Origin-Verify", value);
            }
        }

        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .default_headers(default_headers)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            name: name.into(),
            api_base_url: config.base_url,
            client,
            results_dir: None,
        }
    }

    pub fn with_results_dir(mut self, dir: PathBuf) -> Self {
        self.results_dir = Some(dir);
        self
    }
}
```

### Usage Example

```rust
pub struct BiometricRegisterApiTest {
    base: BaseApiTest,
    setup_manager: TestSetupManager,
    jwt_manager: JwtTokenManager,
    test_installation_ids: Vec<String>,
}

impl Test for BiometricRegisterApiTest {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn results_dir(&self) -> Option<&PathBuf> {
        self.base.results_dir.as_ref()
    }

    fn setup(&mut self) -> Result<(), TestError> {
        // Initialize connections
        self.setup_manager.setup()?;
        Ok(())
    }

    fn run_tests(&mut self) -> Vec<TestResult> {
        vec![
            self.test_unauthorized_access(),
            self.test_invalid_token(),
            self.test_successful_registration(),
            self.test_missing_required_field(),
        ]
    }

    fn teardown(&mut self) -> Result<(), TestError> {
        // Cleanup test records
        for id in &self.test_installation_ids {
            // cleanup logic
        }
        Ok(())
    }
}

impl ApiTest for BiometricRegisterApiTest {
    fn api_base_url(&self) -> &str {
        &self.base.api_base_url
    }

    fn client(&self) -> &Client {
        &self.base.client
    }
}
```
