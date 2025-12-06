use iced::Background;
use iced::Border;
use iced::Color;
use iced::border::Radius;
use iced::widget::scrollable;
use iced::widget::text;
use iced::{Theme, widget::container};

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
