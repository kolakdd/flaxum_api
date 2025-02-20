use serde::{Deserialize, Serialize};

use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GiveAccessDto {
    pub can_read: bool,
    pub can_edit: bool,
    pub can_delete: bool,
    #[validate(email)]
    pub recipient_email: String,
}
