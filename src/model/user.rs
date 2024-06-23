use serde::{Deserialize, Serialize};

///
/// Entity struct, reflect all columns in the table
///
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub email: String,
    pub password: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub name: String,
    pub age: i32,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadUserDto {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub email: String,
}

impl From<User> for ReadUserDto {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            age: value.age,
            email: value.email,
        }
    }
}
