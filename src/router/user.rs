use axum::{
    extract::{Path, State}, routing::MethodRouter, Extension, Json, Router
};
use axum_extra::extract::WithRejection;

use crate::service::user_service::UserService;
use crate::util::api_response::{ApiError, ApiSuccess};
use crate::{
    model::{
        auth::{AuthInfo, PermissionType},
        user::{CreateUserDto, ReadUserDto, UpdateUserDto},
    },
    permission_required,
};
use axum::routing::{delete, get, post, put};

pub fn routes(state: UserService) -> Router {
    Router::new()
        .route(
            "/users",
            MethodRouter::new()
                .merge(get(get_users).layer(permission_required!(PermissionType::ReadUser)))
                .merge(post(create_user).layer(permission_required!(PermissionType::CreateUser))),
        )
        .route(
            "/users/:id",
            MethodRouter::new()
                .merge(get(get_user).layer(permission_required!(PermissionType::ReadUser)))
                .merge(put(update_user).layer(permission_required!(PermissionType::UpdateUser)))
                .merge(delete(delete_user).layer(permission_required!(PermissionType::DeleteUser))),
        )
        .with_state(state)
}

async fn create_user(
    Extension(auth_info): Extension<AuthInfo>,
    State(service): State<UserService>,
    WithRejection(Json(user), _): WithRejection<Json<CreateUserDto>, ApiError>,
) -> Result<Json<ApiSuccess<ReadUserDto>>, ApiError> {
    auth_info.permissions.iter().for_each(|p| tracing::info!("{}", p.as_str()));
    service.create(user).await.map(ApiSuccess::new).map(Json)
}

async fn get_user(
    State(service): State<UserService>,
    WithRejection(Path(id), _): WithRejection<Path<u32>, ApiError>,
) -> Result<Json<ApiSuccess<ReadUserDto>>, ApiError> {
    service.get(id).await.map(ApiSuccess::new).map(Json)
}

async fn get_users(
    State(service): State<UserService>,
) -> Result<Json<ApiSuccess<Vec<ReadUserDto>>>, ApiError> {
    service.get_all().await.map(ApiSuccess::new).map(Json)
}

async fn update_user(
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

async fn delete_user(
    State(service): State<UserService>,
    WithRejection(Path(id), _): WithRejection<Path<u32>, ApiError>,
) -> Result<Json<ApiSuccess<bool>>, ApiError> {
    service.delete(id).await.map(ApiSuccess::new).map(Json)
}
