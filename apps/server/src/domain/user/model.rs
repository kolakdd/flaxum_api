use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::scalar::Id;
use sqlx::{postgres::PgRow, Row};


#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct CreateUserOut{
    pub email: String,
    pub create_date: Option<chrono::DateTime<chrono::Utc>>,

}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct User {
    pub id: Id,
    pub email: String,
    pub password: String,
    #[serde(rename = "createDate")]
    pub create_date: Option<chrono::DateTime<chrono::Utc>>,
    pub storage_size: i64,
}


impl User {
    pub fn new(
        id: Id,
        email: String,
        password: String,
        create_date: Option<chrono::DateTime<chrono::Utc>>,
        storage_size: i64,
    ) -> User {
        User { id, email, password, create_date, storage_size }
    }
}


impl From<PgRow> for User {
    fn from(value: PgRow) -> Self {
        User::new(
            value.get("id"),
            value.get("email"),
            value.get("password"),
            value.get("create_date"),
            value.get("storage_size"),
        )
    }
}