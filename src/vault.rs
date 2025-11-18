use crate::models::ServiceInfo;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use sha2::{Sha256, Digest};
use base64::{encode, decode};
use uuid::Uuid;

const FILE_NAME: &str = "passwords.enc";

pub fn derive_key(master_password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(master_password.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

pub fn save_passwords(passwords: &[ServiceInfo], master_password: &str) -> Result<(), String> {
    let key = Key::from_slice(&derive_key(master_password));
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"unique nonce12"); // 12 bytes

    let json = serde_json::to_string(passwords).map_err(|e| e.to_string())?;
    let ciphertext = cipher.encrypt(nonce, json.as_bytes()).map_err(|e| e.to_string())?;
    let encoded = encode(&ciphertext);

    let mut file = File::create(FILE_NAME).map_err(|e| e.to_string())?;
    file.write_all(encoded.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_passwords(master_password: &str) -> Result<Vec<ServiceInfo>, String> {
    let key = Key::from_slice(&derive_key(master_password));
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"unique nonce12"); // 12 bytes

    let mut file = match File::open(FILE_NAME) {
        Ok(f) => f,
        Err(_) => return Ok(Vec::new()),
    };
    let mut encoded = String::new();
    file.read_to_string(&mut encoded).map_err(|e| e.to_string())?;
    let ciphertext = decode(&encoded).map_err(|e| e.to_string())?;
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).map_err(|e| e.to_string())?;
    let passwords: Vec<ServiceInfo> = serde_json::from_slice(&plaintext).map_err(|e| e.to_string())?;
    Ok(passwords)
}

pub fn search_passwords(passwords: &[ServiceInfo], query: &str) -> Vec<ServiceInfo> {
    passwords
        .iter()
        .filter(|p| {
            p.service.to_lowercase().contains(&query.to_lowercase())
                || p.username.to_lowercase().contains(&query.to_lowercase())
        })
        .cloned()
        .collect()
}

pub fn add_password(
    passwords: &mut Vec<ServiceInfo>,
    service: &str,
    username: &str,
    password: &str,
) -> Result<(), String> {
    let entry = ServiceInfo {
        id: Uuid::new_v4().to_string(),
        service: service.to_string(),
        username: username.to_string(),
        password: password.to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        shared_with: Vec::new(),
    };
    passwords.push(entry);
    Ok(())
}

pub fn delete_password(passwords: &mut Vec<ServiceInfo>, id: &str) -> Result<(), String> {
    passwords.retain(|p| p.id != id);
    Ok(())
}

pub fn share_password(passwords: &mut Vec<ServiceInfo>, id: &str, username: &str) -> Result<(), String> {
    if let Some(entry) = passwords.iter_mut().find(|p| p.id == id) {
        if !entry.shared_with.contains(&username.to_string()) {
            entry.shared_with.push(username.to_string());
        }
    }
    Ok(())
}
