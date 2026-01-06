use std::sync::{Arc, Mutex};

use crate::frontend::pane::{
    editor::{self, editor},
    state_viewer::{self, state_viewer},
    style,
    terminal::{self, terminal},
};
use crate::shared::vm::Computer;

use iced::{
    Element, Length, Padding, alignment,
    widget::{button, column, container, pane_grid, row, space, text, text_editor, text_input},
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
    SetComputer(Computer),
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
    content: text_editor::Content,
    computer: Arc<Mutex<Computer>>,
    input: String,
    output: Vec<Box<str>>,
    error: String,
}

impl State {
    pub fn from_computer(computer: Arc<Mutex<Computer>>) -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::Editor);

        panes.split(pane_grid::Axis::Vertical, pane, Pane::StateViewer);
        panes.split(pane_grid::Axis::Horizontal, pane, Pane::Terminal);

        Self {
            panes,
            pane_focused: None,
            content: text_editor::Content::new(),
            computer,
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
                _ => todo!(),
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
                pane_grid(&self.panes, |pane, state, is_maximized| {
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
                        // TODO: see why this is so slow
                        Pane::Editor => editor(&self.content, &self.input).map(Message::Editor),

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
