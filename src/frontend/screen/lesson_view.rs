use std::sync::{Arc, Mutex};

use crate::{
    backend::lesson_parser::{self, Parser},
    frontend::pane::{
        editor::{self, editor},
        state_viewer::{self, state_viewer},
        style,
        terminal::{self, terminal},
    },
    shared::{runtime::Input, vm::Computer},
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
    Lesson(lesson_parser::Message),
    BackClicked,
    SettingsClicked,
}

pub enum Event {
    ToLessonSelect,
}

#[derive(Debug, Clone)]
pub enum Pane {
    Editor,
    StateViewer,
    Terminal,
    Lesson,
}

#[derive(Debug, Clone)]
pub struct State {
    pub title: String,
    pub path: String,
    pub slide_count: u8,
    panes: pane_grid::State<Pane>,
    pane_focused: Option<pane_grid::Pane>,
    content: text_editor::Content,
    pub computer: Arc<Mutex<Computer>>,
    sender: Arc<Mutex<Sender<Input>>>,
    pub output: Vec<Box<str>>,
    pub error: String,
}

impl State {
    pub fn new(
        title: String,
        path: String,
        slide_count: u8,
        computer: Arc<Mutex<Computer>>,
        sender: Arc<Mutex<Sender<Input>>>,
    ) -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::Editor);

        panes.split(pane_grid::Axis::Vertical, pane, Pane::Lesson);
        panes.split(pane_grid::Axis::Horizontal, pane, Pane::StateViewer);
        panes.split(pane_grid::Axis::Horizontal, pane, Pane::Terminal);

        Self {
            title,
            path,
            slide_count,
            panes,
            pane_focused: None,
            content: text_editor::Content::new(),
            computer,
            sender,
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
            Message::BackClicked => return Some(Event::ToLessonSelect),
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
                        Pane::Lesson => "Lesson",
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
                        Pane::Editor => editor(&self.content, &String::new()).map(Message::Editor),

                        Pane::StateViewer => {
                            // TODO: stop unwrapping
                            state_viewer(&self.computer.lock().unwrap()).map(Message::StateViewer)
                        }

                        Pane::Terminal => {
                            terminal(&self.output, &self.error).map(Message::Terminal)
                        }

                        Pane::Lesson => Parser::new(self.path.clone().into())
                            .unwrap()
                            .parse()
                            .unwrap()
                            .map(Message::Lesson),
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
