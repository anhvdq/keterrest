use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
    http::header::CONTENT_TYPE,
};
use axum_extra::extract::{Form, FormRejection};
use validator::Validate;

use super::api_response::{ApiError, ServiceError};

pub struct JsonOrForm<T>(pub T);

impl<S, T> FromRequest<S> for JsonOrForm<T>
where
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
    T: 'static,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let content_type = req
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                ServiceError::BadRequest("Content-type header is required".to_string())
            })?;

        if content_type.starts_with("application/json") {
            let Json(payload) = Json::<T>::from_request(req, state).await?;
            Ok(Self(payload))
        } else if content_type.starts_with("application/x-www-form-urlencoded") {
            let Form(payload) = Form::<T>::from_request(req, state).await?;
            Ok(Self(payload))
        } else {
            Err(ServiceError::BadRequest("Unsupported content type".to_string()).into())
        }
    }
}

pub struct ValidatedJsonOrForm<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJsonOrForm<T>
where
    JsonOrForm<T>: FromRequest<S, Rejection = ApiError>,
    T: 'static + Validate,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let JsonOrForm(payload) = JsonOrForm::<T>::from_request(req, state).await?;
        payload.validate()?;
        Ok(Self(payload))
    }
}
