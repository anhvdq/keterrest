use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};

#[derive(TryFromMultipart)]
pub struct FileUploadDto {
    pub image: FieldData<Bytes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResponseDto {
    pub image: String,
}
