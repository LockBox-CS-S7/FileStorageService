use std::env;
use lapin::{BasicProperties, Connection, ConnectionProperties};
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;

pub struct RabbitMqMessenger {
    address: String,
}

impl RabbitMqMessenger {
    pub fn from_env() -> Self {
        let address = env::var("RABBIT_ADDRESS")
            .expect("Failed to get the RabbitMQ address from environment");
        
        Self { address }
    }
    
    pub async fn send_message(&self, message: &str) -> Result<(), lapin::Error> {
        let conn = Connection::connect(&self.address, ConnectionProperties::default())
            .await?;

        let channel = conn.create_channel().await?;
        channel.queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        ).await?;

        channel.basic_publish(
            "",
            "hello",
            BasicPublishOptions::default(),
            message.as_bytes(),
            BasicProperties::default(),
        ).await?;

        Ok(())
    }
}