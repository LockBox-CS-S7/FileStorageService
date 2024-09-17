mod aes_encryption;

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Read, Write};
use aes_encryption::{encrypt, decrypt};
use crate::aes_encryption::EncryptedData;

const KEY_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1).expect("file path was not provided.");
    let key = args.get(2).expect("key was not provided");
    let command = args.get(3).expect("command was not provided").as_ref();

    if fs::read(file_path).is_err() {
        panic!("The given file path does not lead to a file.");
    }

    match command {
        "encrypt" => {
            encrypt_file(file_path, key.clone());
            println!("Successfully encrypted file!");
        },
        "decrypt" => {
            decrypt_file(file_path, key.clone());
            println!("Successfully decrypted file!");
        },
        _ => {
            panic!("Invalid command.");
        }
    }
}


fn encrypt_file(path: &str, key: String) {
    let (bytes_read, file_content) = read_file(path)
        .expect("Failed to read file.");

    println!("Bytes to encrypt: {}", bytes_read);

    let file_content = file_content.as_bytes();
    let key = parse_key(key.clone()).expect("Failed to parse the given key.");

    let encrypted_data = encrypt(file_content, &key).expect("An error occurred when encrypting data.");

    // Add the Nonce at the beginning of the encrypted data for later retrieval.
    let mut new_file_content = encrypted_data.nonce().clone();
    new_file_content.append(encrypted_data.data().clone().as_mut());

    clear_write_file(path, new_file_content).expect("Failed to write encrypted data to file.");
}

fn decrypt_file(path: &str, key: String) {
    let (bytes_read, file_contents) = read_file(path).unwrap();
    let mut file_contents = file_contents.as_bytes().to_vec();

    let drain = file_contents.drain(0..NONCE_LENGTH);
    let mut nonce: Vec<u8> = Vec::new();
    for byte in drain {
        nonce.push(byte);
    }
    
    let encrypted_data = EncryptedData::new(file_contents.as_slice(), nonce.as_slice());
    let key = parse_key(key).unwrap();
    let decrypted_data = decrypt(encrypted_data, &key).unwrap();

    clear_write_file(path, decrypted_data).expect("Failed to write encrypted contents to file.");
}


/// Reads a file and returns (bytes_read, file_content)
fn read_file(path: &str) -> std::io::Result<(usize, String)> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)?;

    let mut content_buffer = String::new();
    let bytes_read = file.read_to_string(&mut content_buffer)?;
    file.flush()?;

    Ok((bytes_read, content_buffer))
}

/// Sets the file length to 0, then writes the given content to it.
fn clear_write_file(path: &str, new_content: Vec<u8>) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    file.set_len(0)?;
    file.write_all(new_content.as_ref())?;
    file.flush()?;

    Ok(())
}

fn parse_key(key: String) -> std::io::Result<[u8; 32]> {
    let key = key.as_bytes();
    if key.len() != KEY_LENGTH {
        return Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "Given key did not have the correct length."
        ));
    }

    let mut parsed_key: [u8; KEY_LENGTH] = [0; KEY_LENGTH];
    for i in 0..KEY_LENGTH {
        parsed_key[i] = key[i];
    }

    Ok(parsed_key)
}
