use aes_gcm::{aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Nonce, Key};
use crate::encryption::key_generation;
use crate::file_management::{read_file, clear_write_file};


fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<EncryptedData, aes_gcm::Error> {
    // Transformed from a byte array:
    let key: &Key<Aes256Gcm> = key.into();

    let cipher = Aes256Gcm::new(key);
    
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, data)?;
    
    Ok(EncryptedData::new(&ciphertext, &nonce))
}

fn decrypt(data: EncryptedData, key: &[u8; 32]) -> Result<Vec<u8>, aes_gcm::Error> {
    let key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(key);
    
    let nonce = Nonce::clone_from_slice(data.nonce.as_slice());
    let plain_text = cipher.decrypt(&nonce, data.data.as_slice())?;
    
    Ok(plain_text)
}


pub fn encrypt_file(path: &str, passphrase: String) {
    let (bytes_read, file_content) = read_file(path)
        .expect("Failed to read file.");

    println!("Bytes to encrypt: {}", bytes_read);

    let file_content = file_content.as_slice();
    let salt = key_generation::generate_salt();
    let key = key_generation::derive_key_from_passphrase(passphrase.as_bytes(), &salt);

    let encrypted_data = encrypt(file_content, &key).expect("An error occurred when encrypting data.");

    // Add the Nonce at the beginning of the encrypted data for later retrieval.
    let mut new_file_content = encrypted_data.nonce().clone();
    new_file_content.append(&mut salt.to_vec()); // Add the used salt
    new_file_content.append(encrypted_data.data().clone().as_mut());

    clear_write_file(path, new_file_content).expect("Failed to write encrypted data to file.");
}

/// Decrypts a file, and writes the decrypted content to it. 
#[allow(unused)]
pub fn decrypt_file(path: &str, passphrase: String) -> std::io::Result<()> {
    let decrypted_data = get_decrypted_file_content(path, passphrase)?;
    clear_write_file(path, decrypted_data).expect("Failed to write encrypted contents to file.");
    Ok(())
}

/// Decrypts a file's content and returns it, without actually decrypting the file on disk.
pub fn get_decrypted_file_content(path: &str, passphrase: String) -> std::io::Result<Vec<u8>> {
    let (bytes_read, file_contents) = read_file(path)?;
    println!("Bytes read: {}", bytes_read);
    let mut file_contents = file_contents;

    // Get the nonce from the first 12 bytes of the file.
    let nonce_drain = file_contents.drain(0..12);
    let mut nonce: Vec<u8> = Vec::new();
    for byte in nonce_drain {
        nonce.push(byte);
    }

    // Get the salt from the now first 16 bytes of the file.
    let salt_drain = file_contents.drain(0..16);
    let mut salt: Vec<u8> = Vec::new();
    for byte in salt_drain {
        salt.push(byte);
    }

    let encrypted_data = EncryptedData::new(file_contents.as_slice(), nonce.as_slice());
    let key = key_generation::derive_key_from_passphrase(passphrase.as_bytes(), salt.as_slice());
    let decrypted_data = 
        decrypt(encrypted_data, &key).expect("Failed to decrypt file content.");
    
    Ok(decrypted_data)
}


struct EncryptedData {
    data: Vec<u8>,
    nonce: Vec<u8>,
}

impl EncryptedData {
    pub fn new(data: &[u8], nonce: &[u8]) -> Self {
        Self { 
            data: data.to_vec(), 
            nonce: nonce.to_vec(),
        }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn nonce(&self) -> &Vec<u8> {
        &self.nonce
    }
}