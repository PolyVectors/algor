use std::borrow::Cow;

const FAMILY_NAME: &'static str = "JetBrainsMono Nerd Font";

pub enum Font {
    Regular,
    Bold,
}

impl Font {
    fn weight(&self) -> iced::font::Weight {
        match self {
            Font::Regular => iced::font::Weight::Medium,
            Font::Bold => iced::font::Weight::Bold,
        }
    }
}

impl From<Font> for iced::Font {
    fn from(font: Font) -> Self {
        iced::Font {
            family: iced::font::Family::Name(FAMILY_NAME),
            weight: font.weight(),
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        }
    }
}

impl From<Font> for Cow<'static, [u8]> {
    fn from(font: Font) -> Self {
        match font {
            Font::Regular => include_bytes!("../assets/fonts/JetBrainsMonoNerdFont-Regular.ttf")
                .as_slice()
                .into(),
            Font::Bold => include_bytes!("../assets/fonts/JetBrainsMonoNerdFont-Bold.ttf")
                .as_slice()
                .into(),
        }
    }
}
