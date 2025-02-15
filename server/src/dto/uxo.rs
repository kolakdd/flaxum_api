use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;
use sqlx::{postgres::PgRow, FromRow};

use sqlx::Row;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct GiveAccessDto {
    pub can_read: bool,
    pub can_edit: bool,
    pub can_delete: bool,
    #[validate(email)]
    pub recipient_email: String,
}
