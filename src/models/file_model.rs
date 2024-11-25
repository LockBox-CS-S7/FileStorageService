use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct FileModel {
    pub id: Option<String>,
    pub user_id: String,
    pub file_name: String,
    pub file_type: String,
    pub contents: Vec<u8>,
}
