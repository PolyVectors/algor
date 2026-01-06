use iced::{
    Element, Length, Padding, alignment,
    widget::{button, column, container, pane_grid, row, space, text, text_editor, text_input},
};

use crate::frontend::pane::{
    editor::{self, editor},
    state_viewer::{self, state_viewer},
    style,
    terminal::{self, terminal},
};
use crate::shared::vm::Computer;

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
    editor_content: text_editor::Content,
    input_content: String,
    computer: Computer,
    terminal_output: Vec<Box<str>>,
    terminal_error: String,
}

impl Default for State {
    fn default() -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::Editor);

        panes.split(pane_grid::Axis::Vertical, pane, Pane::StateViewer);
        panes.split(pane_grid::Axis::Horizontal, pane, Pane::Terminal);

        Self {
            panes,
            pane_focused: None,
            editor_content: text_editor::Content::new(),
            input_content: String::new(),
            computer: Computer::default(),
            terminal_output: Vec::new(),
            terminal_error: String::new(),
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

            Message::Editor(_) => todo!(),

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
                        Pane::Editor => {
                            editor(&self.editor_content, &self.input_content).map(Message::Editor)
                        }

                        Pane::StateViewer => state_viewer(&self.computer).map(Message::StateViewer),

                        Pane::Terminal => terminal(&self.terminal_output, &self.terminal_error)
                            .map(Message::Terminal),
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
