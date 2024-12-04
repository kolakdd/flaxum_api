use crate::config;
use crate::utils::jwt;
use crate::{
    config::Config,
    db,
    domain::{auth, object, user},
    Error,
};
use aws_sdk_s3 as s3;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

#[derive(Debug)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub s3: s3::client::Client,
}

pub async fn app() -> Result<Router, Error> {
    let config = Arc::new(Config::load().await?);
    // init migration
    let db = db::connect(&config.database).await?;
    db::migrate(&db.clone()).await?;

    let app_state = Arc::new(AppState {
        db: db.clone(),
        s3: config.s3_client.clone(),
    });

    // app
    let app = Router::new()
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
                .with_state(app_state.clone())
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
        //
        .route(
            "/folder",
            post(object::handler::create_folder)
                .layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
                .with_state(app_state.clone()),
        )
        .route(
            "/upload",
            post(object::handler::upload_file)
                .layer(RequestBodyLimitLayer::new(config::SIZE_1GB))
                .layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
                .with_state(app_state.clone()),
        )
        .route(
            "/download",
            get(object::handler::download_file)
                .layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
                .with_state(app_state.clone()),
        )
        // objects
        .route(
            "/object",
            get(object::handler::get_info)
                .with_state(app_state.clone())
                .put(object::handler::update_info)
                .with_state(app_state.clone())
                .delete(object::handler::delete_object)
                .with_state(app_state.clone()),
        )
        .route(
            "/object/list",
            get(object::handler::get_own_list)
                .layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
                .with_state(app_state.clone()),
        )
        .route(
            "/object/trash/list",
            get(object::handler::get_trash_list)
                .layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
                .with_state(app_state.clone()),
        )
        .layer(TraceLayer::new_for_http());
    Ok(app)
}
