use crate::{domain::user::model::{User, CreateUserOut}, route::AppState, scalar::Id, utils::{crypto, jwt}};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

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
    
    let res = sqlx::query_as!(
        CreateUserOut,
        "INSERT INTO users (id, email, password, create_date) VALUES ($1, $2, $3, $4) RETURNING email, create_date",
        Id::new_v4(),
        body.email.to_string(),
        hash,
        chrono::Utc::now(),
    )
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(user) => Ok((
            StatusCode::CREATED,
            Json(json!({ "status": "success", "data": user})),
        )),
        Err(_) => Err((StatusCode::NOT_FOUND, "Failed to create user".to_string())),
    }
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}


pub async fn access_token(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginDto>,
) -> impl IntoResponse {
    if let Err(e) = body.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    let email = body.email.to_string();

    let res = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(user) => {
            let password = body.password.to_string();
            let hash = user.password;

            let token = jwt::sign(user.id).unwrap();

            match crypto::verify(password, hash).await {
                Ok(true) => Ok((StatusCode::OK, Json(json!({ "status": "success", "token": token })))),
                Ok(false) => Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string())),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            "User with email does not exist".to_string(),
        )),
    }
}


pub async fn refresh_token() {}
