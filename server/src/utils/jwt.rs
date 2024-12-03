use crate::domain::user::model::User;
use crate::route::AppState;
use crate::{env::JWT_SECRET, error};
use axum::extract::State;
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use std::sync::Arc;
use tokio::task_local;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    pub fn new(id: Uuid) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn create_token(id: Uuid) -> Result<String, error::Error> {
    let claims = Claims::new(id);
    let header = Header::default();
    let key = EncodingKey::from_secret(JWT_SECRET.as_ref());
    let token = jsonwebtoken::encode(&header, &claims, &key);
    Ok(token.unwrap())
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

task_local! {
    pub static USER: User;
}

pub async fn auth(
    State(app_state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if let Ok(current_user) = validate_token(app_state, auth_token).await {
        Ok(USER.scope(current_user, next.run(req)).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn validate_token(
    app_state: Arc<AppState>,
    auth_token: &str,
) -> Result<User, (StatusCode, Json<ErrorResponse>)> {
    let claims = decode::<TokenClaims>(
        auth_token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;
    let user = match sqlx::query("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .map(|row: PgRow| User::from(row))
        .fetch_one(&app_state.db)
        .await
    {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    };
    let _user = user.map_err(|err| {
        let json_error = ErrorResponse {
            status: "fail",
            message: err,
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;
    Ok(_user)
}
