use crate::scalar::Id;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;


#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Robot {
    pub id: Id,
    pub creator_id: Id,
    pub name: String,
    pub token: String, 
    pub is_deactivated: bool,
    pub deactivated_at: Option<NaiveDateTime>,
    pub storage_size: i64,
}

#[allow(clippy::too_many_arguments)]
impl Robot {
    pub fn new(
        id: Id,
        creator_id: Id,
        name: String,
        token: String, 
        is_deactivated: bool,
        deactivated_at: Option<NaiveDateTime>,
        storage_size: i64,
    ) -> Robot {
        Robot {
            id,
            creator_id,
            name,
            token,
            is_deactivated,
            deactivated_at,
            storage_size,
        }
    }
}

impl From<PgRow> for Robot {
    fn from(value: PgRow) -> Self {
        Robot::new(
            value.get("id"),
            value.get("creator_id"),
            value.get("name"),
            value.get("token"),
            value.get("is_deactivated"),
            value.get("deactivated_at"),
            value.get("storage_size"),
        )
    }
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RobotsPaginated {
    items: Vec<Robot>,
    limit: i64,
    offset: i64,
    total: i64,
}

impl RobotsPaginated {
    pub fn build(items: Vec<Robot>, limit: i64, offset: i64, total: i64) -> Self {
        Self {
            items,
            limit,
            offset,
            total,
        }
    }
}
