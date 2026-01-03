use iced::Background;
use iced::Border;
use iced::Color;
use iced::border::Radius;
use iced::widget::scrollable;
use iced::widget::text;
use iced::widget::text_input;
use iced::{Theme, widget::container};

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

pub fn terminal(theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let palette = theme.extended_palette();

    let rail = scrollable::Rail {
        background: None,
        border: Border {
            ..Default::default()
        },
        scroller: scrollable::Scroller {
            border: Border {
                ..Default::default()
            },
            color: palette.secondary.base.color,
        },
    };

    scrollable::Style {
        container: container::Style {
            background: Some(Background::Color(Color::from_rgb(0f32, 0f32, 0f32))),
            border: Border {
                width: 0f32,
                radius: Radius::from(2),
                ..Default::default()
            },
            ..Default::default()
        },
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: None,
    }
}

pub fn terminal_out(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(Color::from_rgb(255f32, 255f32, 255f32)),
    }
}

pub fn terminal_err(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(Color::from_rgb(255f32, 0f32, 0f32)),
    }
}
