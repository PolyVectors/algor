use iced::{Border, Color, Length, border::Radius};
use iced_aw::{quad::Quad, widget::InnerBounds};

pub fn horizontal_separator() -> Quad {
    Quad {
        height: Length::Fixed(0.20),
        inner_bounds: InnerBounds::Ratio(1f32, 2f32),
        quad_color: Color::from([0.5; 4]).into(),
        quad_border: Border {
            radius: Radius::new(4),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn vertical_separator() -> Quad {
    Quad {
        width: Length::Fixed(0.10),
        inner_bounds: InnerBounds::Ratio(2f32, 1f32),
        quad_color: Color::from([0.5, 0.5, 0.5, 1f32]).into(),
        quad_border: Border {
            radius: Radius::new(4),
            ..Default::default()
        },
        ..Default::default()
    }
}
