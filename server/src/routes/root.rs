use std::sync::Arc;

use axum::middleware;
use axum::routing::IntoMakeService;
use axum::Router;

use tower::ServiceBuilder;

use crate::config::AppConfig;
use crate::middleware::auth as auth_middleware;
use crate::state::auth_state::AuthState;
use crate::state::object_state::ObjectState;
use crate::state::token_state::TokenState;
use crate::state::user_state::UserState;

use super::object;
use super::user;
use super::uxo;

use super::auth;

pub async fn app(config: Arc<AppConfig>) -> IntoMakeService<Router> {
    let db_conn = Arc::new(config.db_conn.clone());
    let s3_client = Arc::new(config.s3_client.clone());

    let auth_state = AuthState::new(&db_conn);
    let user_state = UserState::new(&db_conn);
    let object_state = ObjectState::new(&db_conn, &s3_client);
    let token_state = TokenState::new(&db_conn);

    let public_routes = auth::routes().with_state(auth_state);

    let protected_routes = Router::new()
        .merge(object::routes().with_state(object_state.clone()))
        .merge(uxo::routes().with_state(object_state))
        .merge(user::routes().with_state(user_state))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            token_state,
            auth_middleware::auth,
        )));

    let app = Router::new().merge(public_routes).merge(protected_routes);

    app.into_make_service()
}
