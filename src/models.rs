use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceInfo {
    pub id: String,
    pub service: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub shared_with: Vec<String>, // Liste des utilisateurs avec qui c'est partagé
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HistoryEntry {
    pub id: String,
    pub action: String, // "created", "updated", "deleted", "accessed"
    pub service: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub master_password_hash: String,
    pub totp_secret: Option<String>, // Pour 2FA
    pub created_at: DateTime<Utc>,
}
