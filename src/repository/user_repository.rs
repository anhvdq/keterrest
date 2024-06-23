use async_trait::async_trait;
use bcrypt;
use sqlx::Error as SqlxError;
use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;

use crate::{
    config::pg_database::{PgDatabase, PgDatabaseTrait},
    model::user::{CreateUserDto, UpdateUserDto, User},
};

pub type UserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn create(&self, user: CreateUserDto) -> Result<User, SqlxError>;
    async fn get(&self, id: i32) -> Result<User, SqlxError>;
    async fn get_by_email(&self, email: String) -> Result<User, SqlxError>;
    async fn get_all(&self) -> Result<Vec<User>, SqlxError>;
    async fn update(&self, id: i32, user: UpdateUserDto) -> Result<User, SqlxError>;
    async fn delete(&self, id: i32) -> Result<bool, SqlxError>;
}

pub struct UserRepositoryImpl {
    hash_cost: u32,
    db_conn: Arc<PgDatabase>,
}

impl UserRepositoryImpl {
    pub fn new(hash_cost: u32, conn: Arc<PgDatabase>) -> Self {
        UserRepositoryImpl {
            hash_cost,
            db_conn: conn,
        }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepositoryImpl {
    async fn create(&self, user: CreateUserDto) -> Result<User, SqlxError> {
        // Must include 'RETURNING *' to return the new record
        sqlx::query_as(
            r#"INSERT INTO users(name, age, email, password) VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(user.name)
        .bind(user.age)
        .bind(user.email)
        .bind(bcrypt::hash(user.password, self.hash_cost).unwrap())
        .fetch_one(self.db_conn.get_pool())
        .await
    }

    async fn get(&self, id: i32) -> Result<User, SqlxError> {
        sqlx::query_as(r#"SELECT * FROM users WHERE id = $1"#)
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await
    }

    async fn get_by_email(&self, email: String) -> Result<User, SqlxError> {
        sqlx::query_as(r#"SELECT * FROM users WHERE email = $1"#)
            .bind(email)
            .fetch_one(self.db_conn.get_pool())
            .await
    }

    async fn get_all(&self) -> Result<Vec<User>, SqlxError> {
        sqlx::query_as(r#"SELECT * FROM users"#)
            .fetch_all(self.db_conn.get_pool())
            .await
    }

    async fn update(&self, id: i32, user: UpdateUserDto) -> Result<User, SqlxError> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE users SET ");
        let mut separated = query_builder.separated(", ");

        separated.push("name = ").push_bind_unseparated(user.name);
        separated.push("age = ").push_bind_unseparated(user.age);
        separated
            .push("password = ")
            .push_bind_unseparated(bcrypt::hash(user.password, self.hash_cost).unwrap());

        let query = query_builder
            .push(" WHERE id = ")
            .push_bind(id)
            .push(" RETURNING *")
            .build_query_as();

        query.fetch_one(self.db_conn.get_pool()).await
    }

    async fn delete(&self, id: i32) -> Result<bool, SqlxError> {
        let result = sqlx::query(r#"DELETE FROM users WHERE id = $1"#)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await;
        result.map(|r| r.rows_affected() > 0)
    }
}
