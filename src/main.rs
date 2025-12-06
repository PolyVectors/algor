// TODO: move stuff currently in frontend to something like frontend/utils and split up this file into components inside the frontend folder, think about how to deal with subscriptions, messages, and events later
use algor::frontend::app::Algor;
use algor::frontend::utils::font;
use iced::Settings;
use iced_aw::iced_fonts;

fn main() -> iced::Result {
    iced::application("algor", Algor::update, Algor::view)
        .settings(Settings {
            fonts: vec![font::Font::Regular.into(), font::Font::Bold.into()],
            default_font: iced::Font::with_name(font::FAMILY_NAME),
            ..Settings::default()
        })
        .subscription(Algor::subscription)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .theme(Algor::iced_theme)
        .run_with(Algor::new)
}
