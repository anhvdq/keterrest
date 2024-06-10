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
        return ApiSuccess { data };
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
        return ApiError { message, status };
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
pub enum InternalError {
    Database(String),
    NotFound,
    Unknown,
}

impl Into<ApiError> for InternalError {
    fn into(self) -> ApiError {
        match self {
            InternalError::Database(msg) => {
                ApiError::new(Some(msg), StatusCode::INTERNAL_SERVER_ERROR.as_u16())
            }
            InternalError::NotFound => {
                ApiError::new(None, StatusCode::INTERNAL_SERVER_ERROR.as_u16())
            }
            InternalError::Unknown => ApiError::new(
                Some(String::from("Undefined error")),
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            ),
        }
    }
}
