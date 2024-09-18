use aes_gcm::{aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Nonce, Key};


pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<EncryptedData, aes_gcm::Error> {
    // Transformed from a byte array:
    let key: &Key<Aes256Gcm> = key.into();

    let cipher = Aes256Gcm::new(key);
    
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, data)?;
    
    Ok(EncryptedData::new(&ciphertext, &nonce))
}

pub fn decrypt(data: EncryptedData, key: &[u8; 32]) -> Result<Vec<u8>, aes_gcm::Error> {
    let key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(key);
    
    let nonce = Nonce::clone_from_slice(data.nonce.as_slice());
    let plain_text = cipher.decrypt(&nonce, data.data.as_slice())?;
    
    Ok(plain_text)
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

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn nonce(&self) -> &Vec<u8> {
        &self.nonce
    }
}