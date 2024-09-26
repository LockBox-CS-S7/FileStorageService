use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct File {
    name: String,
    metadata: String,
    file_type: String,
}
