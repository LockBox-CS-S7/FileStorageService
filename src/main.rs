#[macro_use] extern crate rocket;

mod encryption;
pub mod file_management;
mod file_id;

use encryption::aes_encryption::{decrypt_file, encrypt_file};
use file_id::FileId;

use rocket::fs::TempFile;
use rocket::form::Form;
use rocket::tokio::fs::File;

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


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![test_route, get_file_by_id, upload_file])
}

#[route(GET, uri = "/test")]
fn test_route() -> &'static str {
    "hello world"
}

#[get("/", data = "<form>")]
async fn get_file_by_id(form: Form<GetFileForm>) -> Option<File> {
    let file_id = FileId::from_id(&form.file_id).ok()?;
    let file_path = file_id.file_path();
    let file_path = file_path.to_str()?;
    
    decrypt_file(file_path, form.password.clone());
    File::open(file_path).await.ok()
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
