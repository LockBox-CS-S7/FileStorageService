#[macro_use] extern crate rocket;

mod encryption;
pub mod file_management;
mod file_id;

use encryption::aes_encryption::{decrypt_file, encrypt_file};

use rocket::data::{Data, ToByteUnit};
use file_id::FileId;


const ID_LENGTH: usize = 12;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![test_route, upload_file])
}

#[route(GET, uri = "/test")]
fn test_route() -> &'static str {
    "hello world"
}

#[post("/", data = "<file>")]
async fn upload_file(file: Data<'_>) -> std::io::Result<String> {
    let id = FileId::new(ID_LENGTH);
    file.open(2024.mebibytes()).into_file(id.file_path()).await?;
    Ok(String::from("File uploaded successfully!"))
}
