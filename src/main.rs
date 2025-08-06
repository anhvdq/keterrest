use std::sync::Arc;

use config::{
    pg_database::{self, PgDatabaseTrait},
    settings,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod middleware;
mod model;
mod repository;
mod router;
mod service;
mod util;

#[tokio::main]
async fn main() {
    settings::init();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_env("APP_LOG").unwrap_or_else(|_| {
                "keter_rest=debug,tower_http=debug,axum::rejection=debug,tower_http::trace::make_span=debug".into()
            }),
        )
        .init();

    // Create database connection
    let db_conn = pg_database::PgDatabase::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {e}"));

    // Build app, include all routes
    let app = router::root::routes(Arc::new(db_conn));
    let port = settings::api_port();
    let version = env!("CARGO_PKG_VERSION");

    // Run the app with hyper, listening globally
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    tracing::info!("Listening on port: {}", listener.local_addr().unwrap());
    tracing::info!("App version: {}", version);

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| panic!("Server error: {e}"));
}
