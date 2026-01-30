use iced::{
    Alignment, Color, Element, Length, Theme,
    widget::{column, container, scrollable, text},
};

use crate::frontend::pane::style;

pub fn terminal_out(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(Color::from_rgb(1f32, 1f32, 1f32)),
    }
}

pub fn terminal_err(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(Color::from_rgb(1f32, 0f32, 0f32)),
    }
}

#[derive(Debug, Clone)]
pub enum Message {}

pub fn terminal<'a>(output: &'a Vec<Box<str>>, error: &'a String) -> Element<'a, Message> {
    container(
        scrollable(
            column![
                column(
                    output
                        .iter()
                        .map(|output| { text(&**output).style(terminal_out).into() })
                ),
                text(error).style(terminal_err)
            ]
            .padding(6)
            .spacing(16),
        )
        .style(style::terminal)
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .padding(2)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center)
    .into()
}
