use std::fmt::Display;

use serde::{Deserialize, Serialize};

// Similar to iced theme but restricted to only light and dark themes for now, also implement the ability for (de)serialisation for config files
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum Theme {
    Light,
    Dark,
}

// Represents a list of themes (used in drop-down menus)
impl Theme {
    pub const ALL: &'static [Theme] = &[Theme::Light, Theme::Dark];
}

// Convert Theme to iced theme
impl From<Theme> for iced::Theme {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => iced::Theme::Light,
            Theme::Dark => iced::Theme::Dark,
        }
    }
}

// Convert iced theme into Theme if the theme is supported
impl TryFrom<iced::Theme> for Theme {
    type Error = &'static str;

    fn try_from(iced_theme: iced::Theme) -> Result<Self, Self::Error> {
        match iced_theme {
            iced::Theme::Light => Ok(Theme::Light),
            iced::Theme::Dark => Ok(Theme::Dark),
            _ => Err("Unsupported theme"),
        }
    }
}

// Allow formatting and printing out of strings (also for drop-down menus)
impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Light => write!(f, "Light"),
            Theme::Dark => write!(f, "Dark"),
        }
    }
}
