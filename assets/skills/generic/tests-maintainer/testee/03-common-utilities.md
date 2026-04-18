# Common Test Utilities

This document details the utility managers used across all tests for setup, authentication, caching, and database operations.

## TestSetupManager

Manages test environment initialization including connections and logging.

### Python Original

```python
class TestSetupManager:
    """Manages common test setup including Redis connection and logging."""

    def __init__(self, test_name: str):
        self.test_name = test_name
        self.redis_client = None
        self.auth_cache = None
        self.results_dir = None
        self.log_file = None
        self.start_time = datetime.now()

    def setup(self) -> Dict[str, Any]:
        """Set up test environment including Redis connection and results directory."""
        # Create results directory with script subdirectory
        date_str = self.start_time.strftime("%Y-%m-%d_%H-%M-%S")
        self.results_dir = Path(...) / "results" / date_str / self.test_name
        self.results_dir.mkdir(parents=True, exist_ok=True)

        # Create log file
        self.log_file = self.results_dir / f"{self.test_name}.log"

        # Setup logging header
        header = f"""
{"="*70}
{self.test_name.upper().replace('_', ' ')} TEST SUITE
{"="*70}
Start Time: {self.start_time.isoformat()}
Environment: {os.getenv('environment', 'NOT SET')}
Script: {self.test_name}.py
Results Directory: {self.results_dir}
{"="*70}
"""
        self.log_and_print(header)

        try:
            self.redis_client = RedisDBConnector.get_client()
            self.auth_cache = AuthCache(self.redis_client)
            self.mongo_db_manager = MongoDBManager()
            self.log_and_print("[OK] Connected to Redis")

            return {
                "redis_client": self.redis_client,
                "auth_cache": self.auth_cache,
                "mongo_db_manager": self.mongo_db_manager,
                "results_dir": self.results_dir,
                "log_file": self.log_file,
                "start_time": self.start_time
            }
        except Exception as e:
            self.log_and_print(f"[FAILED] Could not connect to Redis: {e}")
            raise

    def log_and_print(self, message: str):
        """Log message to file and print to console."""
        print(message)
        if self.log_file:
            with open(self.log_file, "a", encoding="utf-8") as f:
                f.write(message + "\n")
```

### Rust Implementation

```rust
use chrono::{DateTime, Utc};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use tracing::info;

/// Setup result containing initialized resources
pub struct SetupResult {
    pub redis_client: redis::Client,
    pub mongo_client: mongodb::Client,
    pub results_dir: PathBuf,
    pub log_file: PathBuf,
    pub start_time: DateTime<Utc>,
}

/// Manages common test setup including connections and logging
pub struct TestSetupManager {
    test_name: String,
    results_dir: Option<PathBuf>,
    log_file: Option<PathBuf>,
    start_time: DateTime<Utc>,
}

impl TestSetupManager {
    pub fn new(test_name: impl Into<String>) -> Self {
        Self {
            test_name: test_name.into(),
            results_dir: None,
            log_file: None,
            start_time: Utc::now(),
        }
    }

    /// Set up test environment including connections and results directory
    pub async fn setup(&mut self) -> Result<SetupResult, SetupError> {
        // Create results directory with timestamp
        let date_str = self.start_time.format("%Y-%m-%d_%H-%M-%S").to_string();
        let base_dir = std::env::current_dir()?
            .join("testing")
            .join("results")
            .join(&date_str)
            .join(&self.test_name);

        fs::create_dir_all(&base_dir)?;
        self.results_dir = Some(base_dir.clone());

        // Create log file
        let log_file = base_dir.join(format!("{}.log", self.test_name));
        self.log_file = Some(log_file.clone());

        // Setup logging header
        let header = format!(
            r#"
======================================================================
{} TEST SUITE
======================================================================
Start Time: {}
Environment: {}
Script: {}.rs
Results Directory: {}
======================================================================
"#,
            self.test_name.to_uppercase().replace('_', " "),
            self.start_time.to_rfc3339(),
            std::env::var("ENVIRONMENT").unwrap_or_else(|_| "NOT SET".to_string()),
            self.test_name,
            base_dir.display()
        );

        self.log_and_print(&header);

        // Connect to Redis
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let redis_client = redis::Client::open(redis_url)
            .map_err(|e| SetupError::Redis(e.to_string()))?;

        self.log_and_print("[OK] Connected to Redis");

        // Connect to MongoDB
        let mongo_uri = std::env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        let mongo_client = mongodb::Client::with_uri_str(&mongo_uri)
            .await
            .map_err(|e| SetupError::MongoDB(e.to_string()))?;

        self.log_and_print("[OK] Connected to MongoDB");

        Ok(SetupResult {
            redis_client,
            mongo_client,
            results_dir: base_dir,
            log_file,
            start_time: self.start_time,
        })
    }

    /// Log message to file and print to console
    pub fn log_and_print(&self, message: &str) {
        println!("{}", message);

        if let Some(ref log_file) = self.log_file {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file)
            {
                let _ = writeln!(file, "{}", message);
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SetupError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Redis connection error: {0}")]
    Redis(String),

    #[error("MongoDB connection error: {0}")]
    MongoDB(String),
}
```

---

## JWTTokenManager

Creates and manages JWT tokens for test authentication.

### Python Original

```python
class JWTTokenManager:
    """Manages JWT token creation and validation for testing."""

    def __init__(self, jwt_config: Optional[Dict] = None):
        if jwt_config:
            self.jwt_config = jwt_config
        else:
            jwt_secrets = Config.get_jwt_secrets()
            self.jwt_config = {
                "key": jwt_secrets.key,
                "algorithm": jwt_secrets.algorithm or "HS256"
            }

    def create_jwt_token(self,
                        user_id: str = "test-user-001",
                        token_id: str = "test-token-001",
                        expires_in_seconds: int = 120,
                        additional_claims: Optional[Dict] = None) -> Tuple[str, Dict]:
        """Create a JWT token for testing."""
        payload = {
            'sub': user_id,          # Subject (user ID)
            'jwtId': token_id,       # JWT ID (token ID)
        }

        if additional_claims:
            payload.update(additional_claims)

        token = jwt.encode(
            payload,
            self.jwt_config["key"],
            algorithm=self.jwt_config["algorithm"]
        )

        return token, payload
```

### Rust Implementation

```rust
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{Utc, Duration};

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,           // Subject (user ID)
    #[serde(rename = "jwtId")]
    pub jwt_id: String,        // JWT ID (token ID)
    pub exp: i64,              // Expiration time
    pub iat: i64,              // Issued at
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// JWT configuration
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub key: String,
    pub algorithm: Algorithm,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            key: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "test-jwt-secret-key".to_string()),
            algorithm: Algorithm::HS256,
        }
    }
}

/// Manages JWT token creation for testing
pub struct JwtTokenManager {
    config: JwtConfig,
}

impl JwtTokenManager {
    pub fn new() -> Self {
        Self {
            config: JwtConfig::default(),
        }
    }

    pub fn with_config(config: JwtConfig) -> Self {
        Self { config }
    }

    /// Create a JWT token for testing
    pub fn create_jwt_token(
        &self,
        user_id: &str,
        token_id: &str,
        expires_in_seconds: i64,
        additional_claims: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<(String, JwtClaims), JwtError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(expires_in_seconds);

        let claims = JwtClaims {
            sub: user_id.to_string(),
            jwt_id: token_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            additional: additional_claims.unwrap_or_default(),
        };

        let token = encode(
            &Header::new(self.config.algorithm),
            &claims,
            &EncodingKey::from_secret(self.config.key.as_bytes()),
        ).map_err(|e| JwtError::EncodingError(e.to_string()))?;

        Ok((token, claims))
    }

    /// Create token with default parameters
    pub fn create_test_token(&self) -> Result<(String, JwtClaims), JwtError> {
        self.create_jwt_token("test-user-001", "test-token-001", 120, None)
    }

    /// Create token for specific user
    pub fn create_user_token(&self, user_id: &str, token_id: &str) -> Result<(String, JwtClaims), JwtError> {
        self.create_jwt_token(user_id, token_id, 300, None)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("JWT encoding error: {0}")]
    EncodingError(String),

    #[error("JWT decoding error: {0}")]
    DecodingError(String),
}
```

---

## RedisSessionManager

Manages Redis session storage for authentication tokens.

### Python Original

```python
class RedisSessionManager:
    """Manages Redis session storage for testing."""

    def __init__(self, redis_client, auth_cache: AuthCache):
        self.redis_client = redis_client
        self.auth_cache = auth_cache

    def store_jwt_session(self, user_id: str, token_id: str, ttl: int = 300) -> str:
        """Store JWT session in Redis."""
        key = self.auth_cache.jwt_record_key(user_id, "access", token_id)
        value = token_id
        self.redis_client.set(key, value, ex=ttl)
        return key

    def store_admin_jwt_session(self, user_id: str, token_id: str, ttl: int = 300) -> str:
        """Store admin JWT session in Redis."""
        key = self.auth_cache.admin_jwt_record_key(user_id, "access", token_id)
        value = token_id
        self.redis_client.set(key, value, ex=ttl)
        return key

    def store_captcha_token(self, token_str: str) -> str:
        captcha_token_str = f"captcha:{token_str}"
        self.redis_client.set(f"captcha:{token_str}", "signup", ex=300)
        return captcha_token_str
```

### Rust Implementation

```rust
use redis::{Client, Commands, RedisError};

/// Redis session manager for test authentication
pub struct RedisSessionManager {
    client: Client,
    key_prefix: String,
}

impl RedisSessionManager {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            key_prefix: "auth".to_string(),
        }
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.key_prefix = prefix.into();
        self
    }

    /// Generate JWT record key
    fn jwt_record_key(&self, user_id: &str, token_type: &str, token_id: &str) -> String {
        format!("{}:jwt:{}:{}:{}", self.key_prefix, user_id, token_type, token_id)
    }

    /// Generate admin JWT record key
    fn admin_jwt_record_key(&self, user_id: &str, token_type: &str, token_id: &str) -> String {
        format!("{}:admin:jwt:{}:{}:{}", self.key_prefix, user_id, token_type, token_id)
    }

    /// Store JWT session in Redis
    pub fn store_jwt_session(
        &self,
        user_id: &str,
        token_id: &str,
        ttl_seconds: usize,
    ) -> Result<String, RedisError> {
        let key = self.jwt_record_key(user_id, "access", token_id);
        let mut conn = self.client.get_connection()?;
        conn.set_ex(&key, token_id, ttl_seconds)?;
        Ok(key)
    }

    /// Store admin JWT session in Redis
    pub fn store_admin_jwt_session(
        &self,
        user_id: &str,
        token_id: &str,
        ttl_seconds: usize,
    ) -> Result<String, RedisError> {
        let key = self.admin_jwt_record_key(user_id, "access", token_id);
        let mut conn = self.client.get_connection()?;
        conn.set_ex(&key, token_id, ttl_seconds)?;
        Ok(key)
    }

    /// Store captcha token in Redis
    pub fn store_captcha_token(
        &self,
        token: &str,
        purpose: &str,
        ttl_seconds: usize,
    ) -> Result<String, RedisError> {
        let key = format!("captcha:{}", token);
        let mut conn = self.client.get_connection()?;
        conn.set_ex(&key, purpose, ttl_seconds)?;
        Ok(key)
    }

    /// Delete a key from Redis
    pub fn delete_key(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.client.get_connection()?;
        let deleted: i32 = conn.del(key)?;
        Ok(deleted > 0)
    }

    /// Delete multiple keys from Redis
    pub fn delete_keys(&self, keys: &[String]) -> Result<usize, RedisError> {
        if keys.is_empty() {
            return Ok(0);
        }

        let mut conn = self.client.get_connection()?;
        let deleted: i32 = conn.del(keys)?;
        Ok(deleted as usize)
    }

    /// Get value for key
    pub fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
        let mut conn = self.client.get_connection()?;
        conn.get(key)
    }
}
```

---

## MongoDBManager

Handles MongoDB CRUD operations for test data.

### Python Original

```python
class MongoDBManager:
    """Manages MongoDB connections and operations for testing."""

    def __init__(self):
        self.client = MongoClientFactory.get_client_for_primary()
        config = Config.get_mongodb_config_primary()
        db_name = config.databases["primary"]
        self.db: Database = self.client.get_database(db_name)

        # Initialize audit client and database
        self.audit_client = MongoClientFactory.get_client_for_audit()
        audit_config = Config.get_mongodb_config_audit()
        audit_db_name = audit_config.databases["audit"]
        self.audit_db: Database = self.audit_client.get_database(audit_db_name)
        self.audit_logs_coll: Collection = self.audit_db.auditLogs

    def insert_test_auth_credential(self, email: str, user_id: str, additional_fields: Optional[Dict] = None) -> str:
        document = {
            "_id": user_id,
            "userId": user_id,
            "userName": email.lower(),
            "method": "password",
            "password": {"hashAndSalt": ""}
        }
        if additional_fields:
            document.update(additional_fields)
        self.auth_credentials_coll.insert_one(document)
        return user_id

    def delete_auth_credentials_by_username(self, email: str) -> bool:
        query = {"userName": email}
        result = self.auth_credentials_coll.delete_one(query)
        return result.deleted_count > 0

    def insert_test_customer_profile(self, user_id: str, fields: dict = None) -> str:
        document = {
            "_id": user_id,
            "userId": user_id,
            "firstName": "TestCode First",
            "lastName": "TestCode Last",
            # ... default fields
        }
        fields = fields or {}
        document.update(fields)
        self.customer_profile_collection.insert_one(document)

    def get_customer_profile(self, query: dict) -> dict:
        return self.customer_profile_collection.find_one(query)

    def update_customer_profile(self, query: dict, fields: dict) -> dict:
        result = self.customer_profile_collection.update_one(query, {"$set": fields})
        return result

    def delete_customer_profile(self, query: dict) -> None:
        self.customer_profile_collection.delete_one(query)
```

### Rust Implementation

```rust
use mongodb::{
    Client, Collection, Database,
    bson::{doc, Document, Bson},
    options::{FindOneOptions, UpdateOptions},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// MongoDB manager for test data operations
pub struct MongoDbManager {
    client: Client,
    db: Database,
    audit_db: Database,
}

impl MongoDbManager {
    pub async fn new(client: Client, db_name: &str, audit_db_name: &str) -> Self {
        let db = client.database(db_name);
        let audit_db = client.database(audit_db_name);

        Self { client, db, audit_db }
    }

    /// Get a collection by name
    pub fn collection<T>(&self, name: &str) -> Collection<T>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        self.db.collection(name)
    }

    /// Get audit logs collection
    pub fn audit_logs(&self) -> Collection<Document> {
        self.audit_db.collection("auditLogs")
    }

    // ==================== Auth Credentials ====================

    /// Insert test auth credential
    pub async fn insert_test_auth_credential(
        &self,
        email: &str,
        user_id: &str,
        additional_fields: Option<Document>,
    ) -> Result<String, mongodb::error::Error> {
        let coll: Collection<Document> = self.db.collection("authCredentials");

        let mut document = doc! {
            "_id": user_id,
            "userId": user_id,
            "userName": email.to_lowercase(),
            "method": "password",
            "password": { "hashAndSalt": "" }
        };

        if let Some(fields) = additional_fields {
            for (key, value) in fields {
                document.insert(key, value);
            }
        }

        coll.insert_one(document, None).await?;
        Ok(user_id.to_string())
    }

    /// Delete auth credentials by username
    pub async fn delete_auth_credentials_by_username(
        &self,
        email: &str,
    ) -> Result<bool, mongodb::error::Error> {
        let coll: Collection<Document> = self.db.collection("authCredentials");
        let result = coll.delete_one(doc! { "userName": email }, None).await?;
        Ok(result.deleted_count > 0)
    }

    /// Get auth credential by query
    pub async fn get_auth_credential(
        &self,
        query: Document,
    ) -> Result<Option<Document>, mongodb::error::Error> {
        let coll: Collection<Document> = self.db.collection("authCredentials");
        coll.find_one(query, None).await
    }

    // ==================== Customer Profiles ====================

    /// Insert test customer profile
    pub async fn insert_test_customer_profile(
        &self,
        user_id: &str,
        fields: Option<Document>,
    ) -> Result<(), mongodb::error::Error> {
        let coll: Collection<Document> = self.db.collection("customerProfiles");

        let mut document = doc! {
            "_id": user_id,
            "userId": user_id,
            "firstName": "TestCode First",
            "lastName": "TestCode Last",
            "gender": "F",
            "address": {
                "houseNumber": "TestCode 123",
                "street": "TestCode St.",
                "locality": "TestCode City",
                "state": "QLD",
                "postcode": "1234",
                "country": "AU"
            },
            "dateOfBirth": Bson::DateTime(DateTime::parse_from_rfc3339("1990-05-16T00:00:00Z").unwrap().with_timezone(&Utc).into())
        };

        if let Some(extra) = fields {
            for (key, value) in extra {
                document.insert(key, value);
            }
        }

        coll.insert_one(document, None).await?;
        Ok(())
    }

    /// Get customer profile
    pub async fn get_customer_profile(
        &self,
        query: Document,
    ) -> Result<Option<Document>, mongodb::error::Error> {
        let coll: Collection<Document> = self.db.collection("customerProfiles");
        coll.find_one(query, None).await
    }

    /// Update customer profile
    pub async fn update_customer_profile(
        &self,
        query: Document,
        fields: Document,
    ) -> Result<UpdateResult, mongodb::error::Error> {
        let coll: Collection<Document> = self.db.collection("customerProfiles");
        coll.update_one(query, doc! { "$set": fields }, None).await
    }

    /// Delete customer profile
    pub async fn delete_customer_profile(
        &self,
        query: Document,
    ) -> Result<DeleteResult, mongodb::error::Error> {
        let coll: Collection<Document> = self.db.collection("customerProfiles");
        coll.delete_one(query, None).await
    }

    // ==================== Generic Operations ====================

    /// Find one document in any collection
    pub async fn find_one(
        &self,
        collection_name: &str,
        query: Document,
    ) -> Result<Option<Document>, mongodb::error::Error> {
        let coll: Collection<Document> = self.db.collection(collection_name);
        coll.find_one(query, None).await
    }

    /// Delete one document in any collection
    pub async fn delete_one(
        &self,
        collection_name: &str,
        query: Document,
    ) -> Result<DeleteResult, mongodb::error::Error> {
        let coll: Collection<Document> = self.db.collection(collection_name);
        coll.delete_one(query, None).await
    }

    /// Update one document in any collection
    pub async fn update_one(
        &self,
        collection_name: &str,
        query: Document,
        update: Document,
    ) -> Result<UpdateResult, mongodb::error::Error> {
        let coll: Collection<Document> = self.db.collection(collection_name);
        coll.update_one(query, update, None).await
    }
}
```

---

## CommonUtils - Assertion and Validation Utilities

Provides shared assertion methods for API response, database, and cache validation.

### Python Original (Key Methods)

```python
class CommonUtils:
    def __init__(self, redis_client=None, mongodb_manager=None, jwt_manager=None, logger=None):
        self.redis_client = redis_client
        self.mongodb_manager = mongodb_manager
        self.jwt_manager = jwt_manager
        self.logger = logger

    def simulate_auth_session(self, user_prefix: str = "test", ttl: int = 180,
                               include_auth_record_create: dict = None) -> tuple:
        """Simulate auth session for testing."""
        timestamp = int(get_now().timestamp())
        user_id = f"{user_prefix}-u-{timestamp}"
        token_id = f"{user_prefix}-t-{timestamp}"

        # Store JWT session in Redis
        auth_cache_record_key = self.redis_session_manager.store_jwt_session(user_id, token_id, ttl=ttl)

        # Create JWT token
        token, _ = self.jwt_manager.create_jwt_token(user_id, token_id)

        if include_auth_record_create:
            email = include_auth_record_create["email"]
            MongoDBManager().insert_test_auth_credential(email=email, user_id=user_id, ...)

        return user_id, token, auth_cache_record_key

    def validate_api_response(self, response: Dict, expected_status: int,
                               expected_error_code: Optional[str] = None,
                               check_body: bool = True) -> tuple:
        """Validate API response status and error codes."""
        actual_status = response.get('status_code')

        if actual_status != expected_status:
            return (False, f"Status code mismatch: got {actual_status}, expected {expected_status}", None)

        if not check_body:
            return (True, f"Request succeeded with status: {expected_status}", None)

        # Parse response body
        body_str = response.get('body', '{}')
        parsed_body = json.loads(body_str) if body_str else {}

        # Check error code if expected
        if expected_error_code:
            actual_error_code = parsed_body.get('errorCode')
            if actual_error_code != expected_error_code:
                return (False, f"Error code mismatch: got '{actual_error_code}', expected '{expected_error_code}'", parsed_body)

        return (True, f"Request succeeded with status: {expected_status}", parsed_body)

    def assert_db_record(self, collection_name: str, query: dict, expected_record: dict) -> tuple:
        """Assert that database record matches expected values."""
        collection = self.mongodb_manager.db[collection_name]
        actual_record = collection.find_one(query)

        if not actual_record:
            return False, f"No record found in {collection_name} for query: {query}", None

        mismatches = self.compare_nested_objects(expected_record, actual_record)

        if mismatches:
            return False, f"Database record mismatches: {'; '.join(mismatches)}", actual_record

        return True, "Database record matches expected values", actual_record

    def assert_cache_record(self, key: str, expected_value: Any, cache_type: str = "generic") -> tuple:
        """Assert that a value exists in cache with expected value."""
        cached_value = self.redis_client.get(key)

        if cached_value != expected_value:
            return False, f"{cache_type} mismatch in cache. Expected: {expected_value}, Got: {cached_value}", key

        return True, f"{cache_type} '{expected_value}' correctly stored in cache", key

    def compare_nested_objects(self, expected: Dict, actual: Dict, parent_key: str = "") -> list:
        """Compare nested objects and return list of mismatches."""
        mismatches = []

        for field, expected_value in expected.items():
            actual_value = actual.get(field)
            field_path = f"{parent_key}.{field}" if parent_key else field

            if isinstance(expected_value, dict):
                if not isinstance(actual_value, dict):
                    mismatches.append(f"{field_path}: expected dict, got {type(actual_value).__name__}")
                else:
                    nested_mismatches = self.compare_nested_objects(expected_value, actual_value, field_path)
                    mismatches.extend(nested_mismatches)
            elif actual_value != expected_value:
                mismatches.append(f"{field_path}: expected '{expected_value}', got '{actual_value}'")

        return mismatches

    def assert_audit_log(self, request_id: str, expected_action: str, expected_status: str,
                          expected_event_type: str, expected_actor: str, timeout: int = 10, ...) -> tuple:
        """Poll for and validate audit log entry."""
        end_time = time.time() + timeout
        audit_log = None
        query = {"metadata.lambdaRequestId": request_id}

        while time.time() < end_time:
            audit_log = self.mongodb_manager.audit_logs_coll.find_one(query)
            if audit_log:
                break
            time.sleep(0.5)

        if not audit_log:
            return False, f"Audit log not found for request {request_id}", None

        expected_record = {
            "action": expected_action,
            "status": expected_status,
            "eventType": expected_event_type,
            "actor": expected_actor
        }

        mismatches = self.compare_nested_objects(expected_record, audit_log)

        if mismatches:
            return False, f"Audit log validation failed: {'; '.join(mismatches)}", audit_log

        return True, f"Audit log validated: action={expected_action}, status={expected_status}", audit_log
```

### Rust Implementation

```rust
use serde_json::Value;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use mongodb::bson::{doc, Document};

/// Assertion result type
pub type AssertionResult<T> = (bool, String, Option<T>);

/// Common utilities for test assertions and validation
pub struct CommonUtils {
    redis_manager: RedisSessionManager,
    mongo_manager: MongoDbManager,
    jwt_manager: JwtTokenManager,
}

impl CommonUtils {
    pub fn new(
        redis_manager: RedisSessionManager,
        mongo_manager: MongoDbManager,
        jwt_manager: JwtTokenManager,
    ) -> Self {
        Self {
            redis_manager,
            mongo_manager,
            jwt_manager,
        }
    }

    /// Simulate auth session for testing
    pub async fn simulate_auth_session(
        &self,
        user_prefix: &str,
        ttl_seconds: usize,
        include_auth_record: Option<AuthRecordCreate>,
    ) -> Result<AuthSession, CommonUtilsError> {
        let timestamp = chrono::Utc::now().timestamp();
        let user_id = format!("{}-u-{}", user_prefix, timestamp);
        let token_id = format!("{}-t-{}", user_prefix, timestamp);

        // Store JWT session in Redis
        let auth_cache_key = self.redis_manager.store_jwt_session(&user_id, &token_id, ttl_seconds)?;

        // Create JWT token
        let (token, _claims) = self.jwt_manager.create_user_token(&user_id, &token_id)?;

        // Optionally create auth record in MongoDB
        if let Some(auth_create) = include_auth_record {
            self.mongo_manager.insert_test_auth_credential(
                &auth_create.email,
                &user_id,
                auth_create.additional_fields,
            ).await?;
        }

        Ok(AuthSession {
            user_id,
            token,
            auth_cache_key,
        })
    }

    /// Validate API response status and error codes
    pub fn validate_api_response(
        &self,
        response: &ApiResponse,
        expected_status: u16,
        expected_error_code: Option<&str>,
        check_body: bool,
    ) -> AssertionResult<Value> {
        let actual_status = response.status_code.unwrap_or(0);

        if actual_status != expected_status {
            return (
                false,
                format!("Status code mismatch: got {}, expected {}", actual_status, expected_status),
                None,
            );
        }

        if !check_body {
            return (
                true,
                format!("Request succeeded with status: {}", expected_status),
                None,
            );
        }

        // Parse response body
        let parsed_body: Value = match response.parse_body_value() {
            Ok(body) => body,
            Err(e) => return (false, format!("Failed to parse response body: {}", e), None),
        };

        // Check error code if expected
        if let Some(expected_code) = expected_error_code {
            let actual_code = parsed_body.get("errorCode").and_then(|v| v.as_str());

            if actual_code != Some(expected_code) {
                return (
                    false,
                    format!(
                        "Error code mismatch: got '{:?}', expected '{}'",
                        actual_code, expected_code
                    ),
                    Some(parsed_body),
                );
            }

            return (
                true,
                format!("Request correctly returned status {} with error code '{}'", expected_status, expected_code),
                Some(parsed_body),
            );
        }

        (
            true,
            format!("Request succeeded with status: {}", expected_status),
            Some(parsed_body),
        )
    }

    /// Assert that database record matches expected values
    pub async fn assert_db_record(
        &self,
        collection_name: &str,
        query: Document,
        expected_record: &Value,
    ) -> AssertionResult<Document> {
        let actual_record = match self.mongo_manager.find_one(collection_name, query.clone()).await {
            Ok(Some(record)) => record,
            Ok(None) => {
                return (
                    false,
                    format!("No record found in {} for query: {:?}", collection_name, query),
                    None,
                );
            }
            Err(e) => {
                return (
                    false,
                    format!("Database query failed: {}", e),
                    None,
                );
            }
        };

        // Convert Document to Value for comparison
        let actual_value: Value = mongodb::bson::from_document(actual_record.clone())
            .unwrap_or(Value::Null);

        let mismatches = self.compare_nested_objects(expected_record, &actual_value, "");

        if !mismatches.is_empty() {
            return (
                false,
                format!("Database record mismatches: {}", mismatches.join("; ")),
                Some(actual_record),
            );
        }

        (true, "Database record matches expected values".to_string(), Some(actual_record))
    }

    /// Assert that cache contains expected value
    pub fn assert_cache_record(
        &self,
        key: &str,
        expected_value: &str,
        cache_type: &str,
    ) -> AssertionResult<String> {
        match self.redis_manager.get(key) {
            Ok(Some(actual_value)) => {
                if actual_value != expected_value {
                    (
                        false,
                        format!(
                            "{} mismatch in cache. Expected: {}, Got: {}",
                            cache_type, expected_value, actual_value
                        ),
                        Some(key.to_string()),
                    )
                } else {
                    (
                        true,
                        format!("{} '{}' correctly stored in cache", cache_type, expected_value),
                        Some(key.to_string()),
                    )
                }
            }
            Ok(None) => (
                false,
                format!("{} not found in cache for key: {}", cache_type, key),
                Some(key.to_string()),
            ),
            Err(e) => (
                false,
                format!("Cache query failed: {}", e),
                Some(key.to_string()),
            ),
        }
    }

    /// Compare nested objects and return list of mismatches
    pub fn compare_nested_objects(
        &self,
        expected: &Value,
        actual: &Value,
        parent_key: &str,
    ) -> Vec<String> {
        let mut mismatches = Vec::new();

        if let Value::Object(expected_map) = expected {
            for (field, expected_value) in expected_map {
                let field_path = if parent_key.is_empty() {
                    field.clone()
                } else {
                    format!("{}.{}", parent_key, field)
                };

                let actual_value = actual.get(field);

                match (expected_value, actual_value) {
                    (Value::Object(_), Some(Value::Object(_))) => {
                        // Recursive comparison for nested objects
                        let nested = self.compare_nested_objects(
                            expected_value,
                            actual_value.unwrap(),
                            &field_path,
                        );
                        mismatches.extend(nested);
                    }
                    (Value::Object(_), Some(other)) => {
                        mismatches.push(format!(
                            "{}: expected object, got {:?}",
                            field_path, other
                        ));
                    }
                    (_, None) => {
                        mismatches.push(format!(
                            "{}: expected '{}', got null/missing",
                            field_path, expected_value
                        ));
                    }
                    (expected_val, Some(actual_val)) if expected_val != actual_val => {
                        mismatches.push(format!(
                            "{}: expected '{}', got '{}'",
                            field_path, expected_val, actual_val
                        ));
                    }
                    _ => {} // Values match
                }
            }
        }

        mismatches
    }

    /// Poll for and validate audit log entry
    pub async fn assert_audit_log(
        &self,
        request_id: &str,
        expected_action: &str,
        expected_status: &str,
        expected_event_type: &str,
        expected_actor: &str,
        timeout_seconds: u64,
    ) -> AssertionResult<Document> {
        let start = Instant::now();
        let timeout = Duration::from_secs(timeout_seconds);
        let query = doc! { "metadata.lambdaRequestId": request_id };

        // Poll for audit log
        let audit_log = loop {
            if start.elapsed() >= timeout {
                return (
                    false,
                    format!("Audit log not found for request {} after {}s timeout", request_id, timeout_seconds),
                    None,
                );
            }

            match self.mongo_manager.audit_logs().find_one(query.clone(), None).await {
                Ok(Some(log)) => break log,
                Ok(None) => {
                    sleep(Duration::from_millis(500)).await;
                    continue;
                }
                Err(e) => {
                    return (false, format!("Audit log query failed: {}", e), None);
                }
            }
        };

        // Build expected record
        let expected_record = serde_json::json!({
            "action": expected_action,
            "status": expected_status,
            "eventType": expected_event_type,
            "actor": expected_actor
        });

        let actual_value: Value = mongodb::bson::from_document(audit_log.clone())
            .unwrap_or(Value::Null);

        let mismatches = self.compare_nested_objects(&expected_record, &actual_value, "");

        if !mismatches.is_empty() {
            return (
                false,
                format!("Audit log validation failed: {}", mismatches.join("; ")),
                Some(audit_log),
            );
        }

        (
            true,
            format!("Audit log validated: action={}, status={}", expected_action, expected_status),
            Some(audit_log),
        )
    }

    /// Cleanup cache records
    pub fn cleanup_cache_records(&self, keys: &[String]) -> Result<(), CommonUtilsError> {
        self.redis_manager.delete_keys(keys)?;
        Ok(())
    }
}

/// Auth session result
pub struct AuthSession {
    pub user_id: String,
    pub token: String,
    pub auth_cache_key: String,
}

/// Auth record creation parameters
pub struct AuthRecordCreate {
    pub email: String,
    pub additional_fields: Option<Document>,
}

#[derive(Debug, thiserror::Error)]
pub enum CommonUtilsError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] JwtError),
}
```
