use crate::entity::user::UserRole;
use crate::error::{api_error::ApiError, token_error::TokenError, user_error::UserError};
use crate::repository::user_repository::UserRepositoryTrait;
use crate::service::token_service::TokenServiceTrait;
use crate::state::token_state::TokenState;

use axum::extract::{Request, State};
use axum::{http, middleware::Next, response::IntoResponse};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::{Authorization, Header};
use jsonwebtoken::errors::ErrorKind;

pub struct Auth;

impl Auth {
    pub async fn user_auth(
        State(state): State<TokenState>,
        mut req: Request,
        next: Next,
    ) -> Result<impl IntoResponse, ApiError> {
        let mut headers = req
            .headers_mut()
            .iter()
            .filter_map(|(header_name, header_value)| {
                if header_name == http::header::AUTHORIZATION {
                    return Some(header_value);
                }
                None
            });
        let header: Authorization<Bearer> =
            Authorization::decode(&mut headers).map_err(|_| TokenError::MissingToken)?;
        let token = header.token();

        match state.token_service.retrieve_token_claims(token) {
            Ok(token_data) => {
                let user = state
                    .user_repo
                    .select_by_email(token_data.claims.email)
                    .await;
                match user {
                    Some(user) => {
                        match (user.is_deleted, user.is_blocked) {
                            (true, _) => return Err(UserError::UserNotFound)?,
                            (_, true) => return Err(UserError::UserNotFound)?,
                            _ => {}
                        }

                        req.extensions_mut().insert(user);
                        Ok(next.run(req).await)
                    }
                    None => Err(UserError::UserNotFound)?,
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::ExpiredSignature => Err(TokenError::TokenExpired)?,
                _ => Err(TokenError::InvalidToken(token.parse().unwrap_or_default()))?,
            },
        }
    }

    pub async fn superuser_auth(
        State(state): State<TokenState>,
        mut req: Request,
        next: Next,
    ) -> Result<impl IntoResponse, ApiError> {
        let mut headers = req
            .headers_mut()
            .iter()
            .filter_map(|(header_name, header_value)| {
                if header_name == http::header::AUTHORIZATION {
                    return Some(header_value);
                }
                None
            });
        let header: Authorization<Bearer> =
            Authorization::decode(&mut headers).map_err(|_| TokenError::MissingToken)?;
        let token = header.token();

        match state.token_service.retrieve_token_claims(token) {
            Ok(token_data) => {
                let super_user = state
                    .user_repo
                    .select_by_email(token_data.claims.email)
                    .await;
                match super_user {
                    Some(super_user) => {
                        match (super_user.is_deleted, super_user.is_blocked, super_user.role_type) {
                            (true, _, _) => return Err(UserError::UserNotFound)?,
                            (_, true, _) => return Err(UserError::UserNotFound)?,
                            (_, _, UserRole::Superuser) => {
                                // ok
                            }
                            _ => return Err(UserError::NotSuperUser)?,
                        }
                        req.extensions_mut().insert(super_user);
                        Ok(next.run(req).await)
                    }
                    None => Err(UserError::UserNotFound)?,
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::ExpiredSignature => Err(TokenError::TokenExpired)?,
                _ => Err(TokenError::InvalidToken(token.parse().unwrap_or_default()))?,
            },
        }
    }
}
