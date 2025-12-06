// TODO: all of this is complete ass, fix

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::frontend::utils::theme::Theme;

use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    str::FromStr,
};

#[cfg(any(
    target_os = "linux",
    target_os = "openbsd",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "dragonfly"
))]
pub const CONFIG_PATH: &str = ".config/algor/config.toml";

#[cfg(target_os = "windows")]
pub const CONFIG_PATH: &str = "AppData\\Roaming\\algor\\config.toml";

#[derive(Clone, Debug, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
pub enum RunSpeed {
    Slow,
    #[default]
    Medium,
    Fast,
    Instant,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub theme: Theme,
    pub editor_font_size: u8,
    pub lessons_directory: String,
    pub run_speed: RunSpeed,
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

        if !algor_dir.exists()
            && let Err(e) = fs::create_dir_all(&algor_dir)
        {
            panic!("Failed to create default lessons directory, {e}");
        }

        let Some(lessons_directory) = algor_dir.to_str() else {
            panic!("Failed to convert path to string, path possibly contains invalid unicode")
        };

        Self {
            theme: Theme::Light,
            editor_font_size: 16,
            lessons_directory: lessons_directory.to_string(),
            run_speed: RunSpeed::Medium,
        }
    }
}

impl TryFrom<PathBuf> for Config {
    type Error = &'static str;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let mut config_dir = path.clone();
        config_dir.pop();

        if !config_dir.exists()
            && let Err(e) = fs::create_dir_all(&config_dir)
        {
            panic!("Failed to create config directory, {e}");
        }

        if !path.exists() {
            let Ok(mut file) = fs::File::create(&path) else {
                panic!("Failed to create config file")
            };

            if let Ok(config) = toml::to_string(&Config::default())
                && let Err(e) = file.write_all(config.as_bytes())
            {
                panic!("{}", e);
            }
        }

        let Ok(file) = fs::read_to_string(&path) else {
            panic!("Failed to read config to buffer")
        };

        let Ok(config) = toml::from_str::<Config>(file.as_str()) else {
            panic!("Failed to read config, check for syntax errors")
        };

        let Ok(lessons_path) = PathBuf::from_str(&config.lessons_directory);

        if !lessons_path.exists()
            && let Err(e) = fs::create_dir_all(&config.lessons_directory)
        {
            panic!("Failed to create lessons directory, {e}");
        }

        Ok(Self {
            theme: config.theme,
            editor_font_size: config.editor_font_size,
            lessons_directory: config.lessons_directory,
            run_speed: config.run_speed,
        })
    }
}
