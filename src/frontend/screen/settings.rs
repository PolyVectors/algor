use std::env;
use std::path::PathBuf;

use crate::backend::config::{self, Config, RunSpeed};
use crate::frontend::screen::Screen;
use crate::frontend::util::{font::Font, theme::Theme, widgets::separator};

use iced::{
    Alignment, Element, Length,
    widget::{button, column, pick_list, radio, row, space, text, text_input},
};
use iced_aw::widgets::number_input;

use rfd::AsyncFileDialog;

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
    EditorFontSizeChanged(u32),
    LessonsDirectoryChanged(String),
    BrowseClicked,
    RunSpeedSelected(RunSpeed),
    BackClicked,
    SaveClicked,
}

pub enum Event {
    GoBack(Box<Screen>),
    SetConfig(Config),
    PickLessonsDirectory(State),
}

pub async fn browse_directory() -> String {
    let mut config_dir = env::home_dir().unwrap();
    config_dir.push(config::CONFIG_PATH);

    let config = Config::try_from(config_dir).unwrap_or_default();

    AsyncFileDialog::new()
        .set_title("Pick lessons directory...")
        .pick_folder()
        .await
        .unwrap_or(PathBuf::from(&config.lessons_directory).into())
        .path()
        .to_str()
        .to_owned()
        .unwrap_or_default()
        .to_owned()
}

#[derive(Debug, Clone)]
pub struct State {
    pub theme: Theme,
    pub editor_font_size: u32,
    pub lessons_directory: String,
    pub run_speed: Option<RunSpeed>,
    pub last_screen: Box<Screen>,
}

impl State {
    pub fn new(value: Config, last_screen: Box<Screen>) -> Self {
        Self {
            theme: value.theme,
            editor_font_size: value.editor_font_size,
            lessons_directory: value.lessons_directory,
            run_speed: Some(value.run_speed),
            last_screen,
        }
    }

    pub fn from_screen(last_screen: Box<Screen>) -> Self {
        Self {
            theme: Theme::Light,
            editor_font_size: 16,
            lessons_directory: String::new(),
            run_speed: Some(RunSpeed::Medium),
            last_screen,
        }
    }
}

impl State {
    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
            Message::LessonsDirectoryChanged(directory) => {
                self.lessons_directory = directory;
            }
            Message::EditorFontSizeChanged(size) => {
                self.editor_font_size = size;
            }
            Message::RunSpeedSelected(speed) => {
                self.run_speed = Some(speed);
            }

            Message::BrowseClicked => return Some(Event::PickLessonsDirectory(self.clone())),
            Message::SaveClicked => return Some(Event::SetConfig(self.into())),

            Message::BackClicked => {
                return Some(match &*self.last_screen {
                    Screen::LessonView(screen_state) => {
                        let mut new_state = screen_state.clone();
                        new_state.text_size = self.editor_font_size;

                        Event::GoBack(Box::new(Screen::LessonView(new_state)))
                    }
                    Screen::Sandbox(screen_state) => {
                        let mut new_state = screen_state.clone();
                        new_state.text_size = self.editor_font_size;

                        Event::GoBack(Box::new(Screen::Sandbox(new_state)))
                    }
                    _ => Event::GoBack(self.last_screen.clone()),
                });
            }
        }
        None
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("Settings").font(Font::Bold).size(32),
            separator::horizontal(),
            row![
                column![
                    text("Appearance").font(Font::Bold).size(24),
                    column![
                        text("Theme:").size(16),
                        pick_list(Theme::ALL, Some(self.theme.clone()), |theme| {
                            Message::ThemeSelected(theme)
                        })
                        .width(Length::Fill)
                    ]
                    .spacing(8),
                    column![
                        text("Font Size:").align_y(Alignment::Center).size(16),
                        row![
                            number_input(&self.editor_font_size, 8..=32, |size| {
                                Message::EditorFontSizeChanged(size)
                            })
                            .style(iced_aw::style::number_input::primary)
                            .step(2)
                            .font(Font::Regular.into())
                            .width(Length::Fill),
                            text("px").width(Length::Fill)
                        ]
                        .spacing(8)
                    ]
                    .spacing(8)
                ]
                .width(Length::Fill)
                .spacing(32),
                separator::vertical(),
                column![
                    text("Functionality").font(Font::Bold).size(24),
                    column![
                        text("Lessons Directory:").size(16),
                        row![
                            text_input("...", &self.lessons_directory)
                                .on_input(Message::LessonsDirectoryChanged),
                            button("Browse").on_press(Message::BrowseClicked)
                        ]
                        .spacing(8)
                    ]
                    .spacing(8),
                    column![
                        text("Run Speed:").size(16),
                        radio(
                            "Slow",
                            RunSpeed::Slow,
                            self.run_speed,
                            Message::RunSpeedSelected,
                        ),
                        radio(
                            "Medium",
                            RunSpeed::Medium,
                            self.run_speed,
                            Message::RunSpeedSelected,
                        ),
                        radio(
                            "Fast",
                            RunSpeed::Fast,
                            self.run_speed,
                            Message::RunSpeedSelected,
                        ),
                        radio(
                            "Instant",
                            RunSpeed::Instant,
                            self.run_speed,
                            Message::RunSpeedSelected,
                        ),
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
                button("Back").on_press(Message::BackClicked),
                space::horizontal(),
                button("Save").on_press(Message::SaveClicked)
            ]
        ]
        .height(Length::Fill)
        .width(Length::Fill)
        .spacing(16)
        .padding(12)
        .into()
    }
}
