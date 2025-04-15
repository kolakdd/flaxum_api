use aws_sdk_s3::Client as S3Client;

use env::EnvironmentVariables;

use rabbitmq::NotifierAmqp;

use std::fs::{self};
use std::sync::Arc;
use tokio::sync::Notify;

use db::DatabaseTrait;

mod db;
mod env;
mod rabbitmq;
mod s3;

static TMP_DIR: &str = "tmp";

#[cfg(target_os = "linux")]
static PATH_SEPO: &str = "/";

#[cfg(target_os = "windows")]
static PATH_SEPO: &str = "\\";

fn parse_path(path: &str) -> String {
    let uuids: Vec<&str> = path.split(PATH_SEPO).last().unwrap().split(".").collect();
    let (user_uuid, file_uuid) = (uuids[0], uuids[1]);
    format!("{}/{}", user_uuid, file_uuid)
}

#[derive(Clone)]
struct Config {
    pub env: Arc<EnvironmentVariables>,
    pub db_conn: Arc<db::Database>,
    pub s3_client: Arc<S3Client>,
}

impl Config {
    async fn init() -> anyhow::Result<Self> {
        let env = env::EnvironmentVariables::from_env().unwrap();
        let db_conn = db::Database::init(&env)
            .await
            .unwrap_or_else(|e| panic!("Database error {}", e));
        let s3_client = s3::S3Client::init(&env).await;

        Ok(Config {
            env: Arc::new(env),
            db_conn: Arc::new(db_conn),
            s3_client: Arc::new(s3_client),
        })
    }
}

fn init_tmp_file() {
    if fs::metadata(TMP_DIR).is_err() {
        fs::create_dir(TMP_DIR).unwrap();
    }
}

pub async fn spawn_worker() {
    init_tmp_file();
    let config = Config::init().await.unwrap();
    let mut ampq = NotifierAmqp::new(&config.env);
    ampq.init(Arc::new(config)).await;
    println!("consume forever..., ctrl+c to exit");
    let guard = Notify::new();
    guard.notified().await;
}
