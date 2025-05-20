use crate::scalar::Id;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};


#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RobotObject {
    pub id: Id,
    pub robot_id: Id,
    pub name: String,
    pub size: Option<i64>,
    pub created_at: NaiveDateTime,
    pub upload_s3: Option<bool>,
    pub decode_key: String,
    pub hash_sha256: Option<String>,
}

#[allow(clippy::too_many_arguments)]
impl RobotObject {
    pub fn new(
        id: Id,
        robot_id: Id,
        name: String,
        size: Option<i64>,
        created_at: NaiveDateTime,
        upload_s3: Option<bool>,
        decode_key: String,
        hash_sha256: Option<String>,
    ) -> RobotObject {
        RobotObject {
            id,
            robot_id,
            name,
            size,
            created_at,
            upload_s3,
            decode_key,
            hash_sha256,
        }
    }
}

impl From<PgRow> for RobotObject {
    fn from(value: PgRow) -> Self {
        RobotObject::new(
            value.get("id"),
            value.get("robot_id"),
            value.get("name"),
            value.get("size"),
            value.get("created_at"),
            value.get("upload_s3"),
            value.get("decode_key"),
            value.get("hash_sha256"),
        )
    }
}
