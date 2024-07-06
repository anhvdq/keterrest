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
        let mut tx = self.db_conn.get_pool().begin().await?;

        // Must include 'RETURNING *' to return the new record
        let new_user: User = sqlx::query_as(
            r#"INSERT INTO users(name, age, email, password) VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(user.name)
        .bind(user.age)
        .bind(user.email)
        .bind(bcrypt::hash(user.password, self.hash_cost).unwrap())
        .fetch_one(&mut *tx)
        .await?;

        if !user.permissions.is_empty() {
            let mut insert_permissions_builder: QueryBuilder<Postgres> =
                QueryBuilder::new(r#"INSERT INTO users_permissions(user_id, permission_id) "#);
            insert_permissions_builder.push_values(user.permissions, |mut b, permission| {
                b.push_bind(new_user.id).push_bind(permission);
            });
            insert_permissions_builder.build().execute(&mut *tx).await?;
        }

        tx.commit().await?;

        self.get(new_user.id).await
    }

    async fn get(&self, id: i32) -> Result<User, SqlxError> {
        sqlx::query_as(
            r#"SELECT u.id, u.name, u.age, u.email, u.password, 
                COALESCE(
                    JSON_AGG(JSONB_BUILD_OBJECT('id', p.id, 'name', p.name)) 
                    FILTER (WHERE p.id IS NOT NULL),
                '[]') as permissions
            FROM users u
				LEFT JOIN users_permissions up ON (u.id = up.user_id)
				LEFT JOIN permissions p ON (p.id = up.permission_id)
            WHERE u.id = $1
            GROUP BY u.id, u.name, u.age, u.email, u.password"#,
        )
        .bind(id)
        .fetch_one(self.db_conn.get_pool())
        .await
    }

    async fn get_by_email(&self, email: String) -> Result<User, SqlxError> {
        sqlx::query_as(
            r#"SELECT u.id, u.name, u.age, u.email, u.password, 
                COALESCE(
                    JSON_AGG(JSONB_BUILD_OBJECT('id', p.id, 'name', p.name)) 
                    FILTER (WHERE p.id IS NOT NULL),
                '[]') as permissions
            FROM users u
				LEFT JOIN users_permissions up ON (u.id = up.user_id)
				LEFT JOIN permissions p ON (p.id = up.permission_id)
            WHERE u.email = $1
            GROUP BY u.id, u.name, u.age, u.email, u.password"#,
        )
        .bind(email)
        .fetch_one(self.db_conn.get_pool())
        .await
    }

    async fn get_all(&self) -> Result<Vec<User>, SqlxError> {
        sqlx::query_as(
            r#"SELECT u.id, u.name, u.age, u.email, u.password, 
                COALESCE(
                    JSON_AGG(JSONB_BUILD_OBJECT('id', p.id, 'name', p.name)) 
                    FILTER (WHERE p.id IS NOT NULL),
                '[]') as permissions
            FROM users u
				LEFT JOIN users_permissions up ON (u.id = up.user_id)
				LEFT JOIN permissions p ON (p.id = up.permission_id)
            GROUP BY u.id, u.name, u.age, u.email, u.password"#,
        )
        .fetch_all(self.db_conn.get_pool())
        .await
    }

    async fn update(&self, id: i32, user: UpdateUserDto) -> Result<User, SqlxError> {
        let mut tx = self.db_conn.get_pool().begin().await?;

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
            .build();

        query.fetch_one(&mut *tx).await?;

        if let Some(permissions) = user.permissions {
            sqlx::query(r#"DELETE FROM users_permissions WHERE user_id = $1"#)
                .bind(id)
                .execute(&mut *tx)
                .await?;
            if !permissions.is_empty() {
                let mut insert_permissions_builder: QueryBuilder<Postgres> =
                    QueryBuilder::new(r#"INSERT INTO users_permissions(user_id, permission_id) "#);
                insert_permissions_builder.push_values(permissions, |mut b, permission| {
                    b.push_bind(id).push_bind(permission);
                });
                insert_permissions_builder.build().execute(&mut *tx).await?;
            }
        }
        tx.commit().await?;

        self.get(id).await
    }

    async fn delete(&self, id: i32) -> Result<bool, SqlxError> {
        let result = sqlx::query(r#"DELETE FROM users WHERE id = $1"#)
            .bind(id)
            .execute(self.db_conn.get_pool())
            .await;
        result.map(|r| r.rows_affected() > 0)
    }
}
