use crate::{
    domain::user::model::{CreateUserOut, ExistsOut, UserRole, User},
    route::AppState,
    scalar::Id,
    utils::crypto,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use crate::utils::jwt;
use validator::Validate;

/// Body для .../user/register
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
}

/// Регистрация пользователя по Логину и Паролю
pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateUserDto>,
) -> impl IntoResponse {
    if let Err(e) = body.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }
    let q = r#"SELECT EXISTS(SELECT 1 FROM "User" WHERE email=$1)"#;

    let res = sqlx::query_as::<_, ExistsOut>(q)
        .bind(body.email.clone())
        .fetch_one(&state.db)
        .await;
    match res {
        Ok(exist) => {
            if exist.exists {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "User email already exist".to_string(),
                ));
            };
        }
        Err(err) => {
            eprintln!("Database error: {err}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create user".to_string(),
            ));
        }
    }
    let hash_password = crypto::hash(body.password.to_string()).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Hashing error: {e}"),
        )
    })?;
    let q = r#"
        INSERT INTO "User" (id, name_1, email, hash_password, role_type)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING email, created_at
    "#;
    let res = sqlx::query_as::<_, CreateUserOut>(q)
        .bind(Id::new_v4())
        .bind("name_1_mock")
        .bind(body.email.clone())
        .bind(hash_password)
        .bind(UserRole::User)
        .fetch_one(&state.db)
        .await;
    match res {
        Ok(user) => Ok((
            StatusCode::CREATED,
            Json(json!({ "status": "success", "data": user })),
        )),
        Err(err) => {
            eprintln!("Database error: {err}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create user".to_string(),
            ))
        }
    }
}

/// Body для .../user/login
#[derive(Serialize, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 128))]

    pub password: String,
}

/// Авторизация пользователя по Логину и Паролю
pub async fn access_token(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginDto>,
) -> impl IntoResponse {
    if let Err(e) = body.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    let q = r#"SELECT * FROM "User" WHERE email = $1"#;
    let res = sqlx::query_as::<_, User>(q)
        .bind(body.email.to_string()).fetch_one(&state.db).await;
    match res {
        Ok(user) => {
            match crypto::verify(body.password.to_string(), user.hash_password).await {
                Ok(true) => Ok((
                    Json(
                        json!({ "status": "success", "token": jwt::create_token(user.id).unwrap() }),
                    ),
                )),
                Ok(false) => Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string())),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
        Err(_) => Err((
            StatusCode::UNAUTHORIZED, "Invalid password".to_string()
        )),
    }
}

pub async fn refresh_token() {}
