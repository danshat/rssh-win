use crate::app::egui::{Context, Ui};
use directories::{BaseDirs, ProjectDirs, UserDirs};
use eframe::App;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::{env, fs, path::Path};

pub struct AppConfiguration {
    directories: ProjectDirs,
}

impl AppConfiguration {
    pub fn new() -> Self {
        let app_conf = AppConfiguration {
            directories: ProjectDirs::from("com", "danshat", "rssh-win")
                .expect("No valid home path detected."),
        };
        app_conf.new_config_dir(&app_conf.directories);
        app_conf.create_files(&app_conf.directories);
        app_conf
    }
}

impl AppConfiguration {
    fn create_files(&self, dirs: &ProjectDirs) {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(dirs.data_dir().join("sessions.rssh"))
            .expect("Failed to create sessions file!");
    }

    fn new_config_dir(&self, dirs: &ProjectDirs) {
        let create_dir_result = fs::create_dir_all(dirs.data_dir());
        match create_dir_result {
            Ok(_) => {}
            Err(error) => {
                match error.kind() {
                    ErrorKind::AlreadyExists => {
                        println!("Config folder exists!");
                    }
                    ErrorKind::PermissionDenied => {
                        // Not enough permissions
                        panic!("Not enough permissions to create configuration directory!");
                    }
                    _ => {
                        panic!("Something unexpected went wrong while creating configuration directory!");
                    }
                }
            }
        }
    }

    // ~/.local/share/rssh-win
}
fn add_new_session(ctx: &Context, ui: &mut Ui) {}
