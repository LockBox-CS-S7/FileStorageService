use std::io::{ErrorKind, Result as IoResult, Error as IoError};
use sqlx::{Connection, Executor, MySqlConnection};
use super::repository_base::RepositoryBase;
use crate::models::FileModel;




pub struct FileRepository {
    db_uri: String,
}

impl RepositoryBase<FileModel> for FileRepository {
    async fn get(&self, id: String) -> IoResult<FileModel> {
        let mut conn =
            MySqlConnection::connect(&self.db_uri).await.unwrap();

        let query = format!("SELECT * FROM Files WHERE Id == {}", id);

        let res = conn.execute(query.as_str()).await.map_err(|_| {
            IoError::new(ErrorKind::NotFound, "Could not find item with requested id")
        })?;

        todo!()
    }

    async fn create(&self, model: FileModel) -> IoResult<String> {
        let mut conn = MySqlConnection::connect(&self.db_uri).await.map_err(|_| {
            IoError::new(ErrorKind::ConnectionRefused, "Database connection refused")
        })?;

        let contents_utf = String::from_utf8_lossy(model.contents.as_slice());
        let query = format!("INSERT INTO Files (FileName, FileType, Contents) VALUES \
        ('{}', '{}', '{}')", model.file_name, model.file_type, contents_utf);

        let res = conn.execute(query.as_str()).await.map_err(|_| {
            IoError::new(ErrorKind::InvalidData, "Failed to insert a new file from given model")
        })?;
        
        let id = format!("{}", res.last_insert_id());
        Ok(id)
    }

    async fn update(&self, model: FileModel) -> IoResult<()> {
        todo!()
    }

    async fn delete(&self, id: String) -> IoResult<()> {
        todo!()
    }
}

impl FileRepository {
    pub fn new(db_uri: &str) -> Self {
        Self {
            db_uri: db_uri.to_string()
        }
    }
}