use std::sync::{Arc, Mutex};

use crate::{
    backend::lesson_parser::{self, Lesson},
    frontend::{
        pane::{
            editor::{self, editor},
            state_viewer::{self, state_viewer},
            style,
            terminal::{self, terminal},
        },
        util::font::Font,
    },
    shared::{runtime::Input, vm::Computer},
};

use iced::{
    Element, Padding,
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
    BackLessonClicked,
    NextLessonClicked,
    BackClicked,
    SettingsClicked,
}

pub enum Event {
    Run,
    Stop,
    Reset,
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
    pub lesson: Lesson,
    pub completed: bool,
    pub slide: usize,
    pub input: usize,
    panes: pane_grid::State<Pane>,
    pane_focused: Option<pane_grid::Pane>,
    content: text_editor::Content,
    text_size: u32,
    pub computer: Arc<Mutex<Computer>>,
    sender: Arc<Mutex<Sender<Input>>>,
    pub output: Vec<Box<str>>,
    pub error: String,
}

impl State {
    pub fn new(
        lesson: Lesson,
        computer: Arc<Mutex<Computer>>,
        sender: Arc<Mutex<Sender<Input>>>,
        text_size: u32,
    ) -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::Editor);

        panes.split(pane_grid::Axis::Vertical, pane, Pane::Lesson);
        panes.split(pane_grid::Axis::Horizontal, pane, Pane::StateViewer);
        panes.split(pane_grid::Axis::Horizontal, pane, Pane::Terminal);

        Self {
            lesson,
            completed: false,
            slide: 0,
            input: 0,
            panes,
            pane_focused: None,
            content: text_editor::Content::new(),
            text_size,
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

            Message::Editor(message) => match message {
                editor::Message::ContentChanged(action) => self.content.perform(action),

                editor::Message::AssembleClicked => {
                    self.error = String::new();

                    if let Ok(mut sender) = self.sender.lock() {
                        // TODO: stop unwrapping
                        sender
                            .try_send(Input::AssembleClicked(self.content.text()))
                            .unwrap()
                    }
                }

                editor::Message::ResetClicked => return Some(Event::Reset),
                editor::Message::StopClicked => return Some(Event::Stop),
                editor::Message::RunClicked => return Some(Event::Run),

                _ => todo!(),
            },

            Message::BackClicked => return Some(Event::ToLessonSelect),

            Message::NextLessonClicked => {
                if self
                    .output
                    .iter()
                    .map(|x| x.parse::<i16>().unwrap_or(0))
                    .collect::<Vec<i16>>()
                    == self.lesson.body.slides[self.slide].outputs.items
                    && self.error == String::new()
                {
                    if self.slide < self.lesson.body.slides.len() - 1 {
                        self.slide += 1
                    } else {
                        self.completed = true
                    }
                }
            }
            Message::BackLessonClicked => {
                if self.slide != 0 {
                    self.slide -= 1
                }
            }

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
                        Pane::Editor => {
                            editor(&self.content, self.text_size, None).map(Message::Editor)
                        }

                        Pane::StateViewer => {
                            // TODO: stop unwrapping
                            state_viewer(&self.computer.lock().unwrap()).map(Message::StateViewer)
                        }

                        Pane::Terminal => {
                            terminal(&self.output, &self.error).map(Message::Terminal)
                        }

                        Pane::Lesson => column![
                            (!self.completed).then(|| {
                                container(column![
                                    self.lesson.body.slides[self.slide].parse(),
                                    space::vertical(),
                                    row![
                                        button("Back").on_press(Message::BackLessonClicked),
                                        space::horizontal(),
                                        button("Next").on_press(Message::NextLessonClicked)
                                    ]
                                ])
                            }),
                            self.completed
                                .then(|| { text("Lesson Completed!").font(Font::Bold).size(24) }),
                        ]
                        .padding(Padding {
                            left: 8f32,
                            top: 6f32,
                            right: 8f32,
                            bottom: 6f32,
                        })
                        .into(),
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
