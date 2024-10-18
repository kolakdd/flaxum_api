use std::sync::Arc;
use chrono;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{domain::user::model::User, route::AppState, scalar::Id, utils::crypto};


use validator::Validate;


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    pub email: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
}


pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateUserDto>,
) -> impl IntoResponse {
    if let Err(e) = body.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    let password = body.password.to_string();
    let hash = crypto::hash(password).await;

    if let Err(e) = hash {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    }
    let hash = hash.unwrap();
    let id = uuid::Uuid::new_v4();
    let res = sqlx::query_as!(
        User,
        "INSERT INTO users (id, email, password, create_date) VALUES ($1, $2, $3, $4) RETURNING *",
        id,
        body.email.to_string(),
        hash,
        chrono::offset::Utc::now(),
    )
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(user) => Ok((
            StatusCode::CREATED,
            Json(json!({ "status": "success", "data": user })),
        )),
        Err(_) => return Err((StatusCode::NOT_FOUND, "Failed to create user".to_string())),
    }
}

pub async fn access_token() {}

pub async fn refresh_token() {}
