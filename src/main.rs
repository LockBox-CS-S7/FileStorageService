#[macro_use]
extern crate rocket;

mod encryption;
mod file_id;
mod file_management;
mod models;
mod repository;

use encryption::aes_encryption::{encrypt_file, get_decrypted_file_content};
use file_id::FileId;
use repository::file_repository::FileRepository;

use crate::models::FileModel;
use crate::repository::repository_base::RepositoryBase;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::response::Responder;
use rocket::tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const ID_LENGTH: usize = 36;
const DB_CONNECTION_URI: &str = "mysql://root:password@file-db:3306/file-db";

#[derive(FromForm)]
struct FileUpload<'r> {
    file: TempFile<'r>,
    user_id: String,
}

#[derive(Responder)]
#[response(status = 200, content_type = "application/octet-stream")]
struct FileStreamResponse(String);

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/api", routes![test_route, get_file_by_id, upload_file])
        .launch()
        .await?;

    Ok(())
}

#[route(GET, uri = "/test")]
fn test_route() -> &'static str {
    "hello world"
}

#[get("/<file_id>")]
async fn get_file_by_id(file_id: &str) -> std::io::Result<File> {
    let repo = FileRepository::new(DB_CONNECTION_URI);
    let model = repo.get(file_id).await?;

    let temp_id = FileId::new(ID_LENGTH);
    let file_name = format!(
        "{}.{}", 
        temp_id.file_path().as_path().to_str().unwrap(),
        model.file_type,
    );
    let mut file = File::create(file_name).await?;
    file.write_all(&model.contents).await?;

    Ok(file)
}

#[post("/", data = "<form>")]
async fn upload_file(form: Form<FileUpload<'_>>) -> std::io::Result<String> {
    let mut file_buffer = Vec::new();
    let mut buf_read = form.file.open().await?;
    buf_read.read_to_end(&mut file_buffer).await?;

    let file_name = form.file.name().unwrap();
    let file_extension = form.file.content_type().unwrap().0.extension().unwrap();

    let repo = FileRepository::new(DB_CONNECTION_URI);
    let model = FileModel {
        id: None,
        file_name: String::from(file_name),
        file_type: file_extension.to_string(),
        contents: file_buffer,
    };

    let file_id = repo.create(model).await?;
    Ok(format!("File uploaded successfully (id = {file_id})"))
}
