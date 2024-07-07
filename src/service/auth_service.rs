use axum::async_trait;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use sqlx::Error as SqlxError;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::model::auth::PermissionType;
use crate::util::constants::ROOT_USER_ID;
use crate::{
    model::auth::{AuthCredentialDto, AuthInfo, LoginDto, TokenClaims},
    repository::user_repository::UserRepository,
    util::api_response::{ApiError, ServiceError},
};

pub type AuthService = Arc<dyn AuthServiceTrait + Send + Sync>;

#[async_trait]
pub trait AuthServiceTrait {
    async fn login(&self, user: LoginDto) -> Result<AuthCredentialDto, ApiError>;
    async fn extract_auth_info(&self, token: &str) -> Result<AuthInfo, ApiError>;
}

pub struct AuthServiceImpl {
    root_user: String,
    root_password: String,
    secret: String,
    expire_duration: u64,
    user_repository: UserRepository,
}

impl AuthServiceImpl {
    pub fn new(
        root_user: String,
        root_password: String,
        secret: String,
        expire_duration: u64,
        user_repository: UserRepository,
    ) -> Self {
        AuthServiceImpl {
            root_user,
            root_password,
            secret,
            expire_duration,
            user_repository,
        }
    }

    fn generate_access_token(&self, id: i32, email: String) -> Result<String, ServiceError> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Return the epoch time");
        // Issue at
        let iat = now.as_secs() as i64;
        // Expire time
        let exp = (now + Duration::from_secs(self.expire_duration)).as_secs() as i64;

        let claims = TokenClaims {
            sub: id,
            email,
            iat,
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|e| ServiceError::FailedTokenCreation(e.to_string()))
    }
}

#[async_trait]
impl AuthServiceTrait for AuthServiceImpl {
    async fn login(&self, user: LoginDto) -> Result<AuthCredentialDto, ApiError> {
        let (id, email, password) = if user.email == self.root_user {
            (
                ROOT_USER_ID,
                self.root_user.clone(),
                self.root_password.clone(),
            )
        } else {
            let existing_user = self
                .user_repository
                .get_by_email(user.email.clone())
                .await
                .map_err(|e| match e {
                    SqlxError::Database(msg) => ServiceError::Database(msg.to_string()),
                    SqlxError::RowNotFound => {
                        ServiceError::NotFound(format!("No user found with email: {}", user.email))
                    }
                    _ => ServiceError::Unknown(e.to_string()),
                })?;
            (
                existing_user.id,
                existing_user.email,
                existing_user.password,
            )
        };

        if bcrypt::verify(user.password, &password).unwrap_or(false) {
            Ok(AuthCredentialDto {
                access_token: self.generate_access_token(id, email)?,
            })
        } else {
            Err(ServiceError::InvalidAuthInfo.into())
        }
    }

    async fn extract_auth_info(&self, token: &str) -> Result<AuthInfo, ApiError> {
        let claims = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => ServiceError::ExpiredAuthToken,
            _ => ServiceError::InvalidAuthToken,
        })?
        .claims;
        if claims.sub == ROOT_USER_ID {
            Ok(AuthInfo {
                user_id: ROOT_USER_ID,
                email: claims.email,
                permissions: HashSet::from_iter(PermissionType::VARIANTS.iter().cloned()),
            })
        } else {
            self.user_repository
                .get(claims.sub)
                .await
                .map(|u| AuthInfo {
                    user_id: u.id,
                    email: u.email,
                    permissions: HashSet::from_iter(
                        u.permissions.into_iter().map(|p| p.name.into()),
                    ),
                })
                .map_err(|e| {
                    match e {
                        SqlxError::RowNotFound => ServiceError::InvalidAuthToken,
                        SqlxError::Database(db_err) => ServiceError::Database(db_err.to_string()),
                        _ => ServiceError::Unknown(e.to_string()),
                    }
                    .into()
                })
                .and_then(|u| {
                    if u.email.eq(&claims.email) {
                        Ok(u)
                    } else {
                        Err(ServiceError::InvalidAuthToken.into())
                    }
                })
        }
    }
}
