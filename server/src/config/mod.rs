pub mod database;
pub mod env;
pub mod parameter;
pub mod s3;

use crate::config::env::EnvironmentVariables;
use anyhow;
use aws_sdk_s3::Client;
use database::{Database, DatabaseTrait};
use s3::S3Client;
use sqlx::migrate::MigrateDatabase;

pub const SIZE_1GB: usize = 1024 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub env: EnvironmentVariables,
    pub db_conn: Database,
    pub s3_client: Client,
}

impl AppConfig {
    pub async fn load() -> anyhow::Result<Self> {
        let env = EnvironmentVariables::from_env()?;

        println!("database_url = {}",&env.database_url);
        if !sqlx::Postgres::database_exists(&env.database_url).await? {
            println!("lolker");
            sqlx::Postgres::create_database(&env.database_url).await?;
        }

        let db_conn = Database::init(&env)
            .await
            .unwrap_or_else(|e| panic!("Database error {}", e));
        let s3_client = S3Client::init(&env).await;
        Ok(Self {
            env,
            db_conn,
            s3_client,
        })
    }
}
