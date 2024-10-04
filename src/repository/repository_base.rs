use std::io::Result as IoResult;

pub trait RepositoryBase<T> {
    fn get(&self, id: i32) -> IoResult<T>;
    fn create(&self, model: T) -> IoResult<i32>;
    fn update(&self, model: T) -> IoResult<()>;
    fn delete(&self, id: i32) -> IoResult<()>;
}