pub mod editor;
pub mod state_viewer;
pub mod terminal;

pub mod style {
    use iced::{
        Background, Border, Color, Theme, border::Radius, widget::container, widget::scrollable,
    };

    // Default container with a background and slight rounding on the top two corners
    pub fn title_bar_focused(theme: &Theme) -> container::Style {
        container::Style {
            // More saturated version of the primary theme colour
            background: Some(Background::Color(
                theme.extended_palette().primary.strong.color,
            )),

            text_color: Some(Color::from_rgb(1f32, 1f32, 1f32)),
            border: Border {
                radius: Radius::new(2).bottom_left(0).bottom_right(0),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    // Ditto title_bar_focused comment but with dull colours, use black text on grey for light themes and white text on grey for dark themes
    pub fn title_bar_unfocused(theme: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(
                theme.extended_palette().secondary.base.color,
            )),
            text_color: if theme == &Theme::Light {
                Some(Color::from_rgb(0f32, 0f32, 0f32))
            } else {
                Some(Color::from_rgb(1f32, 1f32, 1f32))
            },
            border: Border {
                radius: Radius::new(2).bottom_left(0).bottom_right(0),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    // Default container with a saturated round border
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

    // Ditto grid_pane_focused comment but with a dull colour
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

    pub fn terminal(theme: &Theme, _status: scrollable::Status) -> scrollable::Style {
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

    pub fn background_scrollable(theme: &Theme, _status: scrollable::Status) -> scrollable::Style {
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
                background: Some(Background::Color(theme.palette().background)),
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
}
