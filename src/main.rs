#[macro_use]
extern crate rocket;

extern crate dotenv;

mod encryption;
mod file_id;
mod file_management;
mod models;
mod repository;
mod fairings;
mod logging;

use chrono::Utc;
use file_id::FileId;
use repository::file_repository::FileRepository;

use models::FileModel;
use repository::repository_base::RepositoryBase;
use fairings::{CORS, RequestLogging};

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::tokio::fs::File;
use rocket::serde::json::Json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::logging::init_file_logger;
use log::info;
use dotenv::dotenv;

const ID_LENGTH: usize = 36;

#[derive(FromForm)]
struct FileUpload<'r> {
    file: TempFile<'r>,
    user_id: String,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    init_file_logger();
    create_temp_files_dir().await.ok();
    
    let _rocket = rocket::build()
        .mount("/api", routes![test_route, get_file_by_id, get_user_files, upload_file])
        .attach(CORS)
        .attach(RequestLogging)
        .launch()
        .await?;
    
    info!("Rocket application started successfully. {}", Utc::now().to_string());
    Ok(())
}

#[route(GET, uri = "/test")]
fn test_route() -> &'static str {
    "hello world"
}

#[get("/<file_id>")]
async fn get_file_by_id(file_id: &str) -> Option<File> {
    let repo = FileRepository::from_env();
    let model = repo.read(&file_id).await.ok()?;
    
    let temp_id = FileId::new(ID_LENGTH);
    let file_name = format!(
        "{}.{}",
        temp_id.file_path().as_path().to_str()?,
        model.file_type,
    );
    let mut file = File::create(&file_name).await.ok()?;
    file.write_all(&model.contents).await.ok()?;
    file.flush().await.ok()?;
    
    File::open(&file_name).await.ok()
}


#[get("/user-files/<user_id>")]
async fn get_user_files(user_id: &str) -> Json<Vec<FileModel>> {
    let repo = FileRepository::from_env();
    let files = repo.get_files_by_user_id(user_id).await.unwrap();
    
    Json(files)
}


#[post("/", data = "<form>")]
async fn upload_file(form: Form<FileUpload<'_>>) -> std::io::Result<String> {
    let mut file_buffer = Vec::new();
    let mut buf_read = form.file.open().await?;
    buf_read.read_to_end(&mut file_buffer).await?;
    
    let file_name = form.file.name().unwrap();
    let file_extension = form.file.content_type().unwrap().0.extension().unwrap_or("".into());
    
    let repo = FileRepository::from_env();
    let model = FileModel {
        id: None,
        user_id: form.user_id.clone(),
        file_name: String::from(file_name),
        file_type: file_extension.to_string(),
        contents: file_buffer,
    };
    
    let file_id = repo.create(model).await?;
    Ok(format!("File uploaded successfully (id = {file_id})"))
}

/// Creates the _'temp-files'_ directory for temporary file storage if it doesn't exist yet.
async fn create_temp_files_dir() -> std::io::Result<()> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "temp-files");
    let read_dir_res = rocket::tokio::fs::read_dir(path).await;
    
    if read_dir_res.is_err() {
        rocket::tokio::fs::create_dir(path).await?;
    }
    
    Ok(())
}
