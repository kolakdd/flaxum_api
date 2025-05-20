use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::scalar::Id;


#[derive(Serialize, Deserialize, Validate)]
pub struct CreateRobotDto {
    #[validate(length(min = 2, max = 64))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct DeleteRobotDto {
    pub id: Id,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct CreateRobotOut {
    pub id: Id,
    pub name: String,
    pub token: String,
}