use crate::models::HistoryEntry;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use chrono::Utc;
use uuid::Uuid;

const HISTORY_FILE: &str = "history.json";

pub struct HistoryManager;

impl HistoryManager {
    pub fn add_entry(action: &str, service: &str) -> Result<(), String> {
        let mut history = Self::load_history()?;
        let entry = HistoryEntry {
            id: Uuid::new_v4().to_string(),
            action: action.to_string(),
            service: service.to_string(),
            timestamp: Utc::now(),
        };
        history.push(entry);
        Self::save_history(&history)?;
        Ok(())
    }

    pub fn load_history() -> Result<Vec<HistoryEntry>, String> {
        match File::open(HISTORY_FILE) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).map_err(|e| e.to_string())?;
                serde_json::from_str(&content).map_err(|e| e.to_string())
            }
            Err(_) => Ok(Vec::new()),
        }
    }

    pub fn save_history(history: &[HistoryEntry]) -> Result<(), String> {
        let mut file = File::create(HISTORY_FILE).map_err(|e| e.to_string())?;
        let json = serde_json::to_string(history).map_err(|e| e.to_string())?;
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}
