mod models;
mod vault;
mod ui;
mod password_generator;

use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        min_window_size: Some(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "SecureVault - Gestionnaire de mots de passe",
        options,
        Box::new(|_cc| Box::new(ui::MyApp::default())),
    )
}
