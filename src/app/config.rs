use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::fs;

// For now, only private key auth and password auth implemented
#[derive(PartialEq, Default, Serialize, Deserialize, Debug)]
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


#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub ip: String,
    pub port: i32,
    pub username: Option<String>,
    pub auth_method: AuthModes,
    pub key_path: Option<PathBuf>,
    pub password: Option<String>,
}



impl Session {
    pub fn write_config(&mut self, config: &AppConfiguration) -> anyhow::Result<()> {
        let file_path = config.directories.data_dir().join("sessions.rssh");
        let serialized = bincode::serialize(&self)?;
        Ok(fs::write(file_path, serialized)?)
    }
    pub fn new() -> Self {
        Session {
            ip: String::from(""),
            port: 22,
            username: None,
            auth_method: AuthModes::PasswordAuth,
            key_path: None,
            password: None,
        }
    }
}

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
        OpenOptions::new()
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
}

