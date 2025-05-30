use std::{env, path::PathBuf, str::FromStr};

use algor::{
    backend::config::{self, Config},
    frontend::{
        font::{FAMILY_NAME, Font},
        style,
        widgets::{horizontal_separator, vertical_separator},
    },
};
use iced::{
    Alignment, Element, Length, Settings, Task, Theme,
    advanced::text::Shaping,
    alignment,
    widget::{button, column, container, horizontal_space, pick_list, row, text, text_input},
};
use iced_aw::{iced_fonts, number_input};
use rfd::AsyncFileDialog;

fn main() -> iced::Result {
    iced::application("Algor", Algor::update, Algor::view)
        .settings(Settings {
            fonts: vec![Font::Regular.into(), Font::Bold.into()],
            default_font: iced::Font::with_name(FAMILY_NAME),
            ..Settings::default()
        })
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .theme(Algor::theme)
        .run_with(Algor::new)
}

struct Algor {
    screen: Screen,
    theme: Theme,
    editor_font_size: u8,
    lessons_directory: String,
}

#[derive(Clone, Debug)]
enum Message {
    SetScreen(Screen),
    SetTheme(Theme),
    SetEditorFontSize(u8),
    LessonsDirectoryChanged(String),
    BrowseLessonsDirectory,
}

#[derive(Clone, Debug)]
enum Screen {
    Menu,
    LessonSelect,
    LessonView,
    Sandbox,
    Settings,
}

impl Default for Algor {
    fn default() -> Self {
        let config = Config::default();

        Self {
            theme: config.theme,
            screen: Screen::default(),
            editor_font_size: config.editor_font_size,
            lessons_directory: config.lessons_directory,
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Menu
    }
}

impl Algor {
    fn new() -> (Self, Task<Message>) {
        let mut config_dir = env::home_dir().unwrap();
        config_dir.push(config::CONFIG_DIR);

        // TODO: auto generate new config, Config::create()?
        let config = Config::try_from(config_dir).unwrap_or_default();

        (
            Self {
                theme: config.theme,
                editor_font_size: config.editor_font_size,
                lessons_directory: config.lessons_directory,
                ..Default::default()
            },
            Task::none(),
        )
    }

    fn view(&self) -> Element<'_, Message> {
        match self.screen {
            Screen::Menu => self.menu(),
            Screen::Settings => self.settings(),
            _ => todo!(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetScreen(screen) => {
                self.screen = screen;
                Task::none()
            }
            Message::SetTheme(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::SetEditorFontSize(size) => {
                self.editor_font_size = size;
                Task::none()
            }
            Message::LessonsDirectoryChanged(directory) => {
                self.lessons_directory = directory;
                Task::none()
            }
            Message::BrowseLessonsDirectory => {
                Task::perform(pick_folder(), Message::LessonsDirectoryChanged)
            }
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn menu(&self) -> Element<'_, Message> {
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
                            .on_press(Message::SetScreen(Screen::LessonSelect))
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

    fn settings(&self) -> Element<'_, Message> {
        column![
            text("Settings").font(Font::Bold).size(32),
            horizontal_separator(),
            row![
                column![
                    text("Appearance").font(Font::Bold).size(24),
                    column![
                        text("Theme:").size(16),
                        pick_list(Theme::ALL, Some(self.theme.clone()), |theme| {
                            Message::SetTheme(theme)
                        })
                        .width(Length::Fill)
                    ]
                    .spacing(8),
                    column![
                        text("Font Size:").align_y(Alignment::Center).size(16),
                        row![
                            number_input(&self.editor_font_size, 8..=32, |size| {
                                Message::SetEditorFontSize(size)
                            })
                            .style(iced_aw::style::number_input::primary)
                            .step(2)
                            .font(iced_fonts::REQUIRED_FONT)
                            .width(Length::Fill),
                            text("px").width(Length::Fill)
                        ]
                        .spacing(8)
                    ]
                    .spacing(8)
                ]
                .width(Length::Fill)
                .spacing(32),
                vertical_separator(),
                column![
                    text("Preferences").font(Font::Bold).size(24),
                    column![
                        text("Lessons Directory:").size(16),
                        row![
                            text_input("...", &self.lessons_directory)
                                .on_input(|directory| Message::LessonsDirectoryChanged(directory)),
                            button("Browse").on_press(Message::BrowseLessonsDirectory)
                        ]
                        .spacing(8)
                    ]
                    .spacing(8)
                ]
                .width(Length::Fill)
                .spacing(32)
            ]
            .height(Length::Fill)
            .width(Length::Fill)
            .spacing(64),
            row![
                button("Back").on_press(Message::SetScreen(Screen::Menu)),
                horizontal_space(),
                button("Save")
            ]
        ]
        .height(Length::Fill)
        .width(Length::Fill)
        .spacing(16)
        .padding(12)
        .into()
    }
}

async fn pick_folder() -> String {
    AsyncFileDialog::new()
        .set_title("Pick lessons directory...")
        .pick_folder()
        .await
        .unwrap_or(
            PathBuf::from_str(Algor::default().lessons_directory.as_str())
                .unwrap()
                .into(),
        )
        .path()
        .to_str()
        .to_owned()
        .unwrap()
        .to_owned()
}
