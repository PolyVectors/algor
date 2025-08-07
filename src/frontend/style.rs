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

pub fn terminal(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0f32, 0f32, 0f32))),
        ..Default::default()
    }
}

pub fn terminal_text(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(Color::from_rgb(255f32, 255f32, 255f32)),
    }
}

pub fn title_bar_focused(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(
            theme.extended_palette().primary.strong.color,
        )),

        text_color: Some(Color::from_rgb(255f32, 255f32, 255f32)),
        border: Border {
            radius: Radius::new(2).bottom_left(0).bottom_right(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn title_bar_unfocused(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(
            theme.extended_palette().secondary.base.color,
        )),
        text_color: if theme == &Theme::Light {
            Some(Color::from_rgb(0f32, 0f32, 0f32))
        } else {
            Some(Color::from_rgb(255f32, 255f32, 255f32))
        },
        border: Border {
            radius: Radius::new(2).bottom_left(0).bottom_right(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn grid_pane_focused(theme: &Theme) -> container::Style {
    container::Style {
        border: Border {
            color: theme.extended_palette().primary.strong.color,
            width: 2f32,
            radius: Radius::new(2),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn grid_pane_unfocused(theme: &Theme) -> container::Style {
    container::Style {
        border: Border {
            color: theme.extended_palette().secondary.base.color,
            width: 2f32,
            radius: Radius::new(2),
            ..Default::default()
        },
        ..Default::default()
    }
}
