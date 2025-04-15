use std::fmt;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicPublishArguments, Channel, ExchangeDeclareArguments, ExchangeType, QueueBindArguments,
        QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use rand::Rng;

use tracing::{error, info};

use super::env::EnvironmentVariables;
use serde::{Deserialize, Serialize};

pub const EXCHANGE_UPLOAD_OBJECT: &str = "flaxum.upload.object";

pub const QUEUE_EVENT_UPLOAD_USER: &str = "flaxum.upload.object.user";
pub const QUEUE_EVENT_UPLOAD_ROBOT: &str = "flaxum.upload.object.robot";

pub const ROUTING_KEY_EVENT_UPLOAD_USER: &str = "event.upload.user";
pub const ROUTING_KEY_EVENT_UPLOAD_ROBOT: &str = "event.upload.robot";

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
}

impl NotifierAmqp {
    pub fn new(env: &EnvironmentVariables) -> Self {
        NotifierAmqp {
            amqp_host: env.rmq_host.to_string(),
            amqp_port: env.rmq_port,
            amqp_username: env.rmq_user.to_string(),
            amqp_password: env.rmq_pass.to_string(),
        }
    }

    pub async fn init(&mut self) -> Connection {
        // declare queues...
        let connection = self.connection().await;
        connection
            .register_callback(DefaultConnectionCallback)
            .await
            .unwrap();

        let channel = connection.open_channel(None).await.unwrap();
        channel
            .register_callback(DefaultChannelCallback)
            .await
            .unwrap();

        info!(
            "Starting declaring queue ({}, {}) to exchange {} and bind them.",
            QUEUE_EVENT_UPLOAD_USER, QUEUE_EVENT_UPLOAD_ROBOT, EXCHANGE_UPLOAD_OBJECT
        );

        channel
            .exchange_declare(ExchangeDeclareArguments::new(
                EXCHANGE_UPLOAD_OBJECT,
                ExchangeType::Direct.to_string().as_str(),
            ))
            .await
            .unwrap();

        let (queue_name_user, _, _) = channel
            .queue_declare(QueueDeclareArguments::durable_client_named(
                QUEUE_EVENT_UPLOAD_USER,
            ))
            .await
            .unwrap()
            .unwrap();
        let (queue_name_robot, _, _) = channel
            .queue_declare(QueueDeclareArguments::durable_client_named(
                QUEUE_EVENT_UPLOAD_ROBOT,
            ))
            .await
            .unwrap()
            .unwrap();

        channel
            .queue_bind(QueueBindArguments::new(
                &queue_name_user,
                EXCHANGE_UPLOAD_OBJECT,
                ROUTING_KEY_EVENT_UPLOAD_USER,
            ))
            .await
            .unwrap();
        channel
            .queue_bind(QueueBindArguments::new(
                &queue_name_robot,
                EXCHANGE_UPLOAD_OBJECT,
                ROUTING_KEY_EVENT_UPLOAD_ROBOT,
            ))
            .await
            .unwrap();

        info!("Finished declaring amqp resources.");
        connection
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

pub async fn send_upload_user_event(event: UploadUserEvent, conn: &Connection) {
    let channel = conn.open_channel(None).await.unwrap();
    match serde_json::to_string(&event) {
        Ok(content) => {
            let args =
                BasicPublishArguments::new(EXCHANGE_UPLOAD_OBJECT, ROUTING_KEY_EVENT_UPLOAD_USER);
            if let Err(err) = channel
                .basic_publish(
                    BasicProperties::default(),
                    content.as_bytes().to_vec(),
                    args,
                )
                .await
            {
                error!("Error when publish an evento into queue: {}", err);
            }
        }
        Err(err) => {
            error!("Error when transform event into json: {}", err);
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UploadUserEvent {
    pub user_id: String,
    pub object_id: String,
    pub key: String,
}

impl UploadUserEvent {
    pub fn generate_key(key_length_bytes: usize) -> String {
        let mut rng = rand::thread_rng();
        let key: Vec<u8> = (0..key_length_bytes).map(|_| rng.gen()).collect();
        let ans = hex::encode(key);
        println!("debug {}", ans);
        "eeef72847ca361dfb4dc22727910538a6339932998f56b124152690ef5516479".to_string()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadRobotEvent {
    pub user_id: String,
    pub object_id: String,
    pub key: String,
}
