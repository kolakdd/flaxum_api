use serde::{Deserialize, Serialize};

use sqlx::Row;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UserLoginDto {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(min = 6, max = 64))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(min = 6, max = 64))]
    pub password: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct CreateUserOut {
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

impl CreateUserOut {
    pub fn new(email: String, created_at: chrono::NaiveDateTime) -> CreateUserOut {
        CreateUserOut { email, created_at }
    }
}
