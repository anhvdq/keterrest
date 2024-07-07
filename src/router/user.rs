use axum::{
    extract::{Path, State},
    routing::{patch, MethodRouter},
    Json, Router,
};
use axum_extra::extract::WithRejection;

use crate::{
    model::user::UpdateUserPermissionDto,
    util::{
        api_request::ValidatedJsonOrForm,
        api_response::{ApiError, ApiSuccess},
    },
};
use crate::{
    model::{
        auth::{AuthInfo, PermissionType},
        user::{CreateUserDto, ReadUserDto, UpdateUserDto},
    },
    permission_required,
};
use crate::{service::user_service::UserService, util::api_request::JsonOrForm};
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
                .merge(patch(update_user).layer(permission_required!(PermissionType::UpdateUser)))
                .merge(delete(delete_user).layer(permission_required!(PermissionType::DeleteUser))),
        )
        .route(
            "/users/:id/permissions",
            MethodRouter::new().merge(
                put(update_user_permissions)
                    .layer(permission_required!(PermissionType::UpdateUser)),
            ),
        )
        .with_state(state)
}

async fn create_user(
    State(service): State<UserService>,
    WithRejection(ValidatedJsonOrForm(user), _): WithRejection<
        ValidatedJsonOrForm<CreateUserDto>,
        ApiError,
    >,
) -> Result<Json<ApiSuccess<ReadUserDto>>, ApiError> {
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
    WithRejection(ValidatedJsonOrForm(user), _): WithRejection<
        ValidatedJsonOrForm<UpdateUserDto>,
        ApiError,
    >,
) -> Result<Json<ApiSuccess<ReadUserDto>>, ApiError> {
    service
        .update(id, user)
        .await
        .map(ApiSuccess::new)
        .map(Json)
}

async fn update_user_permissions(
    State(service): State<UserService>,
    WithRejection(Path(id), _): WithRejection<Path<u32>, ApiError>,
    WithRejection(JsonOrForm(user), _): WithRejection<
        JsonOrForm<UpdateUserPermissionDto>,
        ApiError,
    >,
) -> Result<Json<ApiSuccess<ReadUserDto>>, ApiError> {
    service
        .update_permissions(id, user)
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
