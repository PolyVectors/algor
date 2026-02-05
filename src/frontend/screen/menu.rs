use iced::border::Radius;
use iced::widget::{button, column, container, row, text, text::Shaping};
use iced::{Alignment, Background, Border, Color, Element, Length, Theme, alignment};

// The messages specific to the main menu screen
#[derive(Debug, Clone)]
pub enum Message {
    SandboxButtonClicked,
    LessonsButtonClicked,
    SettingsButtonClicked,
}

// What can be returned from the update method (used by src/frontend/screen.rs)
pub enum Event {
    ToLessonSelect,
    ToSandbox,
    ToSettings,
}

// The menu has no state it needs to store but does have functions so I create a unit struct (a Zero-Sized Type) which takes up 0 bytes at runtime
#[derive(Debug, Clone)]
pub struct State;

// A style for the menu containers (big indicators above buttons), slightly transparent using the primary theme colours and rounded corners only on the top edges (to leave no ugly pixel gaps as these containers are right up against the buttons)
pub fn menu_container(theme: &Theme) -> container::Style {
    // Get primary colour from theme
    let primary = theme.palette().primary;

    // Create a Style struct allowing for changing the appearance of a container
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
    // Update function (simil;ar to one in main, used by src/frontend/screen.rs)
    pub fn update(&self, message: Message) -> Option<Event> {
        match message {
            // When the user clicks the sandbox button, return an event to go to the sandbox menu
            Message::SandboxButtonClicked => Some(Event::ToSandbox),
            // Ditto for lesson button and lesson select screen
            Message::LessonsButtonClicked => Some(Event::ToLessonSelect),
            // Ditto for settings button and screen
            Message::SettingsButtonClicked => Some(Event::ToSettings),
        }
    }

    // Ditto update comment but for view instead
    pub fn view<'a>(&self) -> Element<'a, Message> {
        column![
            container(
                row![
                    column![
                        // Advanced shaping used to display emojis possible
                        container(text("📘").shaping(Shaping::Advanced).size(96))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            // Put text in center
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Center)
                            .style(menu_container),
                        button("Lessons")
                            .width(Length::Fill)
                            .on_press(Message::LessonsButtonClicked)
                    ],
                    column![
                        // Ditto advanced shaping comment
                        container(text("🛠️").shaping(Shaping::Advanced).size(96))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            // Put text in center
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
            // Put both columns in center
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            // Wide gap
            .padding(128),
            // Settings button in bottom right
            container(button("Settings").on_press(Message::SettingsButtonClicked))
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Right)
                .padding(12)
        ]
        // Turn Column struct into Element struct
        .into()
    }
}
