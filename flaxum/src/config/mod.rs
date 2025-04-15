pub mod database;
pub mod env;
pub mod parameter;
pub mod rabbitmq;
pub mod s3;

use std::sync::Arc;

use crate::config::env::EnvironmentVariables;
use anyhow;
use aws_sdk_s3::Client;
use database::{Database, DatabaseTrait};
use rabbitmq::NotifierAmqp;
use s3::S3Client;

pub const SIZE_1GB: usize = 1024 * 1024 * 1024;

#[derive(Clone)]
pub struct AppConfig {
    pub env: Arc<EnvironmentVariables>,
    pub db_conn: Arc<Database>,
    pub s3_client: Arc<Client>,
    pub rmq_conn: Arc<amqprs::connection::Connection>,
}

impl AppConfig {
    pub async fn load() -> anyhow::Result<Self> {
        let env = EnvironmentVariables::from_env()?;

        let db_conn = Database::init(&env)
            .await
            .unwrap_or_else(|e| panic!("Database error {}", e));

        let s3_client = S3Client::init(&env).await;

        let mut ampq = NotifierAmqp::new(&env);
        let rmq_conn = ampq.init().await;
        // let arc_amqp = Arc::new(amqp);

        Ok(Self {
            env: Arc::new(env),
            db_conn: Arc::new(db_conn),
            s3_client: Arc::new(s3_client),
            rmq_conn: Arc::new(rmq_conn),
        })
    }
}
