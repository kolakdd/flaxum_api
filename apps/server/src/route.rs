use std::sync::Arc;

use crate::{
    config::Config,
    db,
    domain::{auth, object, user},
    Error,
};
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sqlx::{Pool, Postgres};
use tower_http::trace::TraceLayer;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Hello!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn app() -> Result<Router, Error> {
    let config = Arc::new(Config::load()?);

    let db = db::connect(&config.database).await?;
    db::migrate(&db.clone()).await?;

    #[derive(OpenApi)]
    struct ApiDoc;

    let app_state = Arc::new(AppState { db: db.clone() });

    let mut app = Router::new()
        .route("/hello", get(health_checker_handler))
        // auth
        .route(
            "/user/list",
            get(user::handler::admin_get_users).with_state(app_state.clone()),
        )
        .route(
            "/user/me",
            get(user::handler::get_user)
                .with_state(app_state.clone())
                .put(user::handler::update_user)
                .delete(user::handler::delete_user)
                .with_state(app_state.clone()),
        )
        // auth
        .route(
            "/user/register",
            post(auth::handler::register_user).with_state(app_state.clone()),
        )
        .route(
            "/user/login",
            post(auth::handler::access_token).with_state(app_state.clone()),
        )
        .route(
            "/refresh_token",
            get(auth::handler::refresh_token).with_state(app_state.clone()),
        )
        // multipart
        .route(
            "/upload",
            post(object::handler::upload_file).with_state(app_state.clone()),
        )
        .route(
            "/download",
            post(object::handler::download_file).with_state(app_state.clone()),
        )
        // objects
        .route(
            "/object",
            get(object::handler::get_info)
                .with_state(app_state.clone())
                .put(object::handler::update_info)
                .with_state(app_state.clone())
                .delete(object::handler::delete_file)
                .with_state(app_state.clone()),
        )
        .route(
            "/objects",
            get(object::handler::get_list).with_state(app_state.clone()),
        )
        //
        .layer(TraceLayer::new_for_http());

    app = app.merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()));
    Ok(app)
}
