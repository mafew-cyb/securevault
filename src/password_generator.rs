use rand::Rng;

pub struct PasswordGenerator;

impl PasswordGenerator {
    pub fn generate(length: usize, include_uppercase: bool, include_numbers: bool, include_symbols: bool) -> String {
        let mut rng = rand::thread_rng();
        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let numbers = "0123456789";
        let symbols = "!@#$%^&*()_+-=[]{}|;:,.<>?";

        let mut charset = lowercase.to_string();
        if include_uppercase {
            charset.push_str(uppercase);
        }
        if include_numbers {
            charset.push_str(numbers);
        }
        if include_symbols {
            charset.push_str(symbols);
        }

        let chars: Vec<char> = charset.chars().collect();
        (0..length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect()
    }

    pub fn evaluate_strength(password: &str) -> String {
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_numbers = password.chars().any(|c| c.is_numeric());
        let has_symbols = password.chars().any(|c| !c.is_alphanumeric());
        let length = password.len();

        let score = [has_lowercase, has_uppercase, has_numbers, has_symbols]
            .iter()
            .filter(|&&x| x)
            .count() + (length / 8);

        match score {
            0..=2 => "Faible".to_string(),
            3..=4 => "Moyen".to_string(),
            5..=6 => "Fort".to_string(),
            _ => "Très fort".to_string(),
        }
    }
}
