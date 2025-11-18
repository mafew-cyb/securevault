mod models;
mod vault;
mod ui;
mod password_generator;
mod auth;
mod history;

use ui::MyApp;

fn main() {
    let app = MyApp {
        service: String::new(),
        username: String::new(),
        password: String::new(),
        master_password: String::new(),
        totp_code: String::new(),
        passwords: Vec::new(),
        filtered_passwords: Vec::new(),
        search_query: String::new(),
        error: String::new(),
        success: String::new(),
        unlocked: false,
        show_generator: false,
        generated_password: String::new(),
        gen_length: 16,
        gen_uppercase: true,
        gen_numbers: true,
        gen_symbols: true,
        show_history: false,
        history: Vec::new(),
        selected_password_id: None,
        share_username: String::new(),
    };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
