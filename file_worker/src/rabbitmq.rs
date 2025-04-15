use aes::cipher::KeyIvInit;
use amqprs::{
    BasicProperties, Deliver,
    channel::{BasicAckArguments, BasicConsumeArguments, Channel},
    connection::{Connection, OpenConnectionArguments},
    consumer::AsyncConsumer,
};
use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use std::{
    fmt,
    fs::{self},
    sync::Arc,
    time::Instant,
};

use tracing::{error, info};

use crate::Config;

use super::env::EnvironmentVariables;
use serde::{Deserialize, Serialize};

use aes::{Aes256, cipher::KeyInit};
use ctr::Ctr128BE;
use ctr::cipher::StreamCipher;
use sha2::{Sha256, digest::Digest};

type Aes256Ctr = Ctr128BE<Aes256>;

pub const QUEUE_EVENT_UPLOAD_USER: &str = "flaxum.upload.object.user";

pub const ROUTING_KEY_EVENT_UPLOAD_USER: &str = "event.upload.user";

static TMP_DIR: &str = "tmp";

#[cfg(target_os = "linux")]
static PATH_SEPO: &str = "/";

#[cfg(target_os = "windows")]
static PATH_SEPO: &str = "\\";

pub struct ArcturusAmqpConnChannel(pub Connection, pub Channel);

impl fmt::Debug for ArcturusAmqpConnChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArcturusAmqpConnChannel")
            // .field("connection", &self.connection)
            .field("channel", &self.1.channel_id())
            .finish()
    }
}

#[derive(Debug)]
pub struct NotifierAmqp {
    pub amqp_host: String,
    pub amqp_port: u16,
    pub amqp_username: String,
    pub amqp_password: String,
    pub conn_channel_list: Vec<ArcturusAmqpConnChannel>,
}

struct FileUploaderConsumer {
    config: Arc<Config>,
}

impl FileUploaderConsumer {
    pub fn new(config: Arc<Config>) -> Self {
        FileUploaderConsumer { config }
    }

    pub async fn config(config: Arc<Config>, amqp: &NotifierAmqp) -> ArcturusAmqpConnChannel {
        let connection = amqp.connection().await;
        let channel = connection.open_channel(None).await.unwrap();

        let args =
            BasicConsumeArguments::new(QUEUE_EVENT_UPLOAD_USER, ROUTING_KEY_EVENT_UPLOAD_USER);
        channel
            .basic_consume(FileUploaderConsumer::new(config), args)
            .await
            .unwrap();

        ArcturusAmqpConnChannel(connection, channel)
    }

    pub async fn send(&self, event: UploadUserEvent) -> Result<(), Box<dyn std::error::Error>> {
        // todo: error handling
        let path_to_file = format!("tmp{}{}.{}", PATH_SEPO, event.user_id, event.object_id);
        let data = fs::read(&path_to_file)?;

        let mut key = [0u8; 32];
        hex::decode_to_slice(&event.key, &mut key)?;
        let mut hasher = Sha256::new();
        Digest::update(&mut hasher, key);
        let result = hasher.finalize();

        let nonce: [u8; 16] = result[..16].try_into().unwrap();

        let mut cipher = Aes256Ctr::new(&key.into(), &nonce.into());

        let now = Instant::now();
        let mut enc_data = data;
        cipher.apply_keystream(&mut enc_data);
        tracing::debug!("Encrypt elapsed: {:.2?}", now.elapsed());
        fs::write(&path_to_file, &enc_data)?;

        let body = ByteStream::from_path(std::path::Path::new(&path_to_file)).await?;
        let s3_path = format!("{}{}{}", event.user_id, PATH_SEPO, event.object_id);

        let result = self
            .config
            .s3_client
            .put_object()
            .bucket(self.config.env.upload_main_bucket.clone())
            .key(s3_path)
            .body(body)
            .send()
            .await;

        match result {
            Ok(response) => println!("S3 upload success: {:?}", response),
            Err(e) => println!("S3 upload error: {:?}", e),
        }

        // Удаление временного файла
        fs::remove_file(&path_to_file)?;
        Ok(())
    }
}

#[async_trait]
impl AsyncConsumer for FileUploaderConsumer {
    async fn consume(
        &mut self, // use `&mut self` to make trait object to be `Sync`
        channel: &Channel,
        deliver: Deliver,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        info!(
            "consume delivery {} on channel {}, content size: {}",
            deliver,
            channel,
            content.len()
        );

        if let Ok(event) = serde_json::from_str::<UploadUserEvent>(
            String::from_utf8_lossy(&content).to_string().as_str(),
        ) {
            let _ = self.send(event).await;
            info!("ack to delivery {} on channel {}", deliver, channel);
            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            channel.basic_ack(args).await.unwrap();
        } else {
            error!("Error when deserialize a UploadEvent.");
        }
    }
}

impl NotifierAmqp {
    pub fn new(env: &EnvironmentVariables) -> Self {
        NotifierAmqp {
            amqp_host: env.rmq_host.to_string(),
            amqp_port: env.rmq_port,
            amqp_username: env.rmq_user.to_string(),
            amqp_password: env.rmq_pass.to_string(),
            conn_channel_list: Vec::new(),
        }
    }

    pub async fn init(&mut self, config: Arc<Config>) {
        info!("Starting upload worker-consumer.");
        let mail_conn_channel = FileUploaderConsumer::config(config, self).await;
        self.conn_channel_list.push(mail_conn_channel);
        info!("Finished upload worker-consumer.");
    }

    pub async fn connection(&self) -> Connection {
        Connection::open(&OpenConnectionArguments::new(
            self.amqp_host.to_owned().as_str(),
            self.amqp_port,
            self.amqp_username.to_owned().as_str(),
            self.amqp_password.to_owned().as_str(),
        ))
        .await
        .unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadUserEvent {
    pub user_id: String,
    pub object_id: String,
    pub key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadRobotEvent {
    pub user_id: String,
    pub object_id: String,
    pub key: String,
}
