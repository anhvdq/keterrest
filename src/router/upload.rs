use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    routing::{MethodRouter, post},
};
use axum_extra::extract::WithRejection;
use axum_typed_multipart::TypedMultipart;

use crate::{
    model::file::{FileResponseDto, FileUploadDto},
    service::file_service::FileService,
    util::api_response::{ApiError, ApiSuccess},
};

pub fn routes(state: FileService) -> Router {
    Router::new()
        .route(
            "/upload",
            MethodRouter::new().merge(post(upload_file).layer(DefaultBodyLimit::max(10 * 1024))),
        )
        .with_state(state)
}
async fn upload_file(
    State(service): State<FileService>,
    WithRejection(TypedMultipart(file), _): WithRejection<TypedMultipart<FileUploadDto>, ApiError>,
) -> Result<Json<ApiSuccess<FileResponseDto>>, ApiError> {
    service
        .save_image(file.image)
        .await
        .map(ApiSuccess::new)
        .map(Json)
}
