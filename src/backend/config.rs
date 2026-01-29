use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::frontend::{screen::settings, util::theme::Theme};

use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    str::FromStr,
};

// Config path for unix-like operating systems, appended to home directory
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

/* Implement Deserialize and Serialize for saving and loading from files
Implement Default for selecting the default run speed if no config file exists
Implement PartialEq and Eq for comparing against other run speeds */
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
pub enum RunSpeed {
    Slow,
    #[default]
    Medium,
    Fast,
    Instant,
}

impl From<RunSpeed> for Duration {
    fn from(value: RunSpeed) -> Self {
        match value {
            RunSpeed::Slow => Duration::from_millis(1000),
            RunSpeed::Medium => Duration::from_millis(250),
            RunSpeed::Fast => Duration::from_millis(100),
            RunSpeed::Instant => Duration::from_millis(0),
        }
    }
}

// Facilitate deserialising (turning into struct from a file) and serialising (turning the struct into a file), this is required by the toml library
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub theme: Theme,
    pub editor_font_size: u32,
    pub lessons_directory: String,
    pub run_speed: RunSpeed,
}

impl From<&mut settings::State> for Config {
    fn from(value: &mut settings::State) -> Self {
        Self {
            theme: value.theme.clone(),
            editor_font_size: value.editor_font_size,
            lessons_directory: value.lessons_directory.clone(),
            run_speed: value.run_speed.unwrap_or_default(),
        }
    }
}

impl From<settings::State> for Config {
    fn from(value: settings::State) -> Self {
        Self {
            theme: value.theme,
            editor_font_size: value.editor_font_size,
            lessons_directory: value.lessons_directory,
            run_speed: value.run_speed.unwrap_or_default(),
        }
    }
}

impl Config {
    // Let the user save their config file to disk
    pub async fn save(self, path: PathBuf) -> Result<(), io::Error> {
        let Ok(mut file) = File::create(&path).await else {
            panic!("Failed to create config file")
        };

        // Use the toml library to turn the config into a string
        let config = match toml::to_string(&self) {
            Ok(config) => config,
            Err(_error) => match toml::to_string(&Config::default()) {
                Ok(config) => config,
                Err(_e) => unreachable!(),
            },
        };

        // Write string to disk, return an error otherwise
        file.write_all(config.as_bytes()).await?;
        Ok(())
    }
}

// Implement the ability to create a default config (done in the frontend)
impl Default for Config {
    fn default() -> Self {
        // If the user doesn't have a home directory (e.g. "~") on Linux, then there is a serious problem and the program cannot continue
        let mut algor_dir = match env::home_dir().ok_or(std::io::Error::new(
            io::ErrorKind::NotFound,
            "No home directory",
        )) {
            Ok(algor_dir) => algor_dir,
            Err(e) => panic!("{}", e),
        };

        // Add Documents/algor to the user's home directory (i.e. "~/Documents/algor")
        algor_dir.push("Documents");
        algor_dir.push("algor");

        // Create the default lessons directory if it doesn't exist
        if !algor_dir.exists()
            && let Err(e) = fs::create_dir_all(&algor_dir)
        {
            // If the program lacks permissions or the computer lacks storage, etc. exit the program with error information
            panic!("Failed to create default lessons directory, {e}");
        }

        // Exit the program if the directory cannot be converted into a string
        let Some(lessons_directory) = algor_dir.to_str() else {
            panic!("Failed to convert path to string, path possibly contains invalid unicode")
        };

        // Create and return a default config (light theme by default as this tends to be more inviting to new users)
        Self {
            theme: Theme::Light,
            editor_font_size: 16,
            lessons_directory: lessons_directory.to_string(),
            run_speed: RunSpeed::Medium,
        }
    }
}

// Allow turning a path into a config (deserialisation), not using TryFrom here as this operation must succeed or the program will not work
impl TryFrom<PathBuf> for Config {
    type Error = ();

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        // Get config directory from path
        let mut config_dir = path.clone();
        config_dir.pop();

        // Create all directories leading up to config path, if the program can't exit early
        if !config_dir.exists()
            && let Err(e) = fs::create_dir_all(&config_dir)
        {
            panic!("Failed to create config directory, {e}");
        }

        // Create the config file if it doesn't exist, if any errors occur while creating the file or the default config exit the program
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

        // Read the file from disk or exit
        let Ok(file) = fs::read_to_string(&path) else {
            panic!("Failed to read config to buffer")
        };

        // Turn the config file into a struct, otherwise the user must've edited the file manually (i.e. knows what they are doing), tell them to check for syntax errors
        let Ok(config) = toml::from_str::<Config>(file.as_str()) else {
            panic!("Failed to read config, check for syntax errors")
        };

        // Get the lesson path from the config and make it a string, this error type is Infallible, i.e. it will never fail, so no need to match for errors
        let Ok(lessons_path) = PathBuf::from_str(&config.lessons_directory);

        // Make the lesson path if it doesn't exist, if the program can't, exit.
        if !lessons_path.exists()
            && let Err(e) = fs::create_dir_all(&config.lessons_directory)
        {
            panic!("Failed to create lessons directory, {e}");
        }

        Ok(config)
    }
}
