use crate::config::AppConfig;
use crate::state::AppState;
use axum::Router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use super::auth::route::AuthRouter;
use super::object::route::ObjectRouter;
use super::user::route::UserRouter;
use super::uxo::route::UserXObjectRouter;

pub async fn app(config: Arc<AppConfig>) -> anyhow::Result<Router> {
    let app_state = Arc::new(AppState::build(config.as_ref().clone()).await);

    let app = Router::new()
        .nest("/", ObjectRouter::init(app_state.clone()).await)
        .nest("/access", UserXObjectRouter::init(app_state.clone()).await)
        .nest("/user", UserRouter::init(app_state.clone()).await)
        .nest("/", AuthRouter::init(app_state.clone()).await)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());
    Ok(app)
}
