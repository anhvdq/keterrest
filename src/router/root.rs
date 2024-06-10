use std::sync::Arc;

use axum::{routing::IntoMakeService, Router};
use tower_http::trace::TraceLayer;

use crate::{
    config::pg_database::PgDatabase, repository::user_repository::UserRepositoryImpl,
    service::user_service::UserServiceImpl,
};

use super::user;

pub fn routes(db_conn: Arc<PgDatabase>) -> IntoMakeService<Router> {
    // Initialize user service
    let user_repository = Arc::new(UserRepositoryImpl::new(db_conn.clone()));
    let user_service_state = Arc::new(UserServiceImpl::new(user_repository.clone()));

    let app_router = Router::new()
        .merge(user::routes(user_service_state.clone()))
        .merge(Router::new().route(
            "/health-check",
            axum::routing::get(|| async { "Still alive" }),
        ));
    app_router
        .layer(TraceLayer::new_for_http()) // Enable logging
        .into_make_service()
}
