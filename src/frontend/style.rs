use iced::Background;
use iced::Border;
use iced::Color;
use iced::border::Radius;
use iced::{Theme, widget::container};

#[must_use]
pub fn menu_container(theme: &Theme) -> container::Style {
    let primary = theme.palette().primary;

    container::Style {
        background: Some(Background::Color(Color::from_rgba(
            primary.r, primary.g, primary.b, 0.25f32,
        ))),
        border: Border {
            width: 0f32,
            radius: Radius::new(2).bottom_left(0).bottom_right(0),
            ..Default::default()
        },
        ..Default::default()
    }
}
