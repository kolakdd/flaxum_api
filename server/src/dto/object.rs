use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::scalar::Id;

#[derive(Serialize, Deserialize, Default, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectListDto {
    pub parent_id: Option<Id>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderDto {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    pub parent_id: Option<Id>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileDto {
    pub parent_id: Option<Id>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadFileDto {
    pub file_id: Id,
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteObjectDto {
    pub file_id: Id,
    pub delete_mark: bool,
    pub hard_delete: bool,
}
