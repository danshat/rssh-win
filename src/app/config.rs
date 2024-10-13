use bincode::de;
use directories::ProjectDirs;
use eframe::App;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read};
use std::path::PathBuf;

// For now, only private key auth and password auth implemented
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
pub enum AuthModes {
    #[default]
    PasswordAuth,
    KeyFileAuth,
}
// as_str implementation to map enum to strings for Combobox
impl AuthModes {
    pub fn as_str(&self) -> &str {
        match self {
            AuthModes::KeyFileAuth => "Private key",
            AuthModes::PasswordAuth => "Password",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub name: String,
    pub ip: String,
    pub port: i32,
    pub username: Option<String>,
    pub auth_method: AuthModes,
    pub key_path: Option<PathBuf>,
    pub password: Option<String>,
}

impl Session {
    pub fn new() -> Self {
        Session {
            name: String::from(""),
            ip: String::from(""),
            port: 22,
            username: None,
            auth_method: AuthModes::PasswordAuth,
            key_path: None,
            password: None,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub sessions: Vec<Session>,
}
impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            sessions: Vec::new(),
        }
    }
}

pub struct AppConfiguration {
    directories: ProjectDirs,
    pub configuration: Configuration,
    pub session_selected: Vec<bool>,
}

impl AppConfiguration {
    pub fn new() -> anyhow::Result<AppConfiguration> {
        let mut app_conf = AppConfiguration {
            directories: ProjectDirs::from("com", "danshat", "rssh-win")
                .expect("No valid home path detected."),
            configuration: Configuration::new(),
            session_selected: Vec::new(),
        };
        app_conf.new_config_dir();
        app_conf.create_files();
        app_conf.load_data()?;
        Ok(app_conf)
    }
}

impl AppConfiguration {
    pub fn save_config(&self) -> anyhow::Result<()> {
        let file_path = self.directories.data_dir().join("sessions.rssh");
        let serialized = bincode::serialize(&self.configuration)?;
        Ok(fs::write(file_path, serialized)?)
    }
    fn create_files(&self) {
        OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(self.directories.data_dir().join("sessions.rssh"))
            .expect("Failed to create sessions file!");
    }
    pub fn load_data(&mut self) -> anyhow::Result<()> {
        let file_path = self.directories.data_dir().join("sessions.rssh");
        let mut data: Vec<u8> = Vec::new();
        let mut f = File::open(file_path)?;
        f.read_to_end(&mut data)?;
        let deserialization_result = bincode::deserialize::<Vec<Session>>(&data);
        if deserialization_result.is_err() {
            return Ok(());
        }
        self.configuration.sessions = deserialization_result.unwrap();
        self.session_selected = vec![false; self.configuration.sessions.len()];
        Ok(())
    }
    fn new_config_dir(&self) {
        let create_dir_result = fs::create_dir_all(self.directories.data_dir());
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
}
