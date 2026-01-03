pub mod editor;
pub mod terminal;
pub mod viewer;

pub mod style {
    use iced::{Background, Border, Color, Theme, border::Radius, widget::container};

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
}
