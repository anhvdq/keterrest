use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

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
            message: Some(format!("{}", value)),
        }
    }
}

impl From<JsonRejection> for ApiError {
    fn from(value: JsonRejection) -> Self {
        ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: Some(format!("{}", value)),
        }
    }
}

#[allow(dead_code)]
pub enum ServiceError {
    Database(String),
    NotFound(String),
    FailedTokenCreation(String),
    InvalidAuthToken,
    MissingAuthToken,
    ExpiredAuthToken,
    Unknown,
}

impl From<ServiceError> for ApiError {
    fn from(val: ServiceError) -> Self {
        match val {
            ServiceError::Database(msg) => {
                ApiError::new(Some(msg), StatusCode::INTERNAL_SERVER_ERROR.as_u16())
            }
            ServiceError::NotFound(msg) => ApiError::new(Some(msg), StatusCode::NOT_FOUND.as_u16()),
            ServiceError::FailedTokenCreation(msg) => {
                ApiError::new(Some(msg), StatusCode::INTERNAL_SERVER_ERROR.as_u16())
            }
            ServiceError::InvalidAuthToken => ApiError::new(
                Some(String::from("Invalid authorization token")),
                StatusCode::UNAUTHORIZED.as_u16(),
            ),
            ServiceError::MissingAuthToken => ApiError::new(
                Some(String::from("Missing authorization token")),
                StatusCode::UNAUTHORIZED.as_u16(),
            ),
            ServiceError::ExpiredAuthToken => ApiError::new(
                Some(String::from("Expired authorization token")),
                StatusCode::UNAUTHORIZED.as_u16(),
            ),
            ServiceError::Unknown => ApiError::new(
                Some(String::from("Undefined error")),
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            ),
        }
    }
}
