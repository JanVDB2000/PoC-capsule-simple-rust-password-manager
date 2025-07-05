use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use argon2::{password_hash::SaltString, Argon2};
use rand::RngCore;

pub const NONCE_SIZE: usize = 12;

pub fn derive_key(password: &str, salt: &[u8]) -> Key<Aes256Gcm> {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2.hash_password_into(password.as_bytes(), salt, &mut key).unwrap();
    Key::<Aes256Gcm>::from_slice(&key).clone()
}

pub fn encrypt(data: &[u8], key: &Key<Aes256Gcm>) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key);
    let mut nonce = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce);
    let nonce = Nonce::from_slice(&nonce);
    let ciphertext = cipher.encrypt(nonce, data).unwrap();
    [nonce.as_slice(), &ciphertext].concat()
}

pub fn decrypt(data: &[u8], key: &Key<Aes256Gcm>) -> Vec<u8> {
    let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = Aes256Gcm::new(key);
    cipher.decrypt(nonce, ciphertext).unwrap()
}

pub fn generate_salt() -> Vec<u8> {
    SaltString::generate(&mut OsRng).to_string().into_bytes()
}
