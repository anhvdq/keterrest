use serde::Deserialize;

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct Permission {
    #[allow(dead_code)]
    pub id: i32,
    pub name: String,
}