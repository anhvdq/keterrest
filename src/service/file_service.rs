use std::sync::Arc;

use axum::body::Bytes;
use axum_typed_multipart::{FieldData, async_trait};

use crate::{
    model::file::FileResponseDto,
    repository::file_repository::FileRepository,
    util::api_response::{ApiError, ServiceError},
};

pub type FileService = Arc<dyn FileServiceTrait + Send + Sync>;

#[async_trait]
pub trait FileServiceTrait {
    async fn save_image(&self, file: FieldData<Bytes>) -> Result<FileResponseDto, ApiError>;
}

pub struct FileServiceImpl {
    file_repository: FileRepository,
}

impl FileServiceImpl {
    pub fn new(file_repository: FileRepository) -> Self {
        FileServiceImpl { file_repository }
    }
}

#[async_trait]
impl FileServiceTrait for FileServiceImpl {
    async fn save_image(&self, file: FieldData<Bytes>) -> Result<FileResponseDto, ApiError> {
        self.file_repository
            .save_image(file)
            .await
            .map_err(|err| ServiceError::Unknown(err.to_string()).into())
    }
}
