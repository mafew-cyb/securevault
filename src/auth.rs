use sha2::{Sha256, Digest};
use totp_lite::Totp;
use rand::Rng;

pub struct AuthManager;

impl AuthManager {
    pub fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_password(password: &str, hash: &str) -> bool {
        Self::hash_password(password) == hash
    }

    pub fn generate_totp_secret() -> String {
        let mut rng = rand::thread_rng();
        let secret: Vec<u8> = (0..20).map(|_| rng.gen()).collect();
        base64::encode(&secret)
    }

    pub fn verify_totp(secret: &str, code: &str) -> bool {
        let totp = Totp::new(secret.as_bytes(), 30);
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        totp.check(code, time)
    }
}
