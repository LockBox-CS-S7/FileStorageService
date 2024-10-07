#[macro_use] extern crate rocket;

mod encryption;
mod file_management;
mod file_id;
mod repository;
mod models;

use encryption::aes_encryption::{
    encrypt_file, 
    get_decrypted_file_content
};
use file_id::FileId;
use repository::file_repository::FileRepository;

use rocket::fs::TempFile;
use rocket::form::Form;
use rocket::response::Responder;
use tokio::io::AsyncReadExt;
use crate::models::FileModel;
use crate::repository::repository_base::RepositoryBase;

const ID_LENGTH: usize = 12;
const DB_CONNECTION_URI: &str = "mysql://root:password@file-db:3306/file-db";


#[derive(FromForm)]
struct FileUpload<'r> {
    file: TempFile<'r>,
    password: String,
}

#[derive(FromForm)]
struct GetFileForm {
    file_id: String,
    password: String,
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

#[get("/", data = "<form>")]
async fn get_file_by_id(form: Form<GetFileForm>) -> Option<FileStreamResponse> {
    let file_id = FileId::from_id(&form.file_id).ok()?;
    let file_path = file_id.file_path();
    let file_path = file_path.to_str()?;
    
    let decrypted_contents = 
        get_decrypted_file_content(file_path, form.password.clone()).ok()?;
    let decrypted_contents = String::from_utf8(decrypted_contents).ok()?;
    
    Some(FileStreamResponse(decrypted_contents))
}

#[post("/", data = "<form>")]
async fn upload_file(mut form: Form<FileUpload<'_>>) -> std::io::Result<String> {
    let mut file_buffer = Vec::new();
    let mut buf_read = form.file.open().await?;
    buf_read.read_to_end(&mut file_buffer).await?;
    
    let repo = FileRepository::new(DB_CONNECTION_URI);
    let model = FileModel::new(
        String::from("test-file"),
        String::from("text"),
        file_buffer
    );

    repo.create(model).await?;
    
    Ok(String::from("File uploaded successfully!"))
}
