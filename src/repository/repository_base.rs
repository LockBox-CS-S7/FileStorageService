use std::io::Result as IoResult;

pub trait RepositoryBase<T> {
    async fn read(&self, id: &str) -> IoResult<T>;
    async fn create(&self, model: T) -> IoResult<String>;
    async fn update(&self, model: T) -> IoResult<()>;
    async fn delete(&self, id: &str) -> IoResult<()>;
}