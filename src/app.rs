use crate::app::egui::Context;
use crate::app::egui::Order;
use eframe::egui;
use eframe::egui::Ui;

mod config;

// For now, only private key auth and password auth implemented
#[derive(PartialEq, Default)]
enum AuthModes {
    #[default]
    PasswordAuth,
    KeyFileAuth,
}
// as_str implementation to map enum to strings for Combobox
impl AuthModes {
    fn as_str(&self) -> &str {
        match self {
            AuthModes::KeyFileAuth => "Private key",
            AuthModes::PasswordAuth => "Password",
        }
    }
}

pub struct MyApp {
    config: config::AppConfiguration,
    add_session_open: bool,
    add_session_ip: String,
    add_session_port: String,
    add_session_username: String,
    add_session_auth_mode: AuthModes,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            config: config::AppConfiguration::new(),
            add_session_open: false,
            add_session_ip: String::from(""),
            add_session_port: String::from("22"),
            add_session_username: String::from(""),
            add_session_auth_mode: AuthModes::PasswordAuth,
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
    // New session popup window
    fn add_session_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Add new session").show(ctx, |ui| {
            ui.label("Host IP: ");
            ui.text_edit_singleline(&mut self.add_session_ip);
            ui.label("Port: ");
            ui.text_edit_singleline(&mut self.add_session_port);
            ui.label("Username: ");
            ui.text_edit_singleline(&mut self.add_session_username);
            ui.label("Authentication mode: ");
            egui::ComboBox::from_label("")
                .selected_text(self.add_session_auth_mode.as_str())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.add_session_auth_mode,
                        AuthModes::PasswordAuth,
                        "Password",
                    );
                    ui.selectable_value(
                        &mut self.add_session_auth_mode,
                        AuthModes::KeyFileAuth,
                        "Private key",
                    );
                });
            ui.horizontal(|ui| {
                if ui.button("Add").clicked() {}
                if ui.button("Close").clicked() {
                    self.add_session_open = !self.add_session_open;
                    self.add_session_ip = String::new();
                    self.add_session_port = String::from("22");
                }
            });
        });
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
                    };
                    ui.button("Edit");
                    ui.button("Delete");
                    ui.separator();
                    ui.button("Connect");
                });
                ui.separator();
                ui.heading("Saved Sessions");
            });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
