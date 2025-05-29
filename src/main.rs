use iced::{
    Alignment, Background, Color, Element, Font, Length, Settings, Task, Theme,
    advanced::text::Shaping,
    widget::{button, column, container, container::Style, row, text},
};

fn main() -> iced::Result {
    iced::application("Algor", Algor::update, Algor::view)
        .settings(Settings {
            fonts: vec![
                include_bytes!("../assets/fonts/JetBrainsMonoNerdFont-Regular.ttf")
                    .as_slice()
                    .into(),
            ],
            default_font: Font::with_name("JetBrainsMono Nerd Font"),
            ..Settings::default()
        })
        .run_with(Algor::new)
}

#[derive(Default)]
struct Algor {
    screen: Screen,
}

#[derive(Clone, Debug)]
enum Message {
    SetScreen(Screen),
}

#[derive(Clone, Debug)]
enum Screen {
    Menu,
    LessonSelect,
    LessonView,
    Sandbox,
    Settings,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Menu
    }
}

impl Algor {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn view(&self) -> Element<'_, Message> {
        match self.screen {
            Screen::Menu => self.menu(),
            _ => todo!(),
        }
    }

    fn container(theme: &Theme) -> Style {
        let primary = theme.palette().primary;

        Style {
            background: Some(Background::Color(Color::from_rgba(
                primary.r, primary.g, primary.b, 0.25f32,
            ))),
            ..Style::default()
        }
    }

    fn menu(&self) -> Element<'_, Message> {
        container(
            row![
                column![
                    container(text("📘").shaping(Shaping::Advanced).size(96))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .style(Algor::container),
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
                        .style(Algor::container),
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
        .padding(128)
        .into()
    }

    fn update(&mut self, message: Message) {}
}
