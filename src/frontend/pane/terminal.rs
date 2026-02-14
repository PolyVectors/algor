use iced::{
    Alignment, Color, Element, Length, Theme,
    widget::{column, container, scrollable, text},
};

use crate::frontend::pane::style;

// White text
pub fn terminal_out(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(Color::from_rgb(1f32, 1f32, 1f32)),
    }
}

// Red text
pub fn terminal_err(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(Color::from_rgb(1f32, 0f32, 0f32)),
    }
}

// No messages required but provide a mapping for future maintainability
#[derive(Debug, Clone)]
pub enum Message {}

pub fn terminal<'a>(output: &'a Vec<Box<str>>, error: &'a String) -> Element<'a, Message> {
    container(
        scrollable(
            column![
                column(
                    // For every output display text in pure white
                    output
                        .iter()
                        /* Dereference output (i.e. get Box<str>)
                         Dereference again to unbox (i.e. get str)
                         And take a reference again as the size of str by itself is unknown at compile time (i.e. get &str) */
                        .map(|output| { text(&**output).style(terminal_out).into() })
                ),
                // Show errors below output
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
