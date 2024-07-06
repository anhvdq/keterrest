use std::collections::HashSet;

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
    #[allow(dead_code)]
    pub user_id: i32,
    pub email: String,
    pub permissions: HashSet<PermissionType>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PermissionType {
    CreateUser,
    ReadUser,
    UpdateUser,
    DeleteUser,
    Unknown,
}

impl PermissionType {
    pub const VARIANTS: &'static [Self] = &[
        Self::CreateUser,
        Self::ReadUser,
        Self::UpdateUser,
        Self::DeleteUser,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionType::CreateUser => "user.create",
            PermissionType::ReadUser => "user.read",
            PermissionType::UpdateUser => "user.update",
            PermissionType::DeleteUser => "user.delete",
            PermissionType::Unknown => "unknown",
        }
    }
}

impl From<String> for PermissionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "user.create" => PermissionType::CreateUser,
            "user.read" => PermissionType::ReadUser,
            "user.update" => PermissionType::UpdateUser,
            "user.delete" => PermissionType::DeleteUser,
            _ => PermissionType::Unknown,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}
