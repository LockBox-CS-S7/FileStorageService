use std::io::Result as IoResult;
use sqlx::{Connection, MySqlConnection};
use super::repository_base::RepositoryBase;
use crate::models::FileModel;

pub struct FileRepository {
    db_uri: String,
}

impl RepositoryBase<FileModel> for FileRepository {
    async fn get(&self, id: i32) -> IoResult<FileModel> {
        let conn = MySqlConnection::connect("mysql://root:password@file-db/database").await.unwrap();

        todo!()
    }

    async fn create(&self, model: FileModel) -> IoResult<i32> {
        todo!()
    }

    async fn update(&self, model: FileModel) -> IoResult<()> {
        todo!()
    }

    async fn delete(&self, id: i32) -> IoResult<()> {
        todo!()
    }
}