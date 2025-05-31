use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::frontend::theme::Theme;
use std::{env, fs, path::PathBuf};

#[cfg(target_os = "linux")]
pub const CONFIG_DIR: &'static str = ".config/algor/config.toml";
#[cfg(target_os = "windows")]
pub const CONFIG_DIR: &'static str = "AppData\\Roaming\\algor\\config.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub theme: Theme,
    pub editor_font_size: u8,
    pub lessons_directory: String,
}

impl Config {
    pub async fn save(self, path: PathBuf) -> Result<(), &'static str> {
        let mut file = File::create(&path).await.unwrap();

        file.write_all(toml::to_string(&self).unwrap().as_bytes())
            .await
            .unwrap();

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut documents = env::home_dir().unwrap();
        documents.push("Documents");
        documents.push("algor");

        let mut config = env::home_dir().unwrap();
        config.push(CONFIG_DIR);

        Self {
            theme: Theme::try_from(iced::Theme::Light).unwrap(),
            editor_font_size: 16,
            lessons_directory: documents.to_str().to_owned().unwrap().to_owned(),
        }
    }
}

impl TryFrom<PathBuf> for Config {
    type Error = &'static str;

    fn try_from(path_buf: PathBuf) -> Result<Self, Self::Error> {
        let config: Config =
            toml::from_str(fs::read_to_string(path_buf).unwrap().as_str()).unwrap();

        Ok(Self {
            theme: config.theme,
            editor_font_size: config.editor_font_size,
            lessons_directory: config.lessons_directory,
        })
    }
}
