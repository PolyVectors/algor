use std::fmt::Display;

use serde::Deserialize;

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub const ALL: &'static [Theme] = &[Theme::Light, Theme::Dark];
}

impl From<Theme> for iced::Theme {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => iced::Theme::Light,
            Theme::Dark => iced::Theme::Dark,
        }
    }
}

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

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Light => write!(f, "Light"),
            Theme::Dark => write!(f, "Dark"),
        }
    }
}
