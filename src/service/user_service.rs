use crate::model::user::{CreateUserDto, UpdateUserDto, User};
use crate::repository::user_repository::UserRepository;
use crate::util::api_response::{ApiError, InternalError};
use async_trait::async_trait;
use sqlx::Error as SqlxError;
use std::sync::Arc;

pub type UserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[async_trait]
pub trait UserServiceTrait {
    async fn create(&self, user: CreateUserDto) -> Result<User, ApiError>;
    async fn get(&self, id: u32) -> Result<User, ApiError>;
    async fn get_all(&self) -> Result<Vec<User>, ApiError>;
    async fn delete(&self, id: u32) -> Result<bool, ApiError>;
    async fn update(&self, id: u32, user: UpdateUserDto) -> Result<User, ApiError>;
}

pub struct UserServiceImpl {
    user_repository: UserRepository,
}

impl UserServiceImpl {
    pub fn new(user_repository: UserRepository) -> Self {
        UserServiceImpl {
            user_repository: user_repository.clone(),
        }
    }
}

#[async_trait]
impl UserServiceTrait for UserServiceImpl {
    async fn create(&self, user: CreateUserDto) -> Result<User, ApiError> {
        self.user_repository
            .create(user)
            .await
            .map_err(|e| match e {
                SqlxError::Database(db_err) => InternalError::Database(db_err.to_string()).into(),
                _ => InternalError::Unknown.into(),
            })
    }

    async fn get(&self, id: u32) -> Result<User, ApiError> {
        self.user_repository
            .get(id as i32)
            .await
            .map_err(|e| match e {
                SqlxError::Database(db_err) => InternalError::Database(db_err.to_string()).into(),
                _ => InternalError::Unknown.into(),
            })
    }

    async fn get_all(&self) -> Result<Vec<User>, ApiError> {
        self.user_repository.get_all().await.map_err(|e| match e {
            SqlxError::Database(db_err) => InternalError::Database(db_err.to_string()).into(),
            _ => InternalError::Unknown.into(),
        })
    }

    async fn delete(&self, id: u32) -> Result<bool, ApiError> {
        self.user_repository
            .delete(id as i32)
            .await
            .map_err(|e| match e {
                SqlxError::Database(db_err) => InternalError::Database(db_err.to_string()).into(),
                _ => InternalError::Unknown.into(),
            })
    }

    async fn update(&self, id: u32, user: UpdateUserDto) -> Result<User, ApiError> {
        self.user_repository
            .update(id as i32, user)
            .await
            .map_err(|e| match e {
                SqlxError::Database(db_err) => InternalError::Database(db_err.to_string()).into(),
                _ => InternalError::Unknown.into(),
            })
    }
}
