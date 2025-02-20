use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UserLoginDto {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(min = 6, max = 64))]
    pub password: String,
}



#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserMeDto {
    #[validate(length(min = 3, max = 31))]
    pub name_1: Option<String>,
    #[validate(length(min = 3, max = 31))]
    pub name_2: Option<String>,
    #[validate(length(min = 3, max = 31))]
    pub name_3: Option<String>,
}



#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordDto {
    #[validate(length(min = 3, max = 31))]
    pub new_password: String,
}



#[derive(Serialize, sqlx::FromRow)]
pub struct CreateUserOut {
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

