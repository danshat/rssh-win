use std::collections::HashMap;

use config::Session;
use eframe::egui;
use regex::Regex;
use rfd::FileDialog;

mod config;
use config::AuthModes;

pub struct MyApp {
    app_config: config::AppConfiguration,
    add_session_open: bool,
    add_session: Session,
}

impl Default for MyApp {
    fn default() -> MyApp {
        MyApp {
            app_config: config::AppConfiguration::new().unwrap(),
            add_session_open: false,
            add_session: Session::new(),
        }
    }
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
    // New session popup window
    fn add_session_window(&mut self, ctx: &egui::Context) -> anyhow::Result<()> {
        egui::Window::new("Add new session").show(ctx, |ui| {
            ui.label("Session name: ");
            ui.text_edit_singleline(&mut self.add_session.name);
            ui.label("Host IP: ");
            ui.text_edit_singleline(&mut self.add_session.ip);
            ui.label("Port: ");
            let mut port_text: String = self.add_session.port.to_string();
            if ui.text_edit_singleline(&mut port_text).changed() {
                let re = Regex::new(r"[^0-9]+").unwrap();
                port_text = re.replace_all(&port_text, "").to_string();
            };
            let new_port = port_text.parse::<i32>();
            if new_port.is_ok() {
                let new_port_unwrapped = new_port.unwrap();
                if (0..65536).contains(&new_port_unwrapped) {
                    self.add_session.port = new_port_unwrapped;
                }
            };
            ui.label("Username: ");
            let mut username_text: String = String::from("");
            if self.add_session.username.is_some() {
                username_text = self.add_session.username.as_ref().unwrap().to_string();
            }
            if ui.text_edit_singleline(&mut username_text).changed {
                if !username_text.is_empty() {
                    self.add_session.username = Some(username_text);
                } else {
                    self.add_session.username = None;
                }
            };
            ui.label("Authentication mode: ");
            egui::ComboBox::from_label("")
                .selected_text(self.add_session.auth_method.as_str())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.add_session.auth_method,
                        AuthModes::PasswordAuth,
                        "Password",
                    );
                    ui.selectable_value(
                        &mut self.add_session.auth_method,
                        AuthModes::KeyFileAuth,
                        "Private key",
                    );
                });
            if self.add_session.auth_method == AuthModes::PasswordAuth {
                ui.label("Password: ");
                let mut password_text = String::from("");
                if ui.text_edit_singleline(&mut password_text).changed() {
                    if !password_text.is_empty() {
                        self.add_session.password = Some(password_text);
                    } else {
                        self.add_session.password = None;
                    }
                };
            } else {
                ui.label("Key path: ");
                let mut file_path: String = String::from("");
                if self.add_session.key_path.is_some() {
                    file_path = self
                        .add_session
                        .key_path
                        .as_ref()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                }

                if ui.text_edit_singleline(&mut file_path).clicked() {
                    let files = FileDialog::new().pick_file();
                    if files.is_some() {
                        file_path = files.unwrap().to_str().unwrap().to_string();
                    }
                };
                self.add_session.key_path = Some(file_path.into());
            }
            ui.horizontal(|ui| -> anyhow::Result<()> {
                if ui.button("Add").clicked() {
                    self.app_config.configuration.sessions.push(self.add_session.clone());
                    self.app_config.save_config()?;
                    self.app_config.load_data()?;
                    self.add_session_open = false;
                }
                if ui.button("Close").clicked() {
                    self.add_session_open = !self.add_session_open;
                }
                Ok(())
            });
        });
        Ok(())
    }

    // Sessions panel
    fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("session_panel")
            .width_range(225.0..=600.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Add").clicked() {
                        self.add_session_open = true;
                        self.add_session = Session::new();
                    };
                    ui.button("Edit");
                    ui.button("Delete");
                    ui.separator();
                    ui.button("Connect");
                });
                ui.separator();
                ui.heading("Saved Sessions");
                for (index, session) in self.app_config.configuration.sessions.iter().enumerate() {
                    if ui.selectable_label(self.app_config.session_selected[index], &session.ip).clicked() {
                        self.app_config.session_selected = vec![false; self.app_config.session_selected.len()];
                        self.app_config.session_selected[index] = true;
                    };
                }
            });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> () {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.heading("rSSH-Win");
        });

        self.left_panel(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {});

        if self.add_session_open {
            self.add_session_window(ctx);
        }
    }
}
