use axum::{async_trait, body::Bytes};
use axum_typed_multipart::FieldData;
use std::io::Error as SqlxError;
use std::sync::Arc;

use crate::model::file::FileResponseDto;

pub type FileRepository = Arc<dyn FileRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait FileRepositoryTrait {
    async fn save_image(&self, file: FieldData<Bytes>) -> Result<FileResponseDto, SqlxError>;
}

pub struct FileRepositoryImpl;
impl FileRepositoryImpl {
    pub fn new() -> Self {
        FileRepositoryImpl {}
    }
}

#[async_trait]
impl FileRepositoryTrait for FileRepositoryImpl {
    async fn save_image(&self, file: FieldData<Bytes>) -> Result<FileResponseDto, SqlxError> {
        let file_name = file.metadata.file_name.unwrap_or(String::from("temp"));
        let path: String = format!("./data/{}", file_name);
        tokio::fs::write(&path, file.contents)
            .await
            .map(|_| FileResponseDto { image: path })
    }
}
