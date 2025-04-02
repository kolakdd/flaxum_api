use serde::{Deserialize, Serialize};

use validator::Validate;

use crate::scalar::Id;

#[derive(Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GiveAccessDto {
    pub can_read: bool,
    pub can_edit: bool,
    pub can_delete: bool,
    #[validate(email)]
    pub recipient_email: String,
}


#[derive(Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAccessDtoIn {
    pub recipient_id: Id,
}



#[derive(Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAccessDto {
    pub obj_id: Id,
    pub recipient_id: Id,
}
