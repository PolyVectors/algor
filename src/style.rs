use iced::Background;
use iced::Border;
use iced::Color;
use iced::border::Radius;
use iced::{Theme, widget::container::Style};

pub fn menu_container(theme: &Theme) -> Style {
    let primary = theme.palette().primary;

    Style {
        background: Some(Background::Color(Color::from_rgba(
            primary.r, primary.g, primary.b, 0.25f32,
        ))),
        border: Border {
            color: Color::default(),
            width: 0f32,
            radius: Radius::new(2).bottom_left(0).bottom_right(0),
        },
        ..Style::default()
    }
}
