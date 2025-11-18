use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Utc, DateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub id: String,
    pub service: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for ServiceInfo {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            service: String::new(),
            username: String::new(),
            password: String::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
