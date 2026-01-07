use iced::border::Radius;
use iced::widget::{button, column, container, row, text, text::Shaping};
use iced::{Alignment, Background, Border, Color, Element, Length, Theme, alignment};

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
                            .style(menu_container),
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
                            .style(menu_container),
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
