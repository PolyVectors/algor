use crate::backend::config::RunSpeed;
use crate::frontend::utils::theme::Theme;
use crate::frontend::{
    app::{Algor, Screen},
    handlers::messages::Message,
    utils::font::Font,
    utils::widgets::{horizontal_separator, vertical_separator},
};
use iced::{
    Alignment, Element, Length,
    widget::{button, column, horizontal_space, pick_list, radio, row, text, text_input},
};
use iced_aw::{iced_fonts, number_input};

impl Algor {
    pub fn settings(&self) -> Element<'_, Message> {
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
                    text("Functionality").font(Font::Bold).size(24),
                    column![
                        text("Lessons Directory:").size(16),
                        row![
                            text_input("...", &self.lessons_directory)
                                .on_input(Message::LessonsDirectoryChanged),
                            button("Browse").on_press(Message::BrowseLessonsDirectory)
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
                button("Back").on_press(Message::SetScreen(
                    self.last_screen.as_ref().unwrap_or(&Screen::Menu).clone()
                )),
                horizontal_space(),
                button("Save").on_press(Message::SaveConfig)
            ]
        ]
        .height(Length::Fill)
        .width(Length::Fill)
        .spacing(16)
        .padding(12)
        .into()
    }
}
