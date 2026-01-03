use iced::widget::{button, column, container, row, text, text::Shaping};
use iced::{Alignment, Element, Length, alignment};

use crate::frontend::screen::{Screen, sandbox, settings};
use crate::frontend::util::style;

#[derive(Debug, Clone)]
pub enum Message {
    SandboxButtonClicked,
    LessonsButtonClicked,
    SettingsButtonClicked,
}

pub enum Event {
    ToLessonView,
    ToSandbox,
    ToSettings,
}

#[derive(Debug, Clone)]
pub struct State;

impl State {
    pub fn update(&self, message: Message) -> Option<Event> {
        match message {
            Message::SandboxButtonClicked => Some(Event::ToSandbox),
            Message::LessonsButtonClicked => Some(Event::ToLessonView),
            Message::SettingsButtonClicked => Some(Event::ToSettings),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        column![
            container(
                row![
                    column![
                        container(text("📘").shaping(Shaping::Advanced).size(96))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .style(style::menu_container),
                        button("Lessons")
                            .width(Length::Fill)
                            .on_press(Message::LessonsButtonClicked)
                    ],
                    column![
                        container(text("🛠️").shaping(Shaping::Advanced).size(96))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .style(style::menu_container),
                        button("Sandbox")
                            .width(Length::Fill)
                            .on_press(Message::SandboxButtonClicked)
                    ]
                ]
                .spacing(32),
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .padding(128),
            container(button("Settings").on_press(Message::SettingsButtonClicked))
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Right)
                .padding(12)
        ]
        .into()
    }
}
