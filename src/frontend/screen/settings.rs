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

// Opens a file dialog selector specific to the operating system and gets the directory the user picks, closing the dialog returns the value from the config
pub async fn browse_directory() -> String {
    let mut config_path = env::home_dir().unwrap();
    config_path.push(config::CONFIG_PATH);

    let config = Config::try_from(config_path).unwrap_or_default();

    AsyncFileDialog::new()
        .set_title("Pick lessons directory...")
        .pick_folder()
        .await
        // If the user exits out, restore the previous value from the config
        .unwrap_or(PathBuf::from(&config.lessons_directory).into())
        .path()
        .to_str()
        .to_owned()
        // If converting the path to a string doesn't work, return the default string (an empty one)
        .unwrap_or_default()
        .to_owned()
}

// Messages specific to the settings screen
#[derive(Debug, Clone)]
pub enum Message {
    // Theme picked from drop-down menu
    ThemeSelected(Theme),
    // Editor font size changed via buttons/text input
    EditorFontSizeChanged(u32),
    // Lessons directory changed via typing
    LessonsDirectoryChanged(String),
    // Browse button clicked (to change lessons directory)
    BrowseClicked,
    // Run speed selected from radio list
    RunSpeedSelected(RunSpeed),
    // Back button clicked to go back (to previous screen, i.e. menu, lesson viewer, sandbox)
    BackClicked,
    // Save clicked (to set config in Algor struct and serialise changes and write to disk, using backend)
    SaveClicked,
}

// Outputs from this state's update method, used as inputs for Screen enum's update method
pub enum Event {
    // Result from clicking "Back", providing the screen with the previous state
    GoBack(Box<Screen>),
    // Event emitted as a result of clicking the save button
    SetConfig(Config),
    // Event emitted as a result of clicking the browse button (needs to be an event as Task is required for asynchronous function)
    PickLessonsDirectory(State),
}

// Same as Config struct but with the screen the user came from attached
#[derive(Debug, Clone)]
pub struct State {
    pub theme: Theme,
    pub editor_font_size: u32,
    pub lessons_directory: String,
    pub run_speed: Option<RunSpeed>,
    // Allows for restoring the screen from which the user came from
    pub last_screen: Box<Screen>,
}

// Helper associated functions for keep main.rs clean
impl State {
    // Create a settings menu state from a Config and the previous screen
    pub fn new(value: Config, last_screen: Box<Screen>) -> Self {
        Self {
            theme: value.theme,
            editor_font_size: value.editor_font_size,
            lessons_directory: value.lessons_directory,
            run_speed: Some(value.run_speed),
            last_screen,
        }
    }

    // Create a settings menu state from a default Config and the previous screen
    pub fn from_screen(last_screen: Box<Screen>) -> Self {
        let config = Config::default();

        Self {
            theme: config.theme,
            editor_font_size: config.editor_font_size,
            lessons_directory: config.lessons_directory,
            run_speed: Some(config.run_speed),
            last_screen,
        }
    }
}

impl State {
    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            // When the user selects a theme, reflect it in the state
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
            // Ditto for lessons directory
            Message::LessonsDirectoryChanged(directory) => {
                self.lessons_directory = directory;
            }
            // Ditto for editor font size
            Message::EditorFontSizeChanged(size) => {
                self.editor_font_size = size;
            }
            // Ditto for run speed
            Message::RunSpeedSelected(speed) => {
                self.run_speed = Some(speed);
            }

            // When the user clicks the browse button, bubble it up to the Screen update method
            Message::BrowseClicked => return Some(Event::PickLessonsDirectory(self.clone())),
            // Ditto for the save button
            Message::SaveClicked => return Some(Event::SetConfig(self.into())),
            // Ditto for back button
            Message::BackClicked => {
                return Some(match &*self.last_screen {
                    // Immediately reflect changes in text size
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

                    // No text size for other screens, simply clone
                    _ => Event::GoBack(self.last_screen.clone()),
                });
            }
        }

        // For methods that don't return, send an empty value to be ignored by the Screen update method
        None
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            // Title text
            text("Settings").font(Font::Bold).size(32),
            // Horizontal quad
            separator::horizontal(),
            row![
                // Column with settings related to appearance
                column![
                    text("Appearance").font(Font::Bold).size(24),
                    // "Theme" text labelling a theme selector
                    column![
                        text("Theme:").size(16),
                        pick_list(Theme::ALL, Some(self.theme.clone()), |theme| {
                            Message::ThemeSelected(theme)
                        })
                        .width(Length::Fill)
                    ]
                    .spacing(8),
                    // "Font Size" text labelling a bounded number input from 8px to 32px
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
                // Vertical quad
                separator::vertical(),
                // Column with settings related to functionality
                column![
                    text("Functionality").font(Font::Bold).size(24),
                    // "Lesson Directory" text labelling text input and browse button
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
                    // "Run Speed" text labelling radio menu
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
            // Back and save buttons on opposite bottom corners
            row![
                button("Back").on_press(Message::BackClicked),
                // Invisible widget that takes up as much space as possible
                space::horizontal(),
                button("Save").on_press(Message::SaveClicked)
            ]
        ]
        .height(Length::Fill)
        .width(Length::Fill)
        .spacing(16)
        .padding(12)
        // Turn Column struct into Element struct
        .into()
    }
}
