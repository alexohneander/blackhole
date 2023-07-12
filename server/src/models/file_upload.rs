use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileUpload {
    pub id: String,
    pub token: String,
    pub downloaded: bool,
}