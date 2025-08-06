use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::{Router, routing::IntoMakeService};
use tower_http::trace::TraceLayer;

use crate::config::settings::{
    jwt_expire_duration, jwt_hash_cost, jwt_secret, root_password, root_user,
};
use crate::middleware::auth::authorize;
use crate::repository::file_repository::{FileRepository, FileRepositoryImpl};
use crate::repository::user_repository::UserRepository;
use crate::service::auth_service::{AuthService, AuthServiceImpl};
use crate::service::file_service::{FileService, FileServiceImpl};
use crate::service::user_service::UserService;
use crate::{
    config::pg_database::PgDatabase, repository::user_repository::UserRepositoryImpl,
    service::user_service::UserServiceImpl,
};

use super::user;
use super::{auth, upload};

pub fn routes(db_conn: Arc<PgDatabase>) -> IntoMakeService<Router> {
    // Initialize user service
    let user_repository: UserRepository = Arc::new(UserRepositoryImpl::new(
        jwt_hash_cost(),
        Arc::clone(&db_conn),
    ));
    let user_service: UserService = Arc::new(UserServiceImpl::new(Arc::clone(&user_repository)));

    let auth_service: AuthService = Arc::new(AuthServiceImpl::new(
        root_user(),
        bcrypt::hash(root_password(), jwt_hash_cost()).expect("Root password should be hashable"),
        jwt_secret(),
        jwt_expire_duration(),
        Arc::clone(&user_repository),
    ));

    let file_repository: FileRepository = Arc::new(FileRepositoryImpl::new());
    let file_service: FileService = Arc::new(FileServiceImpl::new(Arc::clone(&file_repository)));

    let protected = Router::new()
        .merge(user::routes(Arc::clone(&user_service)))
        .merge(upload::routes(Arc::clone(&file_service)))
        .route_layer(from_fn_with_state(Arc::clone(&auth_service), authorize));

    let public = Router::new()
        .merge(auth::routes(Arc::clone(&auth_service)))
        .merge(Router::new().route(
            "/health-check",
            axum::routing::get(|| async { format!("App version: {}", env!("CARGO_PKG_VERSION")) }),
        ));

    let app_router = Router::new().merge(public).merge(protected);
    app_router
        .layer(TraceLayer::new_for_http()) // Enable logging
        .into_make_service()
}
