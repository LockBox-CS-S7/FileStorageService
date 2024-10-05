use std::io::Result as IoResult;

pub trait RepositoryBase<T> {
    async fn get(&self, id: String) -> IoResult<T>;
    async fn create(&self, model: T) -> IoResult<String>;
    async fn update(&self, model: T) -> IoResult<()>;
    async fn delete(&self, id: String) -> IoResult<()>;
}