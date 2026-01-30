use std::sync::{Arc, Mutex};

use crate::shared::vm::Computer;
use crate::{
    frontend::pane::{
        editor::{self, editor},
        state_viewer::{self, state_viewer},
        style,
        terminal::{self, terminal},
    },
    shared::runtime::Input,
};

use iced::{
    Element,
    futures::channel::mpsc::Sender,
    widget::{button, column, container, pane_grid, row, space, text, text_editor},
};

#[derive(Debug, Clone)]
pub enum Message {
    PaneClicked(pane_grid::Pane),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    Editor(editor::Message),
    StateViewer(state_viewer::Message),
    Terminal(terminal::Message),
    BackClicked,
    SettingsClicked,
}

pub enum Event {
    OpenLMC(State),
    SaveLMC(State),
    Run,
    Stop,
    Reset,
    SubmitInput(String),
    ToMenu,
    ToSettings,
}

#[derive(Debug, Clone)]
pub enum Pane {
    Editor,
    StateViewer,
    Terminal,
}

#[derive(Debug, Clone)]
pub struct State {
    panes: pane_grid::State<Pane>,
    pane_focused: Option<pane_grid::Pane>,
    pub content: text_editor::Content,
    pub text_size: u32,
    pub computer: Arc<Mutex<Computer>>,
    sender: Arc<Mutex<Sender<Input>>>,
    input: String,
    pub output: Vec<Box<str>>,
    pub error: String,
}

impl State {
    pub fn new(
        computer: Arc<Mutex<Computer>>,
        sender: Arc<Mutex<Sender<Input>>>,
        text_size: u32,
    ) -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::Editor);

        panes.split(pane_grid::Axis::Vertical, pane, Pane::StateViewer);
        panes.split(pane_grid::Axis::Horizontal, pane, Pane::Terminal);

        Self {
            panes,
            pane_focused: None,
            content: text_editor::Content::new(),
            text_size,
            computer,
            sender,
            input: String::new(),
            output: Vec::new(),
            error: String::new(),
        }
    }
}

impl State {
    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::PaneClicked(pane) => {
                self.pane_focused = Some(pane);
            }
            Message::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }

            Message::Editor(message) => match message {
                editor::Message::ContentChanged(action) => self.content.perform(action),
                editor::Message::InputChanged(input) => self.input = input,
                editor::Message::InputSubmitted => {
                    return Some(Event::SubmitInput(self.input.clone()));
                }

                editor::Message::AssembleClicked => {
                    self.error = String::new();

                    if let Ok(mut sender) = self.sender.lock() {
                        sender
                            .try_send(Input::AssembleClicked(self.content.text()))
                            .unwrap()
                    }
                }

                editor::Message::OpenClicked => return Some(Event::OpenLMC(self.clone())),
                editor::Message::SaveClicked => return Some(Event::SaveLMC(self.clone())),

                editor::Message::ResetClicked => return Some(Event::Reset),
                editor::Message::StopClicked => return Some(Event::Stop),
                editor::Message::RunClicked => return Some(Event::Run),
            },

            Message::SettingsClicked => return Some(Event::ToSettings),
            Message::BackClicked => return Some(Event::ToMenu),

            _ => {}
        }
        None
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            container(
                pane_grid(&self.panes, |pane, state, _is_maximized| {
                    let focused = self.pane_focused == Some(pane);

                    let title = match state {
                        Pane::Editor => "Editor",
                        Pane::StateViewer => "State Viewer",
                        Pane::Terminal => "Terminal",
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
                        Pane::Editor => editor(&self.content, self.text_size, Some(&self.input))
                            .map(Message::Editor),

                        Pane::StateViewer => {
                            // TODO: stop unwrapping
                            state_viewer(&self.computer.lock().unwrap()).map(Message::StateViewer)
                        }

                        Pane::Terminal => {
                            terminal(&self.output, &self.error).map(Message::Terminal)
                        }
                    })
                    .style(if focused {
                        style::grid_pane_focused
                    } else {
                        style::grid_pane_unfocused
                    })
                    .title_bar(title_bar)
                })
                .spacing(8)
                .on_click(Message::PaneClicked)
                .on_drag(Message::PaneDragged)
                .on_resize(10, Message::PaneResized)
            )
            .padding([8, 0]),
            row![
                button("Back").on_press(Message::BackClicked),
                space::horizontal(),
                button("Settings").on_press(Message::SettingsClicked),
            ]
        ]
        .padding(12)
        .into()
    }
}
