use axum::{
    Json,
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::FormRejection;
use axum_typed_multipart::TypedMultipartError;
use serde::Serialize;
use validator::ValidationErrors;

///
/// Wrapper struct for success responses
///
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ApiSuccess<T: Serialize> {
    data: T,
}

impl<T: Serialize> ApiSuccess<T>
where
    T: Serialize,
{
    pub(crate) fn new(data: T) -> Self {
        ApiSuccess { data }
    }
}

///
/// Wrapper struct for error responses
///
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ApiError {
    message: Option<String>,
    #[serde(rename = "code")] // Rename the field in JSON
    status: u16,
}

impl ApiError {
    pub fn new(message: Option<String>, status: u16) -> Self {
        ApiError { message, status }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        )
            .into_response()
    }
}

///
/// Error conversion
///
impl From<PathRejection> for ApiError {
    fn from(value: PathRejection) -> Self {
        ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: Some(value.body_text()),
        }
    }
}

impl From<JsonRejection> for ApiError {
    fn from(value: JsonRejection) -> Self {
        ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: Some(value.body_text()),
        }
    }
}

impl From<FormRejection> for ApiError {
    fn from(value: FormRejection) -> Self {
        ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: Some(value.to_string()),
        }
    }
}

impl From<TypedMultipartError> for ApiError {
    fn from(value: TypedMultipartError) -> Self {
        ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: Some(value.to_string()),
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(value: ValidationErrors) -> Self {
        ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: Some(value.to_string()),
        }
    }
}

#[allow(dead_code)]
pub enum ServiceError {
    Database(String),
    NotFound(String),
    BadRequest(String),
    FailedTokenCreation(String),
    InvalidAuthToken,
    InvalidAuthInfo,
    MissingAuthToken,
    MissingRequiredPermission(String),
    ExpiredAuthToken,
    Unknown(String),
}

impl From<ServiceError> for ApiError {
    fn from(val: ServiceError) -> Self {
        match val {
            ServiceError::Database(msg) => {
                ApiError::new(Some(msg), StatusCode::INTERNAL_SERVER_ERROR.as_u16())
            }
            ServiceError::NotFound(msg) => ApiError::new(Some(msg), StatusCode::NOT_FOUND.as_u16()),
            ServiceError::BadRequest(msg) => {
                ApiError::new(Some(msg), StatusCode::BAD_REQUEST.as_u16())
            }
            ServiceError::FailedTokenCreation(msg) => {
                ApiError::new(Some(msg), StatusCode::INTERNAL_SERVER_ERROR.as_u16())
            }
            ServiceError::InvalidAuthToken => ApiError::new(
                Some(String::from("Invalid authorization token")),
                StatusCode::UNAUTHORIZED.as_u16(),
            ),
            ServiceError::InvalidAuthInfo => ApiError::new(
                Some(String::from("Invalid username or password")),
                StatusCode::UNAUTHORIZED.as_u16(),
            ),
            ServiceError::MissingAuthToken => ApiError::new(
                Some(String::from("Missing authorization token")),
                StatusCode::UNAUTHORIZED.as_u16(),
            ),
            ServiceError::MissingRequiredPermission(permission) => ApiError::new(
                Some(format!("Missing required permission: {permission}")),
                StatusCode::FORBIDDEN.as_u16(),
            ),
            ServiceError::ExpiredAuthToken => ApiError::new(
                Some(String::from("Expired authorization token")),
                StatusCode::UNAUTHORIZED.as_u16(),
            ),
            ServiceError::Unknown(msg) => ApiError::new(
                Some(format!("Undefined error: {msg}")),
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            ),
        }
    }
}
