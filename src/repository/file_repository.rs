use std::io::{ErrorKind, Result as IoResult, Error as IoError};
use rocket::futures::TryStreamExt;
use sqlx::{Connection, Executor, MySqlConnection, Row};
use super::repository_base::RepositoryBase;
use crate::models::FileModel;




pub struct FileRepository {
    db_uri: String,
}

impl RepositoryBase<FileModel> for FileRepository {
    async fn get(&self, id: String) -> IoResult<FileModel> {
        let mut conn =
            MySqlConnection::connect(&self.db_uri).await.unwrap();

        // let query = format!("SELECT * FROM Files WHERE Id == {}", id);
        //
        // let res = conn.execute(query.as_str()).await.map_err(|err| {
        //     IoError::new(ErrorKind::NotFound, err)
        // })?;

        let mut rows = sqlx::query("SELECT * FROM Files WHERE Id == ?")
            .bind(id)
            .fetch(&mut conn);
        
        let mut files: Vec<FileModel> = Vec::new();
        while let Some(row) = rows.try_next().await.unwrap() {
            let file = FileModel::new(
                row.try_get("file_name").unwrap(),
                row.try_get("file_type").unwrap(),
                row.try_get("contents").unwrap(),
            );
            files.push(file);
        }
        
        if files.len() == 1 {
            Ok(files.first().unwrap().clone())
        } else {
            Err(std::io::Error::new(
                ErrorKind::Other, 
                "Found more than one file with given id"
            ))
        }
    }

    async fn create(&self, model: FileModel) -> IoResult<String> {
        let mut conn = MySqlConnection::connect(&self.db_uri).await.map_err(|err| {
            IoError::new(ErrorKind::ConnectionRefused, err)
        })?;

        let query = "INSERT INTO Files (FileName, FileType, Contents) VALUES (?, ?, ?)";
        let query = sqlx::query(query)
            .bind(model.file_name)
            .bind(model.file_type)
            .bind(model.contents.as_slice());

        let res = conn.execute(query).await.map_err(|err| {
            IoError::new(ErrorKind::InvalidData, err)
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