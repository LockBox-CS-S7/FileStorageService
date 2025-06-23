use std::thread;
use lapin::{Connection, ConnectionProperties, options::*, types::FieldTable};
use rocket::futures::StreamExt;
use log::{info, warn};
use serde::{Deserialize, Serialize};

const ACCOUNT_QUEUE: &str = "account-queue";

pub struct RabbitMqListener {
    address: String,
}

impl RabbitMqListener {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    async fn start_listening(&self) {
        let address = self.address.clone();
        let _handler = thread::spawn(async move || {
            let conn = Connection::connect(
                &address,
                ConnectionProperties::default()
            ).await.expect("Failed to create a connection to RabbitMQ broker");

            let channel = conn.create_channel().await
                .expect("Failed to create a RabbitMQ channel");

            channel.queue_declare(
                ACCOUNT_QUEUE,
                QueueDeclareOptions::default(),
                FieldTable::default()
            ).await.expect("Failed to create queue");

            let mut consumer = channel.basic_consume(
                ACCOUNT_QUEUE, 
                "file-service",
                BasicConsumeOptions::default(),
                FieldTable::default()
            ).await.expect("Failed to create RabbitMQ consumer");
            
            while let Some(delivery) = consumer.next().await {
                if let Ok(delivery) = delivery {
                    info!("Received a message in queue: \"{}\"", ACCOUNT_QUEUE);
                    
                    let message = String::from_utf8(delivery.data).expect("Failed to decode message");
                    let message: AccountMessage = serde_json::from_str(message.as_str()).expect("Failed to parse message");
                    Self::handle_message(message);
                }
            }
        });
    }
    
    fn handle_message(message: AccountMessage) {
        match message.event_type.as_str() {
            "ACCOUNT_DELETION_REQUESTED" => {
                info!("Received account deletion request, start deleting user files...");
                // TODO: Delete user files
            },
            _ => {
                warn!("Failed to recognize the event_type of the incoming account message");
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccountMessage {
    pub event_type: String,
    pub timestamp: String,
    pub source: String,
    pub user_id: String,
    pub body: String,
}