use serde::{Deserialize, Serialize};

use super::permission::Permission;

///
/// Entity struct, reflect all columns in the table
///
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub email: String,
    pub password: String,

    // Relationship to permission table
    #[sqlx(json, default)]
    pub permissions: Vec<Permission>,
}

///
/// DTO structs, reflect request / response data
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub password: String,

    // Ids of permissions
    pub permissions: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub name: String,
    pub age: i32,
    pub password: String,

    // Ids of permissions
    pub permissions: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadUserDto {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub email: String,

    // All permissions
    pub permissions: Vec<String>,
}

impl From<User> for ReadUserDto {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            age: value.age,
            email: value.email,
            permissions: value.permissions.into_iter().map(|p| p.name).collect()
        }
    }
}
