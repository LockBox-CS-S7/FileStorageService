use std::io::Result as IoResult;
use super::repository_base::RepositoryBase;
use crate::models::FileModel;

pub struct FileRepository {
    db_uri: String,
}

impl RepositoryBase<FileModel> for FileRepository {
    fn get(&self, id: i32) -> IoResult<FileModel> {
        todo!()
    }

    fn create(&self, model: FileModel) -> IoResult<i32> {
        todo!()
    }

    fn update(&self, model: FileModel) -> IoResult<()> {
        todo!()
    }

    fn delete(&self, id: i32) -> IoResult<()> {
        todo!()
    }
}