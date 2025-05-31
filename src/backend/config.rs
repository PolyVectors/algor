use serde::Deserialize;

use crate::frontend::theme::Theme;
use std::{env, path::PathBuf};

#[cfg(target_os = "linux")]
pub const CONFIG_DIR: &'static str = ".config/algor/config.toml";
#[cfg(target_os = "windows")]
pub const CONFIG_DIR: &'static str = "AppData\\Roaming\\algor\\config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub theme: Theme,
    pub editor_font_size: u8,
    pub lessons_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut documents = env::home_dir().unwrap();
        documents.push("Documents");
        documents.push("algor");

        Self {
            theme: Theme::try_from(iced::Theme::Light).unwrap(),
            editor_font_size: 16,
            lessons_directory: documents.to_str().to_owned().unwrap().to_owned(),
        }
    }
}

impl TryFrom<PathBuf> for Config {
    type Error = &'static str;

    fn try_from(_path_buf: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            ..Default::default()
        })
    }
}
