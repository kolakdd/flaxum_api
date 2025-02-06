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

pub async fn app() -> anyhow::Result<Router> {
    let config = Arc::new(AppConfig::load().await?);
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

// define some routes separately

// let user_routes = Router::new()
//     .route("/users", get(users_list))
//     .route("/users/{id}", get(users_show));

// let team_routes = Router::new()
//     .route("/teams", get(teams_list));

// // combine them into one
// let app = Router::new()
//     .merge(user_routes)
//     .merge(team_routes);

// could also do `user_routes.merge(team_routes)`

// Our app now accepts
// - GET /users
// - GET /users/{id}
// - GET /teams
