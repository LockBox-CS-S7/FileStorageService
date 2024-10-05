#[derive(Debug, Clone, PartialEq)]
pub struct FileModel {
    pub file_name: String,
    pub file_type: String,
    pub contents: Vec<u8>,
}
impl FileModel {
    pub fn new(file_name: String, file_type: String, contents: Vec<u8>) -> Self {
        Self { file_name, file_type, contents }
    }
}
