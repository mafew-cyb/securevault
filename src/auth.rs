use sha2::{Sha256, Digest};
use rand::Rng;
use base64::{engine::general_purpose, Engine as _};
use totp_lite::{totp, Sha1};

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
        general_purpose::STANDARD.encode(&secret)
    }

    pub fn verify_totp(secret_b64: &str, code: &str) -> bool {
        let secret = match general_purpose::STANDARD.decode(secret_b64) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let expected = format!("{:06}", totp::<Sha1>(&secret, 30));
        expected == code
    }
}
