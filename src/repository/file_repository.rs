use std::io::Result;
use super::repository_base::RepositoryBase;

pub struct FileRepository {
    db_uri: String,
}

impl RepositoryBase for FileRepository {
    fn get(&self, id: i32) -> IoResult<T> {
        todo!()
    }

    fn create(&self, model: T) -> IoResult<i32> {
        todo!()
    }

    fn update(&self, model: T) -> IoResult<()> {
        todo!()
    }

    fn delete(&self, id: i32) -> IoResult<()> {
        todo!()
    }
}