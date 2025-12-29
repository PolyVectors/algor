use crate::backend::config::RunSpeed;
use crate::frontend::screen::Screen;
use crate::frontend::util::{
    font::Font,
    theme::Theme,
    widgets::{horizontal_separator, vertical_separator},
};

use iced::{
    Alignment, Element, Length,
    widget::{button, column, horizontal_space, pick_list, radio, row, text, text_input},
};
use iced_aw::{iced_fonts, widgets::number_input};

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
    EditorFontSizeChanged(u8),
    LessonsDirectoryChanged(String),
    BrowseClicked,
    RunSpeedSelected(RunSpeed),
    BackClicked,
    SaveClicked,
}

pub enum Event {
    GoBack,
    SetSettings(State),
}

#[derive(Debug, Clone)]
pub struct State {
    pub theme: Theme,
    pub editor_font_size: u8,
    pub lessons_directory: String,
    pub run_speed: Option<RunSpeed>,
    // would be nice if this were a &'a Screen
    pub last_screen: Box<Screen>,
}

impl State {
    pub fn with_screen(screen: Screen) -> Self {
        Self {
            theme: Theme::Light,
            editor_font_size: 16,
            lessons_directory: String::new(),
            run_speed: Some(RunSpeed::Medium),
            last_screen: Box::new(screen),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::ThemeSelected(theme) => {
                self.theme = theme;
                None
            }
            // TODO: could be an rc but who gaf
            Message::SaveClicked => Some(Event::SetSettings(self.clone())),
            Message::BackClicked => Some(Event::GoBack),
            _ => todo!(),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        column![
            text("Settings").font(Font::Bold).size(32),
            horizontal_separator(),
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
                horizontal_space(),
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
