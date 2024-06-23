use axum::{
    extract::{Path, State},
    Json, Router,
};
use axum_extra::extract::WithRejection;

use crate::model::user::{CreateUserDto, ReadUserDto, UpdateUserDto};
use crate::service::user_service::UserService;
use crate::util::api_response::{ApiError, ApiSuccess};

pub fn routes(state: UserService) -> Router {
    Router::new()
        .route("/users", axum::routing::get(get_all).post(create))
        .route(
            "/users/:id",
            axum::routing::get(get).put(update).delete(delete),
        )
        .with_state(state)
}

async fn create(
    State(service): State<UserService>,
    WithRejection(Json(user), _): WithRejection<Json<CreateUserDto>, ApiError>,
) -> Result<Json<ApiSuccess<ReadUserDto>>, ApiError> {
    service.create(user).await.map(ApiSuccess::new).map(Json)
}

async fn get(
    State(service): State<UserService>,
    WithRejection(Path(id), _): WithRejection<Path<u32>, ApiError>,
) -> Result<Json<ApiSuccess<ReadUserDto>>, ApiError> {
    service.get(id).await.map(ApiSuccess::new).map(Json)
}

async fn get_all(
    State(service): State<UserService>,
) -> Result<Json<ApiSuccess<Vec<ReadUserDto>>>, ApiError> {
    service.get_all().await.map(ApiSuccess::new).map(Json)
}

async fn update(
    State(service): State<UserService>,
    WithRejection(Path(id), _): WithRejection<Path<u32>, ApiError>,
    WithRejection(Json(user), _): WithRejection<Json<UpdateUserDto>, ApiError>,
) -> Result<Json<ApiSuccess<ReadUserDto>>, ApiError> {
    service
        .update(id, user)
        .await
        .map(ApiSuccess::new)
        .map(Json)
}

async fn delete(
    State(service): State<UserService>,
    WithRejection(Path(id), _): WithRejection<Path<u32>, ApiError>,
) -> Result<Json<ApiSuccess<bool>>, ApiError> {
    service.delete(id).await.map(ApiSuccess::new).map(Json)
}
