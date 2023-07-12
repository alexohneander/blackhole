use async_trait::async_trait;
use libsql_client::Client;

use crate::{interfaces::file_upload::{IFileUploadRepository, RepositoryError}, models::file_upload::FileUpload};

struct SQLDFileUploadRepository {
    db: Client,
}

#[async_trait]
impl IFileUploadRepository for SQLDFileUploadRepository {
    async fn get(&self, id: String) -> Result<FileUpload, RepositoryError> {
        let result = self.db.execute("SELECT * FROM file_uploads WHERE id = ?").await;

        return match result {
            Ok(result) => {
                let mut file_upload = FileUpload {
                    id: "".to_string(),
                    token: "".to_string(),
                    downloaded: false,
                };

                for row in result.rows {
                    file_upload.id = row.get("id").unwrap();
                    file_upload.token = row.get("token").unwrap();
                    file_upload.downloaded = row.get("downloaded").unwrap();
                }

                Ok(file_upload)
            },
            Err(_) => Err(RepositoryError),
        }
    }

    async fn get_all(&self) -> Result<Vec<FileUpload>, RepositoryError> {
        todo!()
    }

    async fn save(&self, task: &FileUpload) -> Result<(), RepositoryError> {
        todo!()
    }

    async fn delete(&self, id: String) -> Result<(), RepositoryError> {
        todo!()
    }
}