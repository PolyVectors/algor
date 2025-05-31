use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::frontend::theme::Theme;
use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

#[cfg(target_os = "linux")]
pub const CONFIG_PATH: &'static str = ".config/algor/config.toml";
#[cfg(target_os = "windows")]
pub const CONFIG_PATH: &'static str = "AppData\\Roaming\\algor\\config.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub theme: Theme,
    pub editor_font_size: u8,
    pub lessons_directory: String,
}

impl Config {
    pub async fn save(self, path: PathBuf) -> Result<(), io::Error> {
        let Ok(mut file) = File::create(&path).await else {
            panic!("Failed to create config file")
        };

        let config = match toml::to_string(&self) {
            Ok(config) => config,
            Err(_error) => match toml::to_string(&Config::default()) {
                Ok(config) => config,
                Err(_e) => unreachable!(),
            },
        };

        file.write_all(config.as_bytes()).await?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut algor_dir = match env::home_dir().ok_or(std::io::Error::new(
            io::ErrorKind::NotFound,
            "No home directory",
        )) {
            Ok(algor_dir) => algor_dir,
            Err(e) => panic!("{}", e),
        };

        algor_dir.push("Documents");
        algor_dir.push("algor");

        if !algor_dir.exists() {
            if let Err(e) = fs::create_dir_all(&algor_dir) {
                panic!("Failed to create default lessons directory, {e}");
            }
        }

        Self {
            theme: if let Ok(theme) = Theme::try_from(iced::Theme::Light) {
                theme
            } else {
                panic!("Default theme is unsupported")
            },
            editor_font_size: 16,
            lessons_directory: if let Some(algor_dir) = algor_dir.to_str() {
                algor_dir.to_string()
            } else {
                panic!("Failed to convert path to string, path possibly contains invalid unicode")
            },
        }
    }
}

impl TryFrom<PathBuf> for Config {
    type Error = &'static str;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let mut config_dir = path.clone();
        config_dir.pop();

        if !config_dir.exists() {
            if let Err(e) = fs::create_dir_all(&config_dir) {
                panic!("Failed to create config directory, {e}");
            }
        }

        if !path.exists() {
            let Ok(mut file) = fs::File::create(&path) else {
                panic!("Failed to create config file")
            };

            if let Ok(config) = toml::to_string(&Config::default()) {
                if let Err(e) = file.write_all(config.as_bytes()) {
                    panic!("{}", e);
                }
            }
        }

        let Ok(file) = fs::read_to_string(&path) else {
            unreachable!()
        };

        let config: Config = if let Ok(config) = toml::from_str(file.as_str()) {
            config
        } else {
            panic!("Failed to read config, check for syntax errors")
        };

        Ok(Self {
            theme: config.theme,
            editor_font_size: config.editor_font_size,
            lessons_directory: config.lessons_directory,
        })
    }
}
