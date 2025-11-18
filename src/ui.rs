use eframe::{egui, epi};
use crate::models::ServiceInfo;
use crate::vault::{save_passwords, load_passwords, search_passwords, add_password, delete_password, share_password};
use crate::password_generator::PasswordGenerator;
use crate::auth::AuthManager;
use crate::history::HistoryManager;

pub struct MyApp {
    pub service: String,
    pub username: String,
    pub password: String,
    pub master_password: String,
    pub totp_code: String,
    pub passwords: Vec<ServiceInfo>,
    pub filtered_passwords: Vec<ServiceInfo>,
    pub search_query: String,
    pub error: String,
    pub success: String,
    pub unlocked: bool,
    pub show_generator: bool,
    pub generated_password: String,
    pub gen_length: usize,
    pub gen_uppercase: bool,
    pub gen_numbers: bool,
    pub gen_symbols: bool,
    pub show_history: bool,
    pub history: Vec<crate::models::HistoryEntry>,
    pub selected_password_id: Option<String>,
    pub share_username: String,
}

impl epi::App for MyApp {
    fn name(&self) -> &str { "Gestionnaire de mots de passe sécurisé" }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔐 Gestionnaire de mots de passe");

            if !self.unlocked {
                self.render_login(ui);
                return;
            }

            ui.horizontal(|ui| {
                if ui.button("➕ Ajouter").clicked() {
                    self.show_generator = false;
                }
                if ui.button("🔑 Générer").clicked() {
                    self.show_generator = true;
                }
                if ui.button("📜 Historique").clicked() {
                    self.show_history = !self.show_history;
                    if self.show_history {
                        self.history = HistoryManager::load_history().unwrap_or_default();
                    }
                }
                if ui.button("🚪 Déverrouiller").clicked() {
                    self.unlocked = false;
                    self.master_password.clear();
                }
            });

            ui.separator();

            if self.show_generator {
                self.render_generator(ui);
            } else if self.show_history {
                self.render_history(ui);
            } else {
                self.render_password_manager(ui);
            }

            if !self.error.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.error);
            }
            if !self.success.is_empty() {
                ui.colored_label(egui::Color32::GREEN, &self.success);
            }
        });
    }
}

impl MyApp {
    fn render_login(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.heading("Connexion");
            ui.label("Mot de passe maître :");
            ui.add(egui::TextEdit::singleline(&mut self.master_password).password(true));
            ui.label("Code 2FA (optionnel) :");
            ui.text_edit_singleline(&mut self.totp_code);

            if ui.button("Déverrouiller").clicked() {
                match load_passwords(&self.master_password) {
                    Ok(list) => {
                        self.passwords = list;
                        self.filtered_passwords = self.passwords.clone();
                        self.unlocked = true;
                        self.error.clear();
                        self.success = "Connecté avec succès !".to_string();
                        let _ = HistoryManager::add_entry("login", "system");
                    }
                    Err(e) => self.error = format!("Erreur : {}", e),
                }
            }
        });
    }

    fn render_password_manager(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Rechercher :");
            if ui.text_edit_singleline(&mut self.search_query).changed() {
                self.filtered_passwords = search_passwords(&self.passwords, &self.search_query);
            }
        });

        ui.separator();
        ui.heading("Ajouter un nouveau mot de passe");

        ui.horizontal(|ui| {
            ui.label("Service :");
            ui.text_edit_singleline(&mut self.service);
        });
        ui.horizontal(|ui| {
            ui.label("Utilisateur :");
            ui.text_edit_singleline(&mut self.username);
        });
        ui.horizontal(|ui| {
            ui.label("Mot de passe :");
            ui.text_edit_singleline(&mut self.password);
            if ui.button("👁").clicked() {
                // Toggle password visibility (optionnel)
            }
        });

        ui.horizontal(|ui| {
            if ui.button("✅ Ajouter").clicked() {
                if !self.service.is_empty() && !self.username.is_empty() && !self.password.is_empty() {
                    let _ = add_password(&mut self.passwords, &self.service, &self.username, &self.password);
                    let _ = save_passwords(&self.passwords, &self.master_password);
                    let _ = HistoryManager::add_entry("created", &self.service);
                    self.success = format!("Mot de passe pour {} ajouté !", self.service);
                    self.service.clear();
                    self.username.clear();
                    self.password.clear();
                    self.filtered_passwords = self.passwords.clone();
                } else {
                    self.error = "Tous les champs sont obligatoires !".to_string();
                }
            }
        });

        ui.separator();
        ui.heading("Mots de passe enregistrés");

        for entry in &self.filtered_passwords {
            ui.horizontal(|ui| {
                ui.label(format!("📌 {} / {}", entry.service, entry.username));
                if ui.button("👁 Voir").clicked() {
                    self.selected_password_id = Some(entry.id.clone());
                }
                if ui.button("🗑 Supprimer").clicked() {
                    let _ = delete_password(&mut self.passwords, &entry.id);
                    let _ = save_passwords(&self.passwords, &self.master_password);
                    let _ = HistoryManager::add_entry("deleted", &entry.service);
                    self.success = format!("Mot de passe pour {} supprimé !", entry.service);
                    self.filtered_passwords = search_passwords(&self.passwords, &self.search_query);
                }
                if ui.button("🔗 Partager").clicked() {
                    self.selected_password_id = Some(entry.id.clone());
                }
            });

            if let Some(ref selected_id) = self.selected_password_id {
                if selected_id == &entry.id {
                    ui.horizontal(|ui| {
                        ui.label("Partager avec :");
                        ui.text_edit_singleline(&mut self.share_username);
                        if ui.button("Partager").clicked() {
                            let _ = share_password(&mut self.passwords, &entry.id, &self.share_username);
                            let _ = save_passwords(&self.passwords, &self.master_password);
                            self.success = format!("Mot de passe partagé avec {} !", self.share_username);
                            self.share_username.clear();
                            self.selected_password_id = None;
                        }
                    });
                    ui.label(format!("Partagé avec : {}", entry.shared_with.join(", ")));
                    ui.label(format!("Mot de passe : {}", entry.password));
                }
            }
        }
    }

    fn render_generator(&mut self, ui: &mut egui::Ui) {
        ui.heading("Générateur de mot de passe");

        ui.horizontal(|ui| {
            ui.label("Longueur :");
            ui.add(egui::Slider::new(&mut self.gen_length, 8..=32));
        });

        ui.checkbox(&mut self.gen_uppercase, "Majuscules");
        ui.checkbox(&mut self.gen_numbers, "Chiffres");
        ui.checkbox(&mut self.gen_symbols, "Symboles");

        if ui.button("Générer").clicked() {
            self.generated_password = PasswordGenerator::generate(
                self.gen_length,
                self.gen_uppercase,
                self.gen_numbers,
                self.gen_symbols,
            );
        }

        if !self.generated_password.is_empty() {
            ui.horizontal(|ui| {
                ui.label(format!("Mot de passe : {}", self.generated_password));
                if ui.button("📋 Copier").clicked() {
                    // Copier dans le presse-papiers (nécessite une dépendance supplémentaire)
                }
            });
            ui.label(format!("Force : {}", PasswordGenerator::evaluate_strength(&self.generated_password)));

            if ui.button("Utiliser ce mot de passe").clicked() {
                self.password = self.generated_password.clone();
                self.show_generator = false;
            }
        }
    }

    fn render_history(&mut self, ui: &mut egui::Ui) {
        ui.heading("Historique des actions");
        for entry in &self.history {
            ui.label(format!(
                "{} - {} sur {} ({})",
                entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                entry.action,
                entry.service,
                entry.id
            ));
        }
    }
}
