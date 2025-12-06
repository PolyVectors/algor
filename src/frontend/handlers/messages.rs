use crate::frontend::app::{Algor, Screen};
use crate::shared::vm::Computer;
use crate::{
    backend::config::{self, Config, RunSpeed},
    frontend::utils::theme::Theme,
    shared::runtime::Input,
};
use iced::{
    Task,
    futures::channel::mpsc::Sender,
    widget::{pane_grid, text_editor},
};
use rfd::AsyncFileDialog;
use std::time::Instant;
use std::{
    env,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub enum Message {
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
    SandboxPaneResized(pane_grid::ResizeEvent),
    AssembleClicked,
    RunClicked,
    StopClicked,
    Halt,
    Reset,
    Step(Instant),
    Ready(Sender<Input>),
    UpdateState(Arc<Mutex<Computer>>),
    SetError(String),
    AskInput,
    InputChanged(String),
    InputSubmitted,
    AppendOutput(Box<str>),
    Todo,
    None,
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

// TODO: there has to be a better way to do this, check halloy or other iced projects
impl Algor {
    pub fn update(&mut self, message: Message) -> Task<Message> {
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
            Message::SandboxPaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.sandbox_panes.resize(split, ratio);
                Task::none()
            }
            Message::SandboxPaneDragged(_) => Task::none(),
            Message::EditorInputChanged(action) => {
                self.editor_content.perform(action);
                Task::none()
            }
            Message::AssembleClicked => {
                self.error = String::new();
                if let Some(sender) = &mut self.sender {
                    sender
                        .try_send(Input::AssembleClicked(self.editor_content.text()))
                        .unwrap(); // TODO: stupid
                }
                Task::none()
            }
            Message::RunClicked => {
                self.running = true;
                Task::none()
            }
            Message::StopClicked => {
                self.running = false;
                Task::none()
            }
            Message::AskInput => {
                self.running = false;
                self.needs_input = true;

                self.output.push("Waiting for input...".into());

                Task::none()
            }
            Message::Step(_) => {
                self.error = String::new();

                if let Some(sender) = &mut self.sender {
                    sender.try_send(Input::Step).unwrap(); // TODO: stupid
                }

                Task::none()
            }
            Message::Reset => {
                self.running = false;
                self.error = String::new();
                self.output = Vec::new();

                if let Some(sender) = &mut self.sender {
                    sender.try_send(Input::Reset).unwrap(); // TODO: stupid
                }

                Task::none()
            }
            Message::Halt => {
                self.running = false;
                Task::none()
            }
            Message::Ready(sender) => {
                self.sender = Some(sender);
                Task::none()
            }
            Message::UpdateState(state) => {
                self.computer = Some(state);

                if let Some(computer) = &self.computer
                    && let Ok(computer) = computer.lock()
                {
                    self.underlined = computer.program_counter;
                }

                Task::none()
            }
            Message::InputChanged(input) => {
                self.input = input;

                Task::none()
            }
            Message::InputSubmitted => {
                if let Some(sender) = &mut self.sender
                    && self.needs_input
                {
                    sender
                        .try_send(Input::SetInput(self.input.clone()))
                        .unwrap(); // TODO: stupid
                    self.running = true;
                }

                Task::none()
            }
            Message::AppendOutput(output) => {
                self.output.push(output);
                Task::none()
            }
            Message::SetError(error) => {
                self.error = error;
                self.running = false;

                Task::none()
            }
            Message::Todo => todo!(),
            Message::None => Task::none(),
        }
    }
}
