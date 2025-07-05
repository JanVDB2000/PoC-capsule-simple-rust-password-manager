use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use crate::crypto::{encrypt, decrypt};
use aes_gcm::{Aes256Gcm, Key};

pub const VAULT_PATH: &str = "capsule.vault";

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<Entry>,
}

impl Vault {
    pub fn new() -> Self {
        Vault { entries: vec![] }
    }

    pub fn load(key: &Key<Aes256Gcm>) -> Self {
        if PathBuf::from(VAULT_PATH).exists() {
            let data = fs::read(VAULT_PATH).unwrap();
            let decrypted = decrypt(&data, key);
            serde_json::from_slice(&decrypted).unwrap()
        } else {
            Vault::new()
        }
    }

    pub fn save(&self, key: &Key<Aes256Gcm>) {
        let data = serde_json::to_vec(self).unwrap();
        let encrypted = encrypt(&data, key);
        fs::write(VAULT_PATH, encrypted).unwrap();
    }
}
