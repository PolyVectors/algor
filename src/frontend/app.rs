use crate::frontend::handlers::messages::Message;
use crate::{
    backend::{
        compiler::{self, generator::Location, lexer::Lexer, parser::Parser},
        config::{self, Config, RunSpeed},
    },
    frontend::components::sandbox::SandboxPane,
    frontend::utils::{
        font::{FAMILY_NAME, Font},
        style::{self, terminal, terminal_err, terminal_out},
        theme::Theme,
        widgets::{horizontal_separator, vertical_separator},
    },
    shared::{
        runtime::{self, Event, Input},
        vm::Computer,
    },
};
use iced::{
    Alignment, Background, Border, Color, Element, Length, Padding, Settings, Subscription, Task,
    advanced::text::Shaping,
    alignment,
    border::Radius,
    futures::channel::mpsc::Sender,
    time,
    widget::{
        button, column, container, horizontal_space, pane_grid, pick_list, radio, rich_text, row,
        scrollable, span, text, text_editor, text_input,
    },
};
use rfd::AsyncFileDialog;
use std::{
    env,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

pub struct Algor {
    pub last_screen: Option<Screen>,
    pub screen: Screen,
    pub theme: Theme,
    pub editor_font_size: u8,
    pub lessons_directory: String,
    pub running: bool,
    pub run_speed: Option<RunSpeed>,
    pub sandbox_panes: pane_grid::State<SandboxPane>,
    pub sandbox_pane_focused: Option<pane_grid::Pane>,
    pub editor_content: text_editor::Content,
    pub computer: Option<Arc<Mutex<Computer>>>,
    pub sender: Option<Sender<Input>>,
    pub needs_input: bool,
    pub input: String,
    pub output: Vec<Box<str>>,
    pub error: String,
    pub underlined: u8,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Screen {
    #[default]
    Menu,
    LessonSelect,
    LessonView,
    Sandbox,
    Settings,
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
        sandbox_panes.split(
            pane_grid::Axis::Horizontal,
            sandbox_pane,
            SandboxPane::Terminal,
        );

        Self {
            theme: config.theme,
            last_screen: None,
            screen: Screen::default(),
            editor_font_size: config.editor_font_size,
            lessons_directory: config.lessons_directory,
            running: false,
            run_speed: Some(config.run_speed),
            sandbox_panes: sandbox_panes,
            sandbox_pane_focused: None,
            editor_content: text_editor::Content::new(),
            computer: None,
            sender: None,
            needs_input: false,
            input: String::new(),
            output: Vec::new(),
            error: String::new(),
            underlined: 0,
        }
    }
}

impl Algor {
    pub fn new() -> (Self, Task<Message>) {
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

    pub fn view(&self) -> Element<'_, Message> {
        match self.screen {
            Screen::Menu => self.menu(),
            Screen::Settings => self.settings(),
            Screen::Sandbox => self.sandbox(),
            _ => todo!(),
        }
    }

    pub fn iced_theme(&self) -> iced::Theme {
        self.theme.clone().into()
    }
}
