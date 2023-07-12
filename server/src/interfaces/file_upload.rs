use std::{error::Error, fmt};
use async_trait::async_trait;
use crate::models::file_upload::FileUpload;

#[async_trait]
pub trait IFileUploadRepository {
    async fn get(&self, id: String) -> Result<FileUpload, RepositoryError>;
    async fn get_all(&self) -> Result<Vec<FileUpload>, RepositoryError>;
    async fn save(&self, file_upload: &FileUpload) -> Result<(), RepositoryError>;
    async fn delete(&self, id: String) -> Result<(), RepositoryError>;
}

#[derive(Debug)]
pub struct RepositoryError;

impl Error for RepositoryError {}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}