use crate::frontend::handlers::messages::Message;
use crate::frontend::{app::Algor, app::Screen};
use iced::advanced::text::Shaping;
use iced::{
    Alignment, Element, Length, alignment,
    widget::{button, column, container, row, text},
};

use iced::Background;
use iced::Border;
use iced::Color;
use iced::Theme;
use iced::border::Radius;

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

impl Algor {
    pub fn menu(&self) -> Element<'_, Message> {
        column![
            container(
                row![
                    column![
                        container(text("📘").shaping(Shaping::Advanced).size(96))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .style(menu_container),
                        button("Lessons")
                            .width(Length::Fill)
                            .on_press(Message::SetScreen(Screen::LessonSelect))
                    ],
                    column![
                        container(text("🛠️").shaping(Shaping::Advanced).size(96))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .style(menu_container),
                        button("Sandbox")
                            .width(Length::Fill)
                            .on_press(Message::SetScreen(Screen::Sandbox))
                    ]
                ]
                .spacing(32),
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .padding(128),
            container(button("Settings").on_press(Message::SetScreen(Screen::Settings)))
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Right)
                .padding(12)
        ]
        .into()
    }
}
