use iced::{
    Alignment, Element, Length,
    widget::{column, container, scrollable, text},
};

use crate::frontend::pane::style;

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
                        .map(|output| { text(&**output).style(style::terminal_out).into() })
                ),
                // Show errors below output
                text(error).style(style::terminal_err)
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
