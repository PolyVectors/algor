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

// Messages specific to the sandbox screen
#[derive(Debug, Clone)]
pub enum Message {
    // Events related to the pane widget
    PaneClicked(pane_grid::Pane),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    // Text editor pane messages
    Editor(editor::Message),
    // State viewer pane messages
    StateViewer(state_viewer::Message),
    // Terminal pane messages
    Terminal(terminal::Message),
    BackClicked,
    SettingsClicked,
}

// Events specific to sandbox screen
pub enum Event {
    // Result from clicking the open button (to load a file)
    OpenLMC(State),
    // Result from clicking the save button
    SaveLMC(State),
    // Result from clicking the run button in editor pane
    Run,
    // Ditto for stop button
    Stop,
    // Ditto for reset button
    Reset,
    // Result from sending input in the input box in the editor pane
    SubmitInput(String),
    // Result from clicking back button
    ToMenu,
    // Result from clicking settings button
    ToSettings,
}

// Represents individual panes in the screen
#[derive(Debug, Clone)]
pub enum Pane {
    Editor,
    StateViewer,
    Terminal,
}

#[derive(Debug, Clone)]
pub struct State {
    panes: pane_grid::State<Pane>,
    // Current pane the user is interacting with
    pane_focused: Option<pane_grid::Pane>,
    // Text content of the text editor pane
    pub content: text_editor::Content,
    pub text_size: u32,
    pub computer: Arc<Mutex<Computer>>,
    sender: Arc<Mutex<Sender<Input>>>,
    // Input in the input box in the editor pane
    input: String,
    // Terminal pane outputs
    pub output: Vec<Box<str>>,
    // Terminal pane error
    pub error: String,
}

impl State {
    // Constructor for the sandbox screen
    pub fn new(
        computer: Arc<Mutex<Computer>>,
        sender: Arc<Mutex<Sender<Input>>>,
        text_size: u32,
    ) -> Self {
        // Start with editor pane
        let (mut panes, pane) = pane_grid::State::new(Pane::Editor);

        // Split vertically (editor pane on left, lesson pane on the right)
        panes.split(pane_grid::Axis::Vertical, pane, Pane::StateViewer);
        // Split horizontally (editor pane on top, terminal pane below, lesson pane on the right)
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
            // Pane interactivity boilerplate
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

                    // Send a message to the sender to compile the program with the source code in the text editor
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

                    // Convert Pane to static string
                    let title = match state {
                        Pane::Editor => "Editor",
                        Pane::StateViewer => "State Viewer",
                        Pane::Terminal => "Terminal",
                    };

                    // Add title to title bar
                    let title_bar = pane_grid::TitleBar::new(
                        container(text(title)).padding([4, 8]),
                    )
                    .style(if focused {
                        style::title_bar_focused
                    } else {
                        style::title_bar_unfocused
                    });

                    pane_grid::Content::new(match state {
                        // Use pane widgets to display content, passing in relevant values

                        // Pass in the input attribute as a Some value to tell the pane widget to show an input box and open and save buttons
                        Pane::Editor => editor(&self.content, self.text_size, Some(&self.input))
                            .map(Message::Editor),

                        Pane::StateViewer => {
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
                    // Set title bar to previously defined title_bar variable
                    .title_bar(title_bar)
                })
                .spacing(8)
                // Connect boilerplate pane interactivity Message variants
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
