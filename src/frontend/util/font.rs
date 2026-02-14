use std::borrow::Cow;

// Name of font family in assets/fonts/*
pub const FAMILY_NAME: &str = "JetBrainsMonoNL Nerd Font";

// Font variants used across the application
pub enum Font {
    Regular,
    Bold,
    Italic,
}

// Convert font type into iced weight
impl Font {
    fn weight(&self) -> iced::font::Weight {
        match self {
            Font::Regular => iced::font::Weight::Medium,
            Font::Bold => iced::font::Weight::Bold,
            Font::Italic => iced::font::Weight::Normal,
        }
    }
}

// Convert Font into iced font
impl From<Font> for iced::Font {
    fn from(font: Font) -> Self {
        let style = if let Font::Italic = font {
            iced::font::Style::Italic
        } else {
            iced::font::Style::Normal
        };

        Self {
            family: iced::font::Family::Name(FAMILY_NAME),
            weight: font.weight(),
            stretch: iced::font::Stretch::Normal,
            style,
        }
    }
}

// Convert Font enum into a list of bytes on the heap with multiple read-only references (Cow<'static, [u8]>)
impl From<Font> for Cow<'static, [u8]> {
    fn from(font: Font) -> Self {
        match font {
            Font::Regular => {
                include_bytes!("../../../assets/fonts/JetBrainsMonoNerdFont-Regular.ttf")
                    .as_slice()
                    .into()
            }
            Font::Bold => include_bytes!("../../../assets/fonts/JetBrainsMonoNerdFont-Bold.ttf")
                .as_slice()
                .into(),
            Font::Italic => {
                include_bytes!("../../../assets/fonts/JetBrainsMonoNerdFont-Italic.ttf")
                    .as_slice()
                    .into()
            }
        }
    }
}
