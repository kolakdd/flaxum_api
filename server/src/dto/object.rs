use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::scalar::Id;

#[derive(Serialize, Deserialize, Default, Validate)]
pub struct GetObjectListDto {
    pub parent_id: Option<Id>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateFolderDto {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    pub parent_id: Option<Id>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFileDto {
    pub parent_id: Option<Id>,
}

impl Default for UploadFileDto {
    fn default() -> Self {
        Self { parent_id: None }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DownloadFileDto {
    pub file_id: Id,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct DeleteObjectDto {
    pub file_id: Id,
    pub delete_mark: bool,
    pub hard_delete: bool,
}
