use algor::{
    backend::config::{self, Config},
    frontend::{
        font::{FAMILY_NAME, Font},
        style,
        theme::Theme,
        widgets::{horizontal_separator, vertical_separator},
    },
};
use iced::{
    Alignment, Background, Color, Element, Length, Padding, Settings, Task,
    advanced::text::Shaping,
    alignment,
    widget::{
        button, column, container, horizontal_space, pane_grid, pick_list, row, scrollable, text,
        text_editor, text_input,
    },
};
use iced_aw::{iced_fonts, number_input};
use rfd::AsyncFileDialog;
use std::{env, path::PathBuf, str::FromStr};

fn main() -> iced::Result {
    iced::application("algor", Algor::update, Algor::view)
        .settings(Settings {
            fonts: vec![Font::Regular.into(), Font::Bold.into()],
            default_font: iced::Font::with_name(FAMILY_NAME),
            ..Settings::default()
        })
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .theme(Algor::iced_theme)
        .run_with(Algor::new)
}

struct Algor {
    screen: Screen,
    theme: Theme,
    editor_font_size: u8,
    lessons_directory: String,
    sandbox_panes: pane_grid::State<SandboxPane>,
    sandbox_pane_focused: Option<pane_grid::Pane>,
    editor_content: text_editor::Content,
    terminal_input_content: String,
    terminal_output_content: Vec<String>,
}

#[derive(Clone, Debug)]
enum Message {
    SetScreen(Screen),
    SetTheme(Theme),
    SetEditorFontSize(u8),
    LessonsDirectoryChanged(String),
    TerminalInputChanged(String),
    EditorInputChanged(text_editor::Action),
    BrowseLessonsDirectory,
    SaveConfig,
    ConfigSaved,
    SandboxPaneClicked(pane_grid::Pane),
    SandboxPaneDragged(pane_grid::DragEvent),
    TerminalInputSubmitted,
}

#[derive(Clone, Debug, Default)]
enum Screen {
    #[default]
    Menu,
    LessonSelect,
    LessonView,
    Sandbox,
    Settings,
}

enum SandboxPane {
    Editor,
    StateViewer,
}

impl Default for Algor {
    fn default() -> Self {
        let config = Config::default();

        let (mut sandbox_panes, sandbox_pane) = pane_grid::State::new(SandboxPane::Editor);
        sandbox_panes.split(
            pane_grid::Axis::Vertical,
            sandbox_pane,
            SandboxPane::StateViewer,
        );

        Self {
            theme: config.theme,
            screen: Screen::default(),
            editor_font_size: config.editor_font_size,
            lessons_directory: config.lessons_directory,
            sandbox_panes: sandbox_panes,
            sandbox_pane_focused: None,
            editor_content: text_editor::Content::new(),
            terminal_input_content: "".to_string(),
            terminal_output_content: Vec::new(),
        }
    }
}

impl Algor {
    fn new() -> (Self, Task<Message>) {
        let mut config_dir = env::home_dir().unwrap();
        config_dir.push(config::CONFIG_PATH);

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
            Screen::Sandbox => self.sandbox(),
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
            Message::SaveConfig => {
                let mut config_dir = env::home_dir().unwrap();
                config_dir.push(config::CONFIG_PATH);

                Task::perform(
                    Config {
                        theme: self.theme.clone(),
                        editor_font_size: self.editor_font_size,
                        lessons_directory: self.lessons_directory.clone(),
                    }
                    .save(config_dir),
                    |_| Message::ConfigSaved,
                )
            }
            Message::ConfigSaved => Task::none(),
            Message::SandboxPaneClicked(pane) => {
                self.sandbox_pane_focused = Some(pane);
                Task::none()
            }
            Message::SandboxPaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.sandbox_panes.drop(pane, target);
                Task::none()
            }
            Message::TerminalInputChanged(content) => {
                self.terminal_input_content = content;
                Task::none()
            }
            Message::TerminalInputSubmitted => {
                self.terminal_output_content
                    .push(self.terminal_input_content.clone());
                self.terminal_input_content = "".to_string();

                Task::none()
            }
            Message::SandboxPaneDragged(_) => Task::none(),
            Message::EditorInputChanged(action) => {
                self.editor_content.perform(action);
                Task::none()
            }
        }
    }

    fn iced_theme(&self) -> iced::Theme {
        self.theme.clone().into()
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

    fn sandbox(&self) -> Element<'_, Message> {
        container(
            pane_grid(&self.sandbox_panes, |pane, state, is_maximized| {
                // TODO: implement From<Pane> for &str
                let focused = self.sandbox_pane_focused == Some(pane);

                let title = match state {
                    SandboxPane::Editor => "Editor",
                    SandboxPane::StateViewer => "State Viewer",
                };

                let title_bar = pane_grid::TitleBar::new(container(text(title)).padding([4, 8]))
                    .style(if focused {
                        style::title_bar_focused
                    } else {
                        style::title_bar_unfocused
                    });

                pane_grid::Content::new(match state {
                    SandboxPane::Editor => container(column![
                        container(
                            column![
                                row![
                                    button("Open"),
                                    horizontal_space(),
                                    button("Save"),
                                    button("Run")
                                ]
                                .spacing(4),
                                text_editor(&self.editor_content)
                                    .height(Length::Fill)
                                    .on_action(Message::EditorInputChanged)
                                    .highlight("py", iced::highlighter::Theme::Base16Ocean)
                            ]
                            .spacing(6)
                            .align_x(alignment::Horizontal::Right)
                        )
                        .style(|theme: &iced::Theme| container::Style {
                            background: Some(Background::Color(theme.palette().background)),
                            ..Default::default()
                        })
                        .padding(6)
                    ])
                    .padding(Padding {
                        top: 0f32,
                        right: 2f32,
                        bottom: 2f32,
                        left: 2f32,
                    })
                    .width(Length::Fill)
                    .height(Length::Fill),
                    SandboxPane::StateViewer => container(column![]),
                })
                .style(if focused {
                    style::grid_pane_focused
                } else {
                    style::grid_pane_unfocused
                })
                .title_bar(title_bar)
            })
            .spacing(8)
            .on_click(Message::SandboxPaneClicked)
            .on_drag(Message::SandboxPaneDragged),
        )
        .padding(8)
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
                    text("Files").font(Font::Bold).size(24),
                    column![
                        text("Lessons Directory:").size(16),
                        row![
                            text_input("...", &self.lessons_directory)
                                .on_input(Message::LessonsDirectoryChanged)
                                .on_submit(Message::TerminalInputSubmitted),
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

async fn pick_folder() -> String {
    let mut config_dir = env::home_dir().unwrap();
    config_dir.push(config::CONFIG_PATH);

    let config = Config::try_from(config_dir).unwrap_or_default();

    AsyncFileDialog::new()
        .set_title("Pick lessons directory...")
        .pick_folder()
        .await
        .unwrap_or(PathBuf::from_str(&config.lessons_directory).unwrap().into())
        .path()
        .to_str()
        .to_owned()
        .unwrap()
        .to_owned()
}
