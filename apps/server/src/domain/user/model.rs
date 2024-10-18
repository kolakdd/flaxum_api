use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    #[serde(rename = "createDate")]
    pub create_date: Option<chrono::DateTime<chrono::Utc>>,
    pub storage_size: i64,
}