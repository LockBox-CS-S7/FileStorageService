use std::env;
use lapin::{BasicProperties, Connection, ConnectionProperties};
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use serde::{Serialize, Deserialize};
use chrono::Utc;

pub struct RabbitMqMessenger {
    address: String,
}

impl RabbitMqMessenger {
    pub fn from_env() -> Self {
        let address = env::var("RABBIT_ADDRESS")
            .expect("Failed to get the RabbitMQ address from environment");
        
        Self { address }
    }
    
    pub async fn send_message(&self, message: &FileMessageData) -> Result<(), lapin::Error> {
        let conn = Connection::connect(&self.address, ConnectionProperties::default())
            .await?;

        let channel = conn.create_channel().await?;
        channel.queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        ).await?;

        let message_str = serde_json::to_string(message)
            .expect("Failed to convert the message to JSON");
        channel.basic_publish(
            "",
            "hello",
            BasicPublishOptions::default(),
            message_str.as_bytes(),
            BasicProperties::default(),
        ).await?;
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMessageData {
    pub event_type: String,
    pub timestamp: String,
    source: String,
    pub user_id: String,
    pub data: Option<Vec<u8>>,
}

impl FileMessageData {
    pub fn new(event_type: &str, user_id: &str, data: Option<Vec<u8>>) -> Self {
        Self {
            event_type: event_type.to_string(),
            timestamp: Utc::now().to_string(),
            source: "FileStorageService".to_string(),
            user_id: user_id.to_string(),
            data,
        }
    }
}