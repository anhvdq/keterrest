use serde::{Deserialize, Serialize};

///
/// Entity struct, reflect all columns in the table
/// 
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

///
/// DTO structs, reflect request / response data
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub age: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub name: String,
    pub age: i32,
}