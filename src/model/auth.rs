use serde::{Deserialize, Serialize};

///
/// DTO structs, reflect request / response data
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentialDto {
    pub access_token: String,
}

#[derive(Clone)]
pub struct AuthInfo {
    pub user_id: i32,
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}
