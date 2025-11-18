use eframe::egui;
use crate::models::ServiceInfo;
use crate::vault::{load_passwords, add_password, delete_password, search_passwords, save_passwords};
use crate::password_generator::PasswordGenerator;
use arboard::Clipboard;

pub struct MyApp {
    service: String,
    username: String,
    password: String,
    master_password: String,
    passwords: Vec<ServiceInfo>,
    filtered_passwords: Vec<ServiceInfo>,
    search_query: String,
    error: String,
    success: String,
    unlocked: bool,
    show_generator: bool,
    generated_password: String,
    gen_length: usize,
    gen_uppercase: bool,
    gen_numbers: bool,
    gen_symbols: bool,
    selected_password_id: Option<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            service: String::new(),
            username: String::new(),
            password: String::new(),
            master_password: String::new(),
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
            selected_password_id: None,
        }
    }
}

impl MyApp {
    fn unlock(&mut self) {
        if self.master_password.is_empty() {
            self.error = "Le mot de passe maître est requis".to_string();
            return;
        }

        match load_passwords(&self.master_password) {
            Ok(passwords) => {
                self.passwords = passwords;
                self.filtered_passwords = self.passwords.clone();
                self.unlocked = true;
                self.success = "Déverrouillé avec succès!".to_string();
                self.error.clear();
            }
            Err(e) => {
                self.error = format!("Échec du déverrouillage: {}", e);
                self.success.clear();
            }
        }
    }

    fn lock(&mut self) {
        self.unlocked = false;
        self.passwords.clear();
        self.filtered_passwords.clear();
        self.master_password.clear();
        self.success = "Verrouillé avec succès".to_string();
        self.error.clear();
    }

    fn add_new_password(&mut self) {
        if self.service.is_empty() || self.username.is_empty() || self.password.is_empty() {
            self.error = "Tous les champs sont requis".to_string();
            return;
        }

        // Suppression de la variable inutilisée new_password
        add_password(
            &mut self.passwords,
            self.service.clone(),
            self.username.clone(),
            self.password.clone(),
        );

        if let Err(e) = save_passwords(&self.passwords, &self.master_password) {
            self.error = format!("Échec de la sauvegarde: {}", e);
            self.success.clear();
            self.passwords.pop(); // Annuler l'ajout si sauvegarde échoue
            return;
        }

        self.filtered_passwords = self.passwords.clone();
        self.success = "Mot de passe ajouté avec succès!".to_string();
        self.error.clear();
        self.service.clear();
        self.username.clear();
        self.password.clear();
    }

    fn delete_selected_password(&mut self) {
        if let Some(id) = &self.selected_password_id {
            if let Err(e) = delete_password(&mut self.passwords, id) {
                self.error = format!("Échec de la suppression: {}", e);
                self.success.clear();
                return;
            }

            if let Err(e) = save_passwords(&self.passwords, &self.master_password) {
                self.error = format!("Échec de la sauvegarde: {}", e);
                self.success.clear();
                return;
            }

            self.filtered_passwords = self.passwords.clone();
            self.success = "Mot de passe supprimé avec succès!".to_string();
            self.error.clear();
            self.selected_password_id = None;
        }
    }

    fn copy_to_clipboard(&self, text: &str) {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(text.to_string()).unwrap();
    }

    fn generate_password(&mut self) {
        let generator = PasswordGenerator::new()
            .length(self.gen_length)
            .uppercase(self.gen_uppercase)
            .numbers(self.gen_numbers)
            .symbols(self.gen_symbols);

        self.generated_password = generator.generate();
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.unlocked {
                self.render_login(ui);
            } else {
                self.render_main_ui(ui);
            }
        });
    }
}

impl MyApp {
    fn render_login(&mut self, ui: &mut egui::Ui) {
        ui.heading("SecureVault - Connexion");
        ui.separator();

        ui.label("Mot de passe maître:");
        if ui.add(egui::TextEdit::singleline(&mut self.master_password)
            .password(true)
            .hint_text("Entrez votre mot de passe maître"))
            .lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
        {
            self.unlock();
        }

        if ui.button("Déverrouiller").clicked() {
            self.unlock();
        }

        if !self.error.is_empty() {
            ui.colored_label(egui::Color32::RED, &self.error);
        }
    }

    fn render_main_ui(&mut self, ui: &mut egui::Ui) {
        // En-tête
        ui.horizontal(|ui| {
            ui.heading("SecureVault - Gestionnaire de mots de passe");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Verrouiller").clicked() {
                    self.lock();
                }
                if ui.button("Générateur").clicked() {
                    self.show_generator = true;
                }
            });
        });

        ui.separator();

        // Barre de recherche
        ui.horizontal(|ui| {
            ui.label("Rechercher:");
            if ui.text_edit_singleline(&mut self.search_query).changed() {
                self.filtered_passwords = search_passwords(&self.passwords, &self.search_query);
            }
        });

        // Messages
        if !self.success.is_empty() {
            ui.colored_label(egui::Color32::GREEN, &self.success);
        }
        if !self.error.is_empty() {
            ui.colored_label(egui::Color32::RED, &self.error);
        }

        ui.separator();

        // Formulaire d'ajout
        ui.group(|ui| {
            ui.heading("Ajouter un nouveau mot de passe");

            ui.label("Service:");
            ui.text_edit_singleline(&mut self.service);

            ui.label("Nom d'utilisateur:");
            ui.text_edit_singleline(&mut self.username);

            ui.label("Mot de passe:");
            if ui.add(egui::TextEdit::singleline(&mut self.password)
                .password(true)
                .hint_text("Mot de passe"))
                .lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
            {
                self.add_new_password();
            }

            if ui.button("Ajouter").clicked() {
                self.add_new_password();
            }
        });

        ui.separator();

        // Liste des mots de passe
        ui.group(|ui| {
            ui.heading("Vos mots de passe");

            egui::ScrollArea::vertical().show(ui, |ui| {
                // On clone les données nécessaires avant la boucle
                let passwords = self.filtered_passwords.clone();
                let selected_id = self.selected_password_id.clone();
                let mut id_to_select = None;
                let mut text_to_copy = None;

                for password in passwords {
                    let is_selected = selected_id.as_ref() == Some(&password.id);

                    ui.horizontal(|ui| {
                        if ui.selectable_label(is_selected, &format!("{} - {}", password.service, password.username))
                        .clicked() {
                            id_to_select = Some(password.id);
                        }

                        if ui.button("Copier").clicked() {
                            text_to_copy = Some(password.password);
                        }
                    });
                }

                // Mettre à jour l'état après la boucle
                if let Some(id) = id_to_select {
                    self.selected_password_id = Some(id);
                }

                if let Some(text) = text_to_copy {
                    self.copy_to_clipboard(&text);
                    self.success = "Mot de passe copié dans le presse-papiers!".to_string();
                }
            });

            if self.selected_password_id.is_some() {
                if ui.button("Supprimer").clicked() {
                    self.delete_selected_password();
                }
            }
        });


        // Générateur de mots de passe (fenêtre modale)
        if self.show_generator {
            let mut open = self.show_generator;
            egui::Window::new("Générateur de mots de passe")
                .open(&mut open)
                .show(ui.ctx(), |ui| {
                    ui.label("Longueur:");
                    ui.add(egui::DragValue::new(&mut self.gen_length).clamp_range(8..=64));

                    ui.checkbox(&mut self.gen_uppercase, "Majuscules");
                    ui.checkbox(&mut self.gen_numbers, "Chiffres");
                    ui.checkbox(&mut self.gen_symbols, "Symboles");

                    if ui.button("Générer").clicked() {
                        self.generate_password();
                    }

                    if !self.generated_password.is_empty() {
                        ui.label("Mot de passe généré:");
                        ui.monospace(&self.generated_password);
                        if ui.button("Copier").clicked() {
                            self.copy_to_clipboard(&self.generated_password);
                            self.success = "Mot de passe généré copié!".to_string();
                        }
                    }
                });
            self.show_generator = open;
        }
    }
}
