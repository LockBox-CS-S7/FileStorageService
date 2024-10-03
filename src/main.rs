#[macro_use] extern crate rocket;

mod encryption;
mod file_management;
mod file_id;

use encryption::aes_encryption::{
    encrypt_file, 
    get_decrypted_file_content
};
use file_id::FileId;

use rocket::fs::TempFile;
use rocket::form::Form;
use rocket::response::Responder;

use sqlx::mysql::MySqlPoolOptions;

const ID_LENGTH: usize = 12;

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
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:password@file-db:33060/database").await.unwrap();

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await.unwrap();

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
    let id = FileId::new(ID_LENGTH);
    let path = id.file_path();
    form.file.persist_to(path.clone()).await?;
    
    // Encrypt the newly saved file
    let path = path.to_str().unwrap();
    let pass = form.password.clone();
    encrypt_file(path, pass);
    
    Ok(String::from("File uploaded successfully!"))
}
