use crate::models::ServiceInfo;
use serde_json;
use std::fs;
use std::path::Path;
use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, generic_array::GenericArray}};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use rand::RngCore;
use rand::rngs::OsRng;
use uuid::Uuid;

const SALT: &[u8] = b"securevault_salt_2023";
const ITERATIONS: u32 = 100_000;

fn derive_key(master_password: &str) -> Result<Vec<u8>, String> {
    let mut key = vec![0u8; 32];
    pbkdf2_hmac::<Sha256>(
        master_password.as_bytes(),
        SALT,
        ITERATIONS,
        &mut key,
    );
    Ok(key)
}

fn generate_nonce() -> GenericArray<u8, typenum::U12> {
    let mut nonce = GenericArray::default();
    OsRng.fill_bytes(&mut nonce);
    nonce
}

pub fn load_passwords(master_password: &str) -> Result<Vec<ServiceInfo>, String> {
    let path = Path::new("passwords.enc");
    if !path.exists() {
        return Ok(Vec::new());
    }

    let encrypted_data = fs::read(path)
        .map_err(|e| format!("Erreur de lecture: {}", e))?;

    if encrypted_data.len() < 12 {
        return Err("Fichier corrompu: nonce manquant".to_string());
    }

    let (nonce, ciphertext) = encrypted_data.split_at(12);
    let nonce = GenericArray::from_slice(nonce);
    let key = derive_key(master_password)?;
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));

    let decrypted = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Erreur de déchiffrement: {}", e))?;

    serde_json::from_slice(&decrypted)
        .map_err(|e| format!("Erreur de désérialisation: {}", e))
}

pub fn save_passwords(passwords: &[ServiceInfo], master_password: &str) -> Result<(), String> {
    let data = serde_json::to_vec(passwords)
        .map_err(|e| format!("Erreur de sérialisation: {}", e))?;

    let key = derive_key(master_password)?;
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
    let nonce = generate_nonce();

    let encrypted = cipher.encrypt(&nonce, data.as_ref())
        .map_err(|e| format!("Erreur de chiffrement: {}", e))?;

    let mut result = nonce.to_vec();
    result.extend(encrypted);

    fs::write("passwords.enc", result)
        .map_err(|e| format!("Erreur d'écriture: {}", e))
}

pub fn add_password(
    passwords: &mut Vec<ServiceInfo>,
    service: String,
    username: String,
    password: String,
) -> ServiceInfo {
    let new_password = ServiceInfo {
        id: Uuid::new_v4().to_string(),
        service,
        username,
        password,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    passwords.push(new_password.clone());
    new_password
}

pub fn delete_password(
    passwords: &mut Vec<ServiceInfo>,
    id: &str,
) -> Result<(), String> {
    if let Some(index) = passwords.iter().position(|p| p.id == id) {
        passwords.remove(index);
        Ok(())
    } else {
        Err("Mot de passe non trouvé".to_string())
    }
}

pub fn search_passwords(passwords: &[ServiceInfo], query: &str) -> Vec<ServiceInfo> {
    passwords.iter()
        .filter(|p|
            p.service.to_lowercase().contains(&query.to_lowercase()) ||
            p.username.to_lowercase().contains(&query.to_lowercase())
        )
        .cloned()
        .collect()
}
