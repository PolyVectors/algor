// TODO: refactor this absolute dogshit immediately

use algor::{
    backend::{
        compiler::{self, generator::Location, lexer::Lexer, parser::Parser},
        config::{self, Config, RunSpeed},
        virtual_machine::{Computer, RuntimeMessage},
    },
    frontend::{
        font::{FAMILY_NAME, Font},
        style,
        theme::Theme,
        widgets::{horizontal_separator, vertical_separator},
    },
    shared::runtime::{self, Event, Input},
};
use iced::{
    Alignment, Background, Border, Color, Element, Length, Padding, Settings, Subscription, Task,
    advanced::text::Shaping,
    alignment,
    border::Radius,
    futures::channel::mpsc::Sender,
    widget::{
        button, column, container, horizontal_space, pane_grid, pick_list, radio, row, scrollable,
        text, text_editor, text_input,
    },
};
use iced_aw::{iced_fonts, number_input};
use rfd::AsyncFileDialog;
use std::{
    env,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

fn main() -> iced::Result {
    iced::application("algor", Algor::update, Algor::view)
        .settings(Settings {
            fonts: vec![Font::Regular.into(), Font::Bold.into()],
            default_font: iced::Font::with_name(FAMILY_NAME),
            ..Settings::default()
        })
        .subscription(Algor::subscription)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .theme(Algor::iced_theme)
        .run_with(Algor::new)
}

struct Algor {
    last_screen: Option<Screen>,
    screen: Screen,
    theme: Theme,
    editor_font_size: u8,
    lessons_directory: String,
    run_speed: Option<RunSpeed>,
    sandbox_panes: pane_grid::State<SandboxPane>,
    sandbox_pane_focused: Option<pane_grid::Pane>,
    editor_content: text_editor::Content,
    computer: Option<Arc<Mutex<Computer>>>,
    sender: Option<Sender<Input>>,
    error: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug)]
enum Message {
    SetScreen(Screen),
    SetTheme(Theme),
    SetEditorFontSize(u8),
    LessonsDirectoryChanged(String),
    RunSpeedSelected(RunSpeed),
    EditorInputChanged(text_editor::Action),
    BrowseLessonsDirectory,
    SaveConfig,
    ConfigSaved,
    SandboxPaneClicked(pane_grid::Pane),
    SandboxPaneDragged(pane_grid::DragEvent),
    AssembleClicked,
    RunClicked,
    Ready(Sender<Input>),
    UpdateState(Arc<Mutex<Computer>>),
    Todo,
    None,
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
            last_screen: None,
            screen: Screen::default(),
            editor_font_size: config.editor_font_size,
            lessons_directory: config.lessons_directory,
            run_speed: Some(config.run_speed),
            sandbox_panes: sandbox_panes,
            sandbox_pane_focused: None,
            editor_content: text_editor::Content::new(),
            computer: None,
            sender: None,
            error: String::new(),
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
                run_speed: Some(config.run_speed),
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

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(runtime::run).map(|event| match event {
            Event::Ready(sender) => Message::Ready(sender),
            Event::UpdateState(state) => Message::UpdateState(state),
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetScreen(screen) => {
                if &screen != &Screen::Settings {
                    self.last_screen = Some(screen.clone());
                }
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
                        run_speed: self.run_speed.clone().unwrap_or_default(),
                    }
                    .save(config_dir),
                    |_| Message::ConfigSaved,
                )
            }
            Message::ConfigSaved => Task::none(),
            Message::RunSpeedSelected(run_speed) => {
                self.run_speed = Some(run_speed);
                Task::none()
            }
            Message::SandboxPaneClicked(pane) => {
                self.sandbox_pane_focused = Some(pane);
                Task::none()
            }
            Message::SandboxPaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.sandbox_panes.drop(pane, target);
                Task::none()
            }
            Message::SandboxPaneDragged(_) => Task::none(),
            Message::EditorInputChanged(action) => {
                self.editor_content.perform(action);
                Task::none()
            }
            Message::AssembleClicked => {
                /* TODO: this should be handled by the runtime

                match compiler::compile(self.editor_content.text().as_str()) {
                    Ok(memory) => {
                        self.computer.memory = memory;
                        self.error = String::new();
                    }
                    Err(e) => {
                        self.error = e.to_string();
                    }
                }
                */
                Task::none()
            }
            Message::RunClicked => {
                if let Some(sender) = &mut self.sender {
                    sender.try_send(Input::RunClicked).unwrap(); // TODO: stupid
                }
                Task::none()
            }
            Message::Ready(sender) => {
                self.sender = Some(sender);
                Task::none()
            }
            Message::UpdateState(state) => {
                self.computer = Some(state);
                Task::none()
            }
            Message::Todo => todo!(),
            Message::None => Task::none(),
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
        column![
            container(
                pane_grid(&self.sandbox_panes, |pane, state, is_maximized| {
                    // TODO: implement From<Pane> for &str
                    let focused = self.sandbox_pane_focused == Some(pane);

                    let title = match state {
                        SandboxPane::Editor => "Editor",
                        SandboxPane::StateViewer => "State Viewer",
                    };

                    let title_bar = pane_grid::TitleBar::new(
                        container(text(title)).padding([4, 8]),
                    )
                    .style(if focused {
                        style::title_bar_focused
                    } else {
                        style::title_bar_unfocused
                    });

                    pane_grid::Content::new(match state {
                        SandboxPane::Editor => {
                            container(column![
                                container(
                                    column![
                                        row![
                                            button("Open").on_press(Message::Todo),
                                            button("Save").on_press(Message::Todo),
                                            horizontal_space(),
                                            button("Assemble").on_press(Message::AssembleClicked),
                                            button("Run").on_press(Message::RunClicked)
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
                                .padding(6),
                                container(
                                    container(
                                        /* TODO: output on top, notify user when input is needed */
                                        column![row![
                                            text("algor-sh $ ").style(style::terminal_text)
                                        ] /* INPUT HERE */]
                                        .padding(2)
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .style(style::terminal)
                                    .padding(2)
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .padding(6)
                            ])
                            .padding(Padding {
                                top: 0f32,
                                right: 2f32,
                                bottom: 2f32,
                                left: 2f32,
                            })
                            .width(Length::Fill)
                            .height(Length::Fill)
                        }
                        SandboxPane::StateViewer => container(
                            scrollable(
                                column![
                                    text("CPU:"),
                                    row![
                                        column![
                                            // TODO: this could be a macro
                                            text(if let Some(computer) = &self.computer {
                                                format!(
                                                    "{:0>2}",
                                                    computer.lock().unwrap().program_counter // TODO: stupid
                                                )
                                            } else {
                                                "?".to_string()
                                            }),
                                            text("PC").size(12)
                                        ]
                                        .align_x(alignment::Horizontal::Center)
                                        .width(Length::Fixed(36f32)),
                                        column![
                                            // TODO: this could be a macro
                                            text(if let Some(computer) = &self.computer {
                                                format!(
                                                    "{:0>3}",
                                                    computer.lock().unwrap().accumulator // TODO: stupid
                                                )
                                            } else {
                                                "?".to_string()
                                            }),
                                            text("ACC").size(12)
                                        ]
                                        .align_x(alignment::Horizontal::Center)
                                        .width(Length::Fixed(36f32)),
                                        column![
                                            // TODO: this could be a macro
                                            text(if let Some(computer) = &self.computer {
                                                format!(
                                                    "{:0>1}",
                                                    computer
                                                        .lock()
                                                        .unwrap()
                                                        .current_instruction_register // TODO: stupid
                                                )
                                            } else {
                                                "?".to_string()
                                            }),
                                            text("CIR").size(12)
                                        ]
                                        .align_x(alignment::Horizontal::Center)
                                        .width(Length::Fixed(36f32)),
                                        column![
                                            // TODO: this could be a macro
                                            text(if let Some(computer) = &self.computer {
                                                format!(
                                                    "{:0>2}",
                                                    computer
                                                        .lock()
                                                        .unwrap()
                                                        .memory_address_register // TODO: stupid
                                                )
                                            } else {
                                                "?".to_string()
                                            }),
                                            text("MAR").size(12)
                                        ]
                                        .align_x(alignment::Horizontal::Center)
                                        .width(Length::Fixed(36f32)),
                                        column![text("TBD"), text("MDR").size(12)]
                                            .align_x(alignment::Horizontal::Center)
                                            .width(Length::Fixed(36f32))
                                    ]
                                    .spacing(16),
                                    text("RAM:"),
                                    // TODO: values
                                ]
                                .spacing(16)
                                .padding(8),
                            )
                            .style(|theme: &iced::Theme, status: scrollable::Status| {
                                let palette = theme.extended_palette();

                                let rail = scrollable::Rail {
                                    background: None,
                                    scroller: scrollable::Scroller {
                                        border: Border {
                                            radius: Radius::new(2),
                                            ..Default::default()
                                        },
                                        color: palette.secondary.base.color,
                                    },
                                    border: Border {
                                        ..Default::default()
                                    },
                                };

                                scrollable::Style {
                                    container: container::Style {
                                        background: Some(Background::Color(
                                            palette.background.base.color,
                                        )),
                                        ..Default::default()
                                    },
                                    vertical_rail: rail,
                                    horizontal_rail: rail,
                                    gap: None,
                                }
                            })
                            .width(Length::Fill)
                            .height(Length::Fill),
                        )
                        .padding(Padding {
                            top: 0f32,
                            right: 2f32,
                            bottom: 2f32,
                            left: 2f32,
                        })
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(Alignment::Center),
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
                .on_drag(Message::SandboxPaneDragged)
            )
            .padding(8),
            row![
                button("Back").on_press(Message::SetScreen(Screen::Menu)),
                horizontal_space(),
                button("Settings").on_press(Message::SetScreen(Screen::Settings)),
            ]
        ]
        .padding(12)
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
