use iced::{
    Alignment, Background, Border, Color, Element, Length, Theme,
    border::Radius,
    widget::{column, container, scrollable, text},
};

pub fn terminal(theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let palette = theme.extended_palette();
    let background = Background::Color(palette.secondary.base.color);

    let rail = scrollable::Rail {
        background: None,
        border: Border {
            ..Default::default()
        },
        scroller: scrollable::Scroller {
            border: Border {
                ..Default::default()
            },
            background,
        },
    };

    let border = Border {
        width: 0f32,
        radius: Radius::from(2),
        ..Default::default()
    };

    scrollable::Style {
        container: container::Style {
            background: Some(Background::Color(Color::from_rgb(0f32, 0f32, 0f32))),
            border,
            ..Default::default()
        },
        auto_scroll: scrollable::AutoScroll {
            background,
            border: border,
            shadow: iced::Shadow {
                color: Color::from_rgba(0f32, 0f32, 0f32, 0f32),
                offset: iced::Vector { x: 0f32, y: 0f32 },
                blur_radius: 0f32,
            },
            icon: palette.secondary.base.color,
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
