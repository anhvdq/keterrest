use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use sqlx::{Error, Pool, Postgres};

use super::settings;

pub trait PgDatabaseTrait {
    async fn init() -> Result<Self, Error>
    where
        Self: Sized;
    fn get_pool(&self) -> &Pool<Postgres>;
}

pub struct PgDatabase {
    pool: Pool<Postgres>,
}
impl PgDatabaseTrait for PgDatabase {
    async fn init() -> Result<Self, Error> {
        let host = settings::pg_database_host();
        let port = settings::pg_database_port();
        let db = settings::pg_database_db();
        let username = settings::pg_database_username();
        let password = settings::pg_database_password();

        let mut options = PgConnectOptions::new()
            .host(&host)
            .port(port)
            .database(&db)
            .username(&username);
        if let Some(pass) = password {
            options = options.password(&pass);
        }
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;
        Ok(Self { pool })
    }
    fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
