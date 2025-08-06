use axum::{Json, Router, extract::State};
use axum_extra::extract::WithRejection;

use crate::{
    model::auth::{AuthCredentialDto, LoginDto},
    service::auth_service::AuthService,
    util::api_response::{ApiError, ApiSuccess},
};

pub fn routes(service: AuthService) -> Router {
    Router::new()
        .route("/login", axum::routing::post(login))
        .with_state(service)
}

async fn login(
    State(service): State<AuthService>,
    WithRejection(Json(user), _): WithRejection<Json<LoginDto>, ApiError>,
) -> Result<Json<ApiSuccess<AuthCredentialDto>>, ApiError> {
    service.login(user).await.map(ApiSuccess::new).map(Json)
}
