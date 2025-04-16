use aes::cipher::KeyIvInit;
use amqprs::{
    BasicProperties, Deliver,
    channel::{BasicAckArguments, BasicConsumeArguments, Channel},
    connection::{Connection, OpenConnectionArguments},
    consumer::AsyncConsumer,
};
use async_trait::async_trait;
use aws_sdk_s3::{
    primitives::ByteStream,
    types::{CompletedMultipartUpload, CompletedPart},
};
use std::{
    fmt,
    fs::{self},
    sync::Arc,
};

use tracing::{error, info};

use crate::Config;

use super::env::EnvironmentVariables;
use serde::{Deserialize, Serialize};

use aes::Aes256;
use ctr::Ctr128BE;
use ctr::cipher::StreamCipher;
use sha2::{Sha256, digest::Digest};

type Aes256Ctr = Ctr128BE<Aes256>;

const CHUNK_SIZE: u64 = 1024 * 1024 * 8;

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
        let upload_bucket = self.config.env.upload_main_bucket.clone();
        let s3_path = format!("{}/{}", event.user_id, event.object_id);
        let path_to_file = format!("{}{}{}.{}", TMP_DIR, PATH_SEPO, event.user_id, event.object_id);
    
        // Читаем и шифруем файл
        let data = fs::read(&path_to_file)?;
        println!("RMQ: event = {:?}", event);
    
        let mut key = [0u8; 32];
        let nonce: [u8; 16] = {
            hex::decode_to_slice(&event.key, &mut key)?;
            let mut hasher = Sha256::new();
            Digest::update(&mut hasher, key);
            let result = hasher.finalize();
            result[..16].try_into().unwrap()
        };
    
        let mut cipher = Aes256Ctr::new(&key.into(), &nonce.into());
        let mut enc_data = data.clone();
        cipher.apply_keystream(&mut enc_data);
    
        let enc_path = format!("{}.enc", path_to_file);
        fs::write(&enc_path, &enc_data)?;
    
        let size = enc_data.len() as u64;
        let chunk_count = (size + CHUNK_SIZE - 1) / CHUNK_SIZE; 
    
        let multipart_upload_res = self
            .config
            .s3_client
            .create_multipart_upload()
            .bucket(upload_bucket.clone())
            .key(&s3_path)
            .send()
            .await?;
    
        let upload_id = multipart_upload_res.upload_id().unwrap();
        let mut upload_parts = Vec::new();
    
        for chunk_index in 0..chunk_count {
            let start = chunk_index * CHUNK_SIZE;
            let end = std::cmp::min(start + CHUNK_SIZE, size);
            let chunk_size = end - start;
    
            let stream = ByteStream::read_from()
                .path(&enc_path)
                .offset(start)
                .length(aws_sdk_s3::primitives::Length::Exact(chunk_size))
                .build()
                .await?;
    
            let upload_part_res = self
                .config
                .s3_client
                .upload_part()
                .bucket(upload_bucket.clone())
                .key(&s3_path)
                .upload_id(upload_id)
                .part_number((chunk_index + 1) as i32)
                .body(stream)
                .send()
                .await?;
    
            upload_parts.push(
                CompletedPart::builder()
                    .e_tag(upload_part_res.e_tag.unwrap_or_default())
                    .part_number((chunk_index + 1) as i32)
                    .build(),
            );
        }
    
        // Завершаем загрузку
        let completed_upload = CompletedMultipartUpload::builder()
            .set_parts(Some(upload_parts))
            .build();
    
        self.config
            .s3_client
            .complete_multipart_upload()
            .bucket(upload_bucket)
            .key(s3_path)
            .upload_id(upload_id)
            .multipart_upload(completed_upload)
            .send()
            .await?;
    
        // Удаляем временные файлы
        fs::remove_file(&path_to_file)?;
        fs::remove_file(&enc_path)?;
    
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
