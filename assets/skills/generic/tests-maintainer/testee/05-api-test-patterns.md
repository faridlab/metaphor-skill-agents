# API Test Patterns

This document shows complete test implementations with various patterns for API testing.

## Test Scenario Categories

Every API endpoint should have tests covering:

1. **Authentication Tests**
   - `test_unauthorized_access()` - No auth header → 401
   - `test_invalid_token()` - Invalid JWT → 403

2. **Success Tests**
   - `test_successful_operation()` - Valid request → expected response
   - Database verification
   - Cache verification

3. **Validation Tests**
   - `test_missing_required_field()` - Missing fields → 400
   - `test_invalid_field_value()` - Invalid values → 400
   - `test_invalid_format()` - Wrong format → 400

4. **Business Logic Tests**
   - `test_conflict()` - Duplicate/conflict → 400/409
   - `test_not_found()` - Resource not found → 404
   - `test_idempotency()` - Repeated requests → same result

5. **Edge Cases**
   - Boundary values
   - Empty strings
   - Special characters

---

## Complete Example: Biometric Registration API Test

### Python Original

```python
class RegisterBiometricApiTest(ApiTest):
    """API test for biometric device registration functionality."""

    # Test data constants
    VALID_PUBLIC_KEY = """-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAokIfSx4PP...
-----END PUBLIC KEY-----"""

    WEAK_PUBLIC_KEY = """-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC7vbqajDw4o6gW...
-----END PUBLIC KEY-----"""

    INVALID_PUBLIC_KEY = """-----BEGIN PUBLIC KEY-----
INVALID_KEY_FORMAT
-----END PUBLIC KEY-----"""

    def __init__(self, api_base_url: str = None, results_dir: Path = None):
        api_url = api_base_url or os.getenv('API_BASE_URL', 'http://127.0.0.1:3000')
        super().__init__("register_biometric_api", api_url, results_dir)

        self.endpoint = "/biometrics/register"
        self.setup_manager = TestSetupManager("test_register_biometric_api")
        self.jwt_manager = JWTTokenManager()
        self.test_installation_ids = []  # Track for cleanup

    def _create_auth_token(self, user_id: str, token_id: str) -> str:
        """Create a JWT token and store session in Redis."""
        self.redis_session_manager.store_jwt_session(user_id, token_id, ttl=180)
        token, _ = self.jwt_manager.create_jwt_token(user_id, token_id)
        return token

    def _generate_installation_id(self) -> str:
        """Generate a unique installation ID."""
        return base64.b64encode(uuid.uuid4().bytes).decode('utf-8')

    def _create_valid_request_body(self, installation_id: str = None) -> Dict[str, Any]:
        """Create a valid biometric registration request body."""
        if installation_id:
            self.test_installation_ids.append(installation_id)
        return {
            "publicKey": self.VALID_PUBLIC_KEY,
            "algorithm": "RSA",
            "installationId": installation_id or self._generate_installation_id(),
            "appVersion": "1.0.0",
            "appBuildNumber": 1
        }

    def _verify_biometric_in_database(self, installation_id: str, user_id: str) -> bool:
        """Verify that biometric record was saved to MongoDB."""
        record = self.mongodb_client[self.db_name].biometricAuth.find_one({"_id": installation_id})
        return record and record.get("userId") == user_id

    def setup(self) -> None:
        """Setup API session and Redis connection."""
        setup_result = self.setup_manager.setup()
        self.redis_client = setup_result["redis_client"]
        self.auth_cache = setup_result["auth_cache"]
        self.redis_session_manager = RedisSessionManager(self.redis_client, self.auth_cache)
        self.setup_mongodb()
        super().setup()

    def teardown(self) -> None:
        """Cleanup biometric records from MongoDB."""
        for installation_id in self.test_installation_ids:
            self.mongodb_client[self.db_name].biometricAuth.delete_one({"_id": installation_id})
        super().teardown()

    def run_tests(self) -> List[TestResult]:
        """Execute all biometric registration API tests."""
        return [
            self.test_unauthorized_access(),
            self.test_invalid_token(),
            self.test_successful_registration(),
            self.test_missing_required_field(),
            self.test_invalid_algorithm(),
            self.test_invalid_public_key(),
            self.test_weak_key_size(),
            self.test_upsert_registration(),
        ]

    # ==================== Test Methods ====================

    def test_unauthorized_access(self) -> TestResult:
        """Test that endpoint requires authentication."""
        request_body = self._create_valid_request_body()

        response = self.make_request("POST", self.endpoint, json=request_body)

        success = response.get("status_code") == 401

        return TestResult(
            test_name="Unauthorized Access",
            success=success,
            details=f"Status: {response.get('status_code')} (expected 401)",
            input={"endpoint": self.endpoint, "method": "POST", "headers": "None"},
            output={"response": response}
        )

    def test_invalid_token(self) -> TestResult:
        """Test that invalid JWT token is rejected."""
        request_body = self._create_valid_request_body()

        response = self.make_request(
            "POST", self.endpoint,
            headers={"Authorization": "Bearer invalid.jwt.token"},
            json=request_body
        )

        success = response.get("status_code") == 403

        return TestResult(
            test_name="Invalid Token",
            success=success,
            details=f"Status: {response.get('status_code')} (expected 403)",
            input={"endpoint": self.endpoint, "method": "POST"},
            output={"response": response}
        )

    def test_successful_registration(self) -> TestResult:
        """Test successful biometric registration with valid data."""
        user_id = "biometric-test-user-001"
        token_id = "biometric-test-token-001"
        token = self._create_auth_token(user_id, token_id)

        installation_id = self._generate_installation_id()
        request_body = self._create_valid_request_body(installation_id)

        response = self.make_request(
            "POST", self.endpoint,
            headers={"Authorization": f"Bearer {token}"},
            json=request_body
        )

        success = response.get("status_code") == 204
        validation_details = []

        if not success:
            validation_details.append(f"Status: {response.get('status_code')} (expected 204)")
        else:
            validation_details.append("Status: 204")

            # Verify record in database
            if self._verify_biometric_in_database(installation_id, user_id):
                validation_details.append("Record verified in database")
            else:
                success = False
                validation_details.append("Record not found in database")

        return TestResult(
            test_name="Successful Registration",
            success=success,
            details="; ".join(validation_details),
            input={"endpoint": self.endpoint, "method": "POST", "installationId": installation_id},
            output={"response": response}
        )

    def test_missing_required_field(self) -> TestResult:
        """Test that missing required fields return 400."""
        token = self._create_auth_token("biometric-test-user-002", "biometric-test-token-002")

        # Missing publicKey field
        request_body = {
            "algorithm": "RSA",
            "installationId": self._generate_installation_id(),
            "appVersion": "1.0.0",
            "appBuildNumber": 1
        }

        response = self.make_request(
            "POST", self.endpoint,
            headers={"Authorization": f"Bearer {token}"},
            json=request_body
        )

        success = response.get("status_code") == 400

        return TestResult(
            test_name="Missing Required Field (publicKey)",
            success=success,
            details=f"Status: {response.get('status_code')} (expected 400)",
            input={"endpoint": self.endpoint, "missing_field": "publicKey"},
            output={"response": response}
        )

    def test_upsert_registration(self) -> TestResult:
        """Test that re-registration (upsert) overwrites existing record."""
        user_id = "biometric-test-user-006"
        token = self._create_auth_token(user_id, "biometric-test-token-006")
        installation_id = self._generate_installation_id()

        # First registration
        request_body1 = self._create_valid_request_body(installation_id)
        request_body1["appVersion"] = "1.0.0"
        response1 = self.make_request("POST", self.endpoint,
                                       headers={"Authorization": f"Bearer {token}"},
                                       json=request_body1)

        # Second registration (should upsert)
        request_body2 = self._create_valid_request_body(installation_id)
        request_body2["appVersion"] = "1.1.0"
        response2 = self.make_request("POST", self.endpoint,
                                       headers={"Authorization": f"Bearer {token}"},
                                       json=request_body2)

        success = (response1.get("status_code") == 204 and
                   response2.get("status_code") == 204)

        validation_details = []
        if success:
            # Verify the record was updated
            record = self.mongodb_client[self.db_name].biometricAuth.find_one({"_id": installation_id})
            if record and record.get("appVersion") == "1.1.0":
                validation_details.append("Record successfully updated (upsert)")
            else:
                success = False
                validation_details.append("Record was not properly updated")

        return TestResult(
            test_name="Upsert Registration",
            success=success,
            details="; ".join(validation_details),
            input={"endpoint": self.endpoint, "installationId": installation_id},
            output={"first_response": response1, "second_response": response2}
        )
```

### Rust Implementation

```rust
use crate::framework::{ApiTest, BaseApiTest, Test, TestError, TestResult};
use crate::utils::{
    CommonUtils, JwtTokenManager, MongoDbManager, RedisSessionManager, TestSetupManager,
};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::{header::HeaderMap, Method};
use serde_json::json;
use std::path::PathBuf;
use uuid::Uuid;

/// Test data constants
const VALID_PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAokIfSx4PP...
-----END PUBLIC KEY-----"#;

const WEAK_PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC7vbqajDw4o6gW...
-----END PUBLIC KEY-----"#;

const INVALID_PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
INVALID_KEY_FORMAT
-----END PUBLIC KEY-----"#;

/// Biometric Registration API Test
pub struct RegisterBiometricApiTest {
    // Base test infrastructure
    base: BaseApiTest,
    setup_manager: TestSetupManager,
    jwt_manager: JwtTokenManager,
    redis_manager: Option<RedisSessionManager>,
    mongo_manager: Option<MongoDbManager>,
    common_utils: Option<CommonUtils>,

    // Test state
    endpoint: String,
    test_installation_ids: Vec<String>,
}

impl RegisterBiometricApiTest {
    pub fn new(results_dir: Option<PathBuf>) -> Self {
        let config = ApiTestConfig::default();
        let base = BaseApiTest::new("register_biometric_api", config);

        Self {
            base: if let Some(dir) = results_dir {
                base.with_results_dir(dir)
            } else {
                base
            },
            setup_manager: TestSetupManager::new("test_register_biometric_api"),
            jwt_manager: JwtTokenManager::new(),
            redis_manager: None,
            mongo_manager: None,
            common_utils: None,
            endpoint: "/biometrics/register".to_string(),
            test_installation_ids: Vec::new(),
        }
    }

    // ==================== Helper Methods ====================

    fn generate_installation_id(&mut self) -> String {
        let id = STANDARD.encode(Uuid::new_v4().as_bytes());
        self.test_installation_ids.push(id.clone());
        id
    }

    fn create_valid_request_body(&mut self, installation_id: Option<String>) -> serde_json::Value {
        let id = installation_id.unwrap_or_else(|| self.generate_installation_id());

        json!({
            "publicKey": VALID_PUBLIC_KEY,
            "algorithm": "RSA",
            "installationId": id,
            "appVersion": "1.0.0",
            "appBuildNumber": 1
        })
    }

    async fn create_auth_token(&self, user_id: &str, token_id: &str) -> Result<String, TestError> {
        let redis = self.redis_manager.as_ref()
            .ok_or_else(|| TestError::SetupError("Redis not initialized".into()))?;

        redis.store_jwt_session(user_id, token_id, 180)
            .map_err(|e| TestError::SetupError(e.to_string()))?;

        let (token, _) = self.jwt_manager.create_user_token(user_id, token_id)
            .map_err(|e| TestError::SetupError(e.to_string()))?;

        Ok(token)
    }

    async fn verify_biometric_in_database(&self, installation_id: &str, user_id: &str) -> bool {
        let mongo = match self.mongo_manager.as_ref() {
            Some(m) => m,
            None => return false,
        };

        match mongo.find_one("biometricAuth", doc! { "_id": installation_id }).await {
            Ok(Some(record)) => record.get_str("userId").ok() == Some(user_id),
            _ => false,
        }
    }

    // ==================== Test Methods ====================

    async fn test_unauthorized_access(&mut self) -> TestResult {
        let request_body = self.create_valid_request_body(None);

        let response = self.make_request(
            Method::POST,
            &self.endpoint,
            None, // No auth headers
            Some(request_body.clone()),
        ).await;

        let success = response.status_code == Some(401);

        TestResult::new("Unauthorized Access", success)
            .with_details(format!("Status: {:?} (expected 401)", response.status_code))
            .with_input("endpoint", &self.endpoint)
            .with_input("method", "POST")
            .with_output("response", &response)
    }

    async fn test_invalid_token(&mut self) -> TestResult {
        let request_body = self.create_valid_request_body(None);

        let mut headers = self.default_headers();
        headers.insert(
            "Authorization",
            "Bearer invalid.jwt.token".parse().unwrap(),
        );

        let response = self.make_request(
            Method::POST,
            &self.endpoint,
            Some(headers),
            Some(request_body),
        ).await;

        let success = response.status_code == Some(403);

        TestResult::new("Invalid Token", success)
            .with_details(format!("Status: {:?} (expected 403)", response.status_code))
            .with_input("endpoint", &self.endpoint)
            .with_output("response", &response)
    }

    async fn test_successful_registration(&mut self) -> TestResult {
        let user_id = "biometric-test-user-001";
        let token_id = "biometric-test-token-001";

        let token = match self.create_auth_token(user_id, token_id).await {
            Ok(t) => t,
            Err(e) => {
                return TestResult::fail("Successful Registration")
                    .with_details(format!("Failed to create auth token: {}", e));
            }
        };

        let installation_id = self.generate_installation_id();
        let request_body = json!({
            "publicKey": VALID_PUBLIC_KEY,
            "algorithm": "RSA",
            "installationId": &installation_id,
            "appVersion": "1.0.0",
            "appBuildNumber": 1
        });

        let response = self.make_request(
            Method::POST,
            &self.endpoint,
            Some(self.auth_headers(&token)),
            Some(request_body),
        ).await;

        let mut validation_details = Vec::new();
        let mut success = response.status_code == Some(204);

        if !success {
            validation_details.push(format!("Status: {:?} (expected 204)", response.status_code));
        } else {
            validation_details.push("Status: 204".to_string());

            // Verify record in database
            if self.verify_biometric_in_database(&installation_id, user_id).await {
                validation_details.push("Record verified in database".to_string());
            } else {
                success = false;
                validation_details.push("Record not found in database".to_string());
            }
        }

        TestResult::new("Successful Registration", success)
            .with_details(validation_details.join("; "))
            .with_input("endpoint", &self.endpoint)
            .with_input("installationId", &installation_id)
            .with_output("response", &response)
    }

    async fn test_missing_required_field(&mut self) -> TestResult {
        let token = match self.create_auth_token(
            "biometric-test-user-002",
            "biometric-test-token-002"
        ).await {
            Ok(t) => t,
            Err(e) => {
                return TestResult::fail("Missing Required Field")
                    .with_details(format!("Setup failed: {}", e));
            }
        };

        // Missing publicKey field
        let request_body = json!({
            "algorithm": "RSA",
            "installationId": self.generate_installation_id(),
            "appVersion": "1.0.0",
            "appBuildNumber": 1
        });

        let response = self.make_request(
            Method::POST,
            &self.endpoint,
            Some(self.auth_headers(&token)),
            Some(request_body),
        ).await;

        let success = response.status_code == Some(400);

        TestResult::new("Missing Required Field (publicKey)", success)
            .with_details(format!("Status: {:?} (expected 400)", response.status_code))
            .with_input("missing_field", "publicKey")
            .with_output("response", &response)
    }

    async fn test_invalid_algorithm(&mut self) -> TestResult {
        let token = match self.create_auth_token(
            "biometric-test-user-003",
            "biometric-test-token-003"
        ).await {
            Ok(t) => t,
            Err(e) => {
                return TestResult::fail("Invalid Algorithm")
                    .with_details(format!("Setup failed: {}", e));
            }
        };

        let request_body = json!({
            "publicKey": VALID_PUBLIC_KEY,
            "algorithm": "ECDSA",  // Unsupported
            "installationId": self.generate_installation_id(),
            "appVersion": "1.0.0",
            "appBuildNumber": 1
        });

        let response = self.make_request(
            Method::POST,
            &self.endpoint,
            Some(self.auth_headers(&token)),
            Some(request_body),
        ).await;

        let success = response.status_code == Some(400);

        TestResult::new("Invalid Algorithm", success)
            .with_details(format!("Status: {:?} (expected 400)", response.status_code))
            .with_input("algorithm", "ECDSA")
            .with_output("response", &response)
    }

    async fn test_weak_key_size(&mut self) -> TestResult {
        let token = match self.create_auth_token(
            "biometric-test-user-005",
            "biometric-test-token-005"
        ).await {
            Ok(t) => t,
            Err(e) => {
                return TestResult::fail("Weak Key Size")
                    .with_details(format!("Setup failed: {}", e));
            }
        };

        let request_body = json!({
            "publicKey": WEAK_PUBLIC_KEY,
            "algorithm": "RSA",
            "installationId": self.generate_installation_id(),
            "appVersion": "1.0.0",
            "appBuildNumber": 1
        });

        let response = self.make_request(
            Method::POST,
            &self.endpoint,
            Some(self.auth_headers(&token)),
            Some(request_body),
        ).await;

        let success = response.status_code == Some(400);

        TestResult::new("Weak Key Size", success)
            .with_details(format!("Status: {:?} (expected 400)", response.status_code))
            .with_input("keySize", "1024")
            .with_output("response", &response)
    }

    async fn test_upsert_registration(&mut self) -> TestResult {
        let user_id = "biometric-test-user-006";
        let token = match self.create_auth_token(user_id, "biometric-test-token-006").await {
            Ok(t) => t,
            Err(e) => {
                return TestResult::fail("Upsert Registration")
                    .with_details(format!("Setup failed: {}", e));
            }
        };

        let installation_id = self.generate_installation_id();

        // First registration
        let request_body1 = json!({
            "publicKey": VALID_PUBLIC_KEY,
            "algorithm": "RSA",
            "installationId": &installation_id,
            "appVersion": "1.0.0",
            "appBuildNumber": 1
        });

        let response1 = self.make_request(
            Method::POST,
            &self.endpoint,
            Some(self.auth_headers(&token)),
            Some(request_body1),
        ).await;

        // Second registration (upsert)
        let request_body2 = json!({
            "publicKey": VALID_PUBLIC_KEY,
            "algorithm": "RSA",
            "installationId": &installation_id,
            "appVersion": "1.1.0",
            "appBuildNumber": 2
        });

        let response2 = self.make_request(
            Method::POST,
            &self.endpoint,
            Some(self.auth_headers(&token)),
            Some(request_body2),
        ).await;

        let mut success = response1.status_code == Some(204) && response2.status_code == Some(204);
        let mut validation_details = Vec::new();

        if success {
            // Verify record was updated
            if let Some(mongo) = &self.mongo_manager {
                match mongo.find_one("biometricAuth", doc! { "_id": &installation_id }).await {
                    Ok(Some(record)) => {
                        if record.get_str("appVersion").ok() == Some("1.1.0") {
                            validation_details.push("Record successfully updated (upsert)".to_string());
                        } else {
                            success = false;
                            validation_details.push("Record was not properly updated".to_string());
                        }
                    }
                    _ => {
                        success = false;
                        validation_details.push("Could not verify record".to_string());
                    }
                }
            }
        } else {
            validation_details.push(format!(
                "First: {:?}, Second: {:?}",
                response1.status_code, response2.status_code
            ));
        }

        TestResult::new("Upsert Registration", success)
            .with_details(validation_details.join("; "))
            .with_input("installationId", &installation_id)
            .with_output("first_response", &response1)
            .with_output("second_response", &response2)
    }
}

// ==================== Trait Implementations ====================

impl Test for RegisterBiometricApiTest {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn results_dir(&self) -> Option<&PathBuf> {
        self.base.results_dir.as_ref()
    }

    fn setup(&mut self) -> Result<(), TestError> {
        // Initialize setup manager
        let rt = tokio::runtime::Runtime::new().unwrap();
        let setup_result = rt.block_on(self.setup_manager.setup())
            .map_err(|e| TestError::SetupError(e.to_string()))?;

        // Store managers
        self.redis_manager = Some(RedisSessionManager::new(setup_result.redis_client));
        self.mongo_manager = Some(MongoDbManager::new(setup_result.mongo_client, "primary", "audit"));

        self.common_utils = Some(CommonUtils::new(
            self.redis_manager.clone().unwrap(),
            self.mongo_manager.clone().unwrap(),
            self.jwt_manager.clone(),
        ));

        Ok(())
    }

    fn run_tests(&mut self) -> Vec<TestResult> {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            vec![
                self.test_unauthorized_access().await,
                self.test_invalid_token().await,
                self.test_successful_registration().await,
                self.test_missing_required_field().await,
                self.test_invalid_algorithm().await,
                self.test_weak_key_size().await,
                self.test_upsert_registration().await,
            ]
        })
    }

    fn teardown(&mut self) -> Result<(), TestError> {
        // Cleanup test records from MongoDB
        if let Some(ref mongo) = self.mongo_manager {
            let rt = tokio::runtime::Runtime::new().unwrap();

            for installation_id in &self.test_installation_ids {
                let _ = rt.block_on(
                    mongo.delete_one("biometricAuth", doc! { "_id": installation_id })
                );
            }
        }

        self.test_installation_ids.clear();
        Ok(())
    }
}

#[async_trait]
impl ApiTest for RegisterBiometricApiTest {
    fn api_base_url(&self) -> &str {
        &self.base.api_base_url
    }

    fn client(&self) -> &reqwest::Client {
        &self.base.client
    }
}
```

---

## Multi-Step Flow Test Pattern

For complex operations requiring multiple steps with validations.

### Example: Profile API Flow

```rust
async fn test_profile_api_flow(&mut self) -> TestResult {
    // Step 1: Create Profile
    let create_response = self.make_request(
        Method::POST,
        "/customers/profile",
        Some(self.auth_headers(&self.token)),
        Some(self.profile_payload.clone()),
    ).await;

    // Step 1a: Validate 201 response
    let (create_success, create_details, create_body) = self.common_utils
        .as_ref()
        .unwrap()
        .validate_api_response(&create_response, 201, None, true);

    if !create_success {
        return TestResult::fail("Profile API Flow")
            .with_details(format!("Step 1a FAILED: {}", create_details));
    }

    // Step 1b: Validate response body matches expected
    let body = create_body.unwrap();
    let mismatches = self.common_utils
        .as_ref()
        .unwrap()
        .compare_nested_objects(&self.expected_response, &body, "");

    if !mismatches.is_empty() {
        return TestResult::fail("Profile API Flow")
            .with_details(format!("Step 1b FAILED: {}", mismatches.join("; ")));
    }

    // Step 1c: Validate database record
    let (db_success, db_details, _) = self.common_utils
        .as_ref()
        .unwrap()
        .assert_db_record(
            "customerProfiles",
            doc! { "userId": &self.user_id },
            &self.expected_db_record,
        ).await;

    if !db_success {
        return TestResult::fail("Profile API Flow")
            .with_details(format!("Step 1c FAILED: {}", db_details));
    }

    // Step 1d: Validate cache
    let cache_key = format!("user-data:temp-data:{}:mobile", self.user_id);
    let (cache_success, cache_details, _) = self.common_utils
        .as_ref()
        .unwrap()
        .assert_cache_record(&cache_key, &expected_mobile, "unverified mobile");

    if !cache_success {
        return TestResult::fail("Profile API Flow")
            .with_details(format!("Step 1d FAILED: {}", cache_details));
    }

    // Step 2: GET Profile
    let get_response = self.make_request(
        Method::GET,
        "/customers/profile",
        Some(self.auth_headers(&self.token)),
        None,
    ).await;

    // ... continue with more steps

    TestResult::pass("Profile API Flow")
        .with_details("SUCCESS: All steps passed")
        .with_output("create_response", &create_response)
        .with_output("get_response", &get_response)
}
```

---

## Validation Pattern Summary

```rust
// Pattern 1: Simple status check
let success = response.status_code == Some(expected_status);

// Pattern 2: Status + error code check
let (success, details, body) = common_utils.validate_api_response(
    &response,
    expected_status,
    Some("ERROR_CODE"),
    true  // check_body
);

// Pattern 3: Status + body comparison
let (success, details, body) = common_utils.validate_api_response(&response, 200, None, true);
if success {
    let mismatches = common_utils.compare_nested_objects(&expected, body.as_ref().unwrap(), "");
    if !mismatches.is_empty() {
        // Handle mismatches
    }
}

// Pattern 4: Status + database verification
if success {
    let (db_ok, db_details, record) = common_utils.assert_db_record(
        "collection",
        doc! { "id": id },
        &expected_record
    ).await;
}

// Pattern 5: Status + cache verification
if success {
    let (cache_ok, cache_details, _) = common_utils.assert_cache_record(
        &key,
        &expected_value,
        "cache_type"
    );
}

// Pattern 6: Status + audit log verification
if success {
    let request_id = response.headers.get("X-Request-ID").unwrap();
    let (audit_ok, audit_details, _) = common_utils.assert_audit_log(
        request_id,
        "ACTION",
        "SUCCESS",
        "EVENT_TYPE",
        &actor,
        10  // timeout seconds
    ).await;
}
```
