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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileViewModel {
    pub id: Option<String>,
    pub user_id: String,
    pub file_name: String,
    pub file_type: String,
    pub contents: Option<Vec<u8>>,
}

impl FileViewModel {
    pub fn from_model(model: FileModel) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            file_name: model.file_name,
            file_type: model.file_type,
            contents: Some(model.contents),
        }
    }
    
    pub fn remove_contents(mut self) -> Self {
        self.contents = None;
        self
    }
}
