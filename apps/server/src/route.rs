use std::sync::Arc;

use crate::utils::jwt;
use crate::{
    config::Config,
    db,
    domain::{auth, object, user},
    Error,
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use minio::s3;
use minio::s3::args::{BucketExistsArgs, MakeBucketArgs};
use sqlx::{Pool, Postgres};
use tower_http::trace::TraceLayer;

#[derive(Debug)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub s3: s3::client::Client,
    pub upload_bucket: String,
}

pub async fn app() -> Result<Router, Error> {
    let config = Arc::new(Config::load()?);

    //todo: вынести init'ы

    // init migration
    let db = db::connect(&config.database).await?;
    db::migrate(&db.clone()).await?;

    // init buckets
    let s3 = s3::client::Client::new(
        config.s3.url.clone(),
        Some(Box::new(config.s3.static_provider.clone())),
        None,
        None,
    )
    .unwrap();

    let exists = s3
        .bucket_exists(&BucketExistsArgs::new(&config.s3.upload_bucket).unwrap())
        .await
        .unwrap();

    if !exists {
        s3.make_bucket(&MakeBucketArgs::new(&config.s3.upload_bucket).unwrap())
            .await
            .unwrap();
    }

    let app_state = Arc::new(AppState {
        db: db.clone(),
        s3: s3.clone(),
        upload_bucket: config.s3.upload_bucket.clone(),
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
                // .layer(RequestBodyLimitLayer::new(config::SIZE_1GB))
                // .layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
                .with_state(app_state.clone()),
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
                .delete(object::handler::delete_object)
                .with_state(app_state.clone()),
        )
        .route(
            "/objects",
            get(object::handler::get_own_list)
                .layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
                .with_state(app_state.clone()),
        )
        .layer(TraceLayer::new_for_http());
    Ok(app)
}
