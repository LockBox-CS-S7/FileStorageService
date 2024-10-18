use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct FileModel {
    pub id: Option<String>,
    pub file_name: String,
    pub file_type: String,
    pub contents: Vec<u8>,
}
