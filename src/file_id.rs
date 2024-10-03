use std::borrow::Cow;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use rand::{self, Rng};
use rocket::request::FromParam;



/// A _probably_ unique paste ID.
#[derive(UriDisplayPath)]
pub struct FileId<'a>(Cow<'a, str>);

impl FileId<'_> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize) -> FileId<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        FileId(Cow::Owned(id))
    }
    
    /// Creates a new FileId from a string slice, returns _ErrorKind::NotFound_ 
    /// when no file with given id is stored on disk.
    pub fn from_id(id: &str) -> std::io::Result<Self> {
        let file_id = Self(Cow::Owned(id.to_string()));
        
        // Return an error when the entered id does not have a corresponding file.
        if !file_id.file_path().is_file() {
            return Err(std::io::Error::new(
                ErrorKind::NotFound, 
                "No file with given id found."
            ));
        }
        
        Ok(file_id)
    }

    /// Returns the path to the paste in `upload/` corresponding to this ID.
    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads");
        Path::new(root).join(self.0.as_ref())
    }
}

/// Returns an instance of `PasteId` if the path segment is a valid ID.
/// Otherwise, returns the invalid ID as the `Err` value.
impl<'a> FromParam<'a> for FileId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param.chars().all(|c| c.is_ascii_alphanumeric())
            .then(|| FileId(param.into()))
            .ok_or(param)
    }
}
