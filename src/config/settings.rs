/// Load all environment params specified in .env file
/// Should be called first on the start of main function
pub fn init() {
    dotenv::dotenv().ok();
}

/// Used to get the specified env param
///
///  * `param_name` - Parameter name to get
fn get(param_name: &str) -> Option<String> {
    std::env::var(param_name).ok()
}

/// Used to get the specified env param, and return 'default' if not found
///
///  * `param_name` - Parameter name to get
fn get_or_default(param_name: &str, default: String) -> String {
    std::env::var(param_name).unwrap_or(default)
}

/// --- Define new environment parameter below
pub fn root_user() -> String {
    get("APP_ROOT_USER").expect("'APP_ROOT_USER' is not defined in the environment.")
}

pub fn root_password() -> String {
    get("APP_ROOT_PASSWORD").expect("'APP_ROOT_PASSWORD' is not defined in the environment.")
}

pub fn api_port() -> u16 {
    get_or_default("APP_PORT", 3000.to_string())
        .parse()
        .expect("'APP_PORT' must be a valid port number")
}

pub fn pg_database_host() -> String {
    get("PG_DATABASE_HOST").expect("'PG_DATABASE_HOST' is not defined in the environment.")
}

pub fn pg_database_port() -> u16 {
    get("PG_DATABASE_PORT")
        .expect("'PG_DATABASE_PORT' is not defined in the environment.")
        .parse()
        .expect("'PG_DATABASE_PORT' must be a valid port number")
}

pub fn pg_database_db() -> String {
    get("PG_DATABASE_DB").expect("'PG_DATABASE_DB' is not defined in the environment.")
}

pub fn pg_database_username() -> String {
    get("PG_DATABASE_USERNAME").expect("'PG_DATABASE_USERNAME' is not defined in the environment.")
}

pub fn pg_database_password() -> Option<String> {
    get("PG_DATABASE_PASSWORD")
}

pub fn jwt_secret() -> String {
    get("JWT_SECRET").expect("'JWT_SECRET' is not defined in the environment.")
}

pub fn jwt_expire_duration() -> u64 {
    get_or_default("JWT_EXPIRE_DURATION", 3600.to_string())
        .parse()
        .expect("'JWT_EXPIRE_DURATION' must be a integer number")
}

pub fn jwt_hash_cost() -> u32 {
    get_or_default("JWT_HASH_COST", 4.to_string())
        .parse()
        .expect("'JWT_HASH_COST' must be a integer number")
}
