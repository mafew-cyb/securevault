use rand::Rng;

pub struct PasswordGenerator {
    length: usize,
    uppercase: bool,
    numbers: bool,
    symbols: bool,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        Self {
            length: 16,
            uppercase: true,
            numbers: true,
            symbols: true,
        }
    }

    pub fn length(mut self, length: usize) -> Self {
        self.length = length.clamp(8, 64);
        self
    }

    pub fn uppercase(mut self, uppercase: bool) -> Self {
        self.uppercase = uppercase;
        self
    }

    pub fn numbers(mut self, numbers: bool) -> Self {
        self.numbers = numbers;
        self
    }

    pub fn symbols(mut self, symbols: bool) -> Self {
        self.symbols = symbols;
        self
    }

    pub fn generate(&self) -> String {
        let mut charset: Vec<char> = ('a'..='z').collect();

        if self.uppercase {
            charset.extend('A'..='Z');
        }
        if self.numbers {
            charset.extend('0'..='9');
        }
        if self.symbols {
            charset.extend("!@#$%^&*()_+-=[]{}|;:,.<>?".chars());
        }

        let mut rng = rand::thread_rng();
        (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx]
            })
            .collect()
    }
}
