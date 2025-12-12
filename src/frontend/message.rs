use crate::backend::config::RunSpeed;
use crate::frontend::screen::Screen;
use crate::frontend::theme::Theme;
use crate::shared::runtime::Input;
use crate::shared::vm::Computer;

use iced::futures::channel::mpsc::Sender;
use iced::widget::{pane_grid, text_editor};

use std::sync::{Arc, Mutex};
use std::time::Instant;

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
