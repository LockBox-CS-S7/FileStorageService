use super::repository_base::RepositoryBase;
use crate::models::FileModel;
use crate::FileId;
use sqlx::{Connection, Executor, MySqlConnection};
use std::io::{Error as IoError, ErrorKind, Result as IoResult};

pub struct FileRepository {
    db_uri: String,
}

impl RepositoryBase<FileModel> for FileRepository {
    async fn get(&self, id: &str) -> IoResult<FileModel> {
        let mut conn = MySqlConnection::connect(&self.db_uri)
            .await
            .map_err(|err| IoError::new(ErrorKind::ConnectionRefused, err))?;

        let query = "SELECT * FROM Files WHERE id = ?";
        let found_file = sqlx::query_as::<_, FileModel>(query)
            .bind(id)
            .fetch_optional(&mut conn)
            .await
            .map_err(|err| IoError::new(ErrorKind::ConnectionRefused, err))?;

        if let Some(file) = found_file {
            Ok(file)
        } else {
            Err(std::io::Error::new(
                ErrorKind::NotFound,
                "Could not find file with the given id",
            ))
        }
    }

    /// Inserts a new entry in the Files table, ignores _FileModel.id_
    async fn create(&self, model: FileModel) -> IoResult<String> {
        let mut conn = MySqlConnection::connect(&self.db_uri)
            .await
            .map_err(|err| IoError::new(ErrorKind::ConnectionRefused, err))?;

        let id = FileId::new(36);
        let query = "INSERT INTO Files (id, user_id, file_name, file_type, contents) VALUES (?, ?, ?, ?, ?)";
        let query = sqlx::query(query)
            .bind(id.as_str())
            .bind(model.user_id)
            .bind(model.file_name)
            .bind(model.file_type)
            .bind(model.contents.as_slice());

        let res = conn
            .execute(query)
            .await
            .map_err(|err| IoError::new(ErrorKind::InvalidData, err))?;

        let id = format!("{}", res.last_insert_id());
        Ok(String::from(id.as_str()))
    }

    async fn update(&self, model: FileModel) -> IoResult<()> {
        todo!()
    }

    async fn delete(&self, id: &str) -> IoResult<()> {
        todo!()
    }
}

impl FileRepository {
    pub fn new(db_uri: &str) -> Self {
        Self {
            db_uri: db_uri.to_string(),
        }
    }

    pub async fn get_file_by_user_id(&self, user_id: &str) -> IoResult<Vec<FileModel>> {
        let mut conn = MySqlConnection::connect(&self.db_uri)
            .await
            .map_err(|err| IoError::new(ErrorKind::ConnectionRefused, err))?;

        let query = "SELECT * FROM Files WHERE user_id = ?";
        let found_files = sqlx::query_as::<_, FileModel>(query)
            .bind(user_id)
            .fetch_all(&mut conn)
            .await
            .map_err(|err| IoError::new(ErrorKind::ConnectionRefused, err))?;

        if found_files.len() > 0 {
            Ok(found_files)
        } else {
            Err(std::io::Error::new(
                ErrorKind::NotFound,
                "Could not find any file with the given user id",
            ))
        }
    }
}
