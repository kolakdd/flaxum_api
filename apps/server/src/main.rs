#![allow(dead_code)]
#![allow(unused)]

use axum::{
    extract::Multipart,
    routing::{get, post},
    Router,
};
use sqlx::{query_file, query_file_as, FromRow, PgPool};


async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("Length of `{}` is {} bytes", name, data.len());
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&dotenvy::var("DB_URL")?).await?;
    sqlx::migrate!("./db/migrations").run(&pool).await?;

    let app = Router::new()
        .route("/hello", get(|| async { "hello" }))
        .route("/upload", post(upload));

    println!("Running on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
