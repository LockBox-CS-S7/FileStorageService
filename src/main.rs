mod aes_encryption;
mod key_generation;
mod file_management;

use std::env;
use std::fs;
use aes_encryption::{encrypt_file, decrypt_file};


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1).expect("file path was not provided.");
    let passphrase = args.get(2).expect("key was not provided");
    let command = args.get(3).expect("command was not provided").as_ref();

    if fs::read(file_path).is_err() {
        panic!("The given file path does not lead to a file.");
    }

    match command {
        "encrypt" => {
            encrypt_file(file_path, passphrase.clone());
            println!("Successfully encrypted file!");
        },
        "decrypt" => {
            decrypt_file(file_path, passphrase.clone());
            println!("Successfully decrypted file!");
        },
        _ => {
            panic!("Invalid command.");
        }
    }
}
