use aes_gcm::{aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Nonce, Key, AesGcm};
use aes_gcm::aead::consts::U12;
use aes_gcm::aes::Aes256;

fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<EncryptedData, aes_gcm::Error> {
    // Transformed from a byte array:
    let key: &[u8; 32] = &[42; 32];
    let key: &Key<Aes256Gcm> = key.into();

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    
    let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
    // let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
    // assert_eq!(&plaintext, b"plaintext message");
    
    Ok(EncryptedData::new(&ciphertext, &nonce))
}

fn decrypt(data: EncryptedData, key: &[u8; 32]) -> Result<Vec<u8>, aes_gcm::Error> {
    
    
    todo!()
}


pub struct EncryptedData {
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
}