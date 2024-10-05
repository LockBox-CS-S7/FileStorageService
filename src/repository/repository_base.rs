use std::io::Result as IoResult;

pub trait RepositoryBase<T> {
    async fn get(&self, id: i32) -> IoResult<T>;
    async fn create(&self, model: T) -> IoResult<i32>;
    async fn update(&self, model: T) -> IoResult<()>;
    async fn delete(&self, id: i32) -> IoResult<()>;
}