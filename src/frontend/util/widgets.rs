pub mod separator {
    use iced::{Color, Length};
    use iced_aw::{quad::Quad, widget::InnerBounds};

    #[must_use]
    pub fn horizontal() -> Quad {
        Quad {
            width: Length::Fill,
            height: Length::Fixed(2f32),
            inner_bounds: InnerBounds::Ratio(1f32, 1f32),
            quad_color: Color::from([0.5; 4]).into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn vertical() -> Quad {
        Quad {
            width: Length::Fixed(2f32),
            height: Length::Fill,
            inner_bounds: InnerBounds::Ratio(1f32, 1f32),
            quad_color: Color::from([0.5; 4]).into(),
            ..Default::default()
        }
    }
}
