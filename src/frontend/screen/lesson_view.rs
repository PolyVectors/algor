use std::sync::{Arc, Mutex};

use crate::{
    backend::lesson_parser::Lesson,
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

// Messages specific to lesson view screen
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
    // Back button in lesson pane clicked
    BackLessonClicked,
    // Next button in lesson pane clicked
    NextLessonClicked,
    BackClicked,
    SettingsClicked,
}

// Events specific to lesson view screen
pub enum Event {
    // Result from clicking run button in editor pane
    Run,
    // Ditto for stop button
    Stop,
    // Ditto for reset button
    Reset,
    // Result from clicking back button
    ToLessonSelect,
    // Result from clicking settings button
    ToSettings,
}

// Represents individual panes in the screen
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
    // Whether or not a lesson has been completed
    pub completed: bool,
    // The current slide (of a lesson) the user is on
    pub slide: usize,
    // The index of the next input
    pub input: usize,
    panes: pane_grid::State<Pane>,
    // Current pane the user is interacting with
    pane_focused: Option<pane_grid::Pane>,
    // Text content of the text editor pane
    content: text_editor::Content,
    pub text_size: u32,
    pub computer: Arc<Mutex<Computer>>,
    sender: Arc<Mutex<Sender<Input>>>,
    // Terminal pane outputs
    pub output: Vec<Box<str>>,
    // Terminal pane error
    pub error: String,
}

impl State {
    // Constructor for the lesson viewer
    pub fn new(
        lesson: Lesson,
        computer: Arc<Mutex<Computer>>,
        sender: Arc<Mutex<Sender<Input>>>,
        text_size: u32,
    ) -> Self {
        // Start with editor pane
        let (mut panes, pane) = pane_grid::State::new(Pane::Editor);

        // Split vertically (editor pane on left, lesson pane on the right)
        panes.split(pane_grid::Axis::Vertical, pane, Pane::Lesson);
        // Split horizontally (editor pane on top, state viewer pane below, lesson pane on the right)
        panes.split(pane_grid::Axis::Horizontal, pane, Pane::StateViewer);
        // Split horizontally again (editor pane on top, followed by terminal and state viewer below, lesson pane on the right)
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

                editor::Message::AssembleClicked => {
                    self.error = String::new();

                    // Send a message to the sender to compile the program with the source code in the text editor
                    if let Ok(mut sender) = self.sender.lock() {
                        sender
                            .try_send(Input::AssembleClicked(self.content.text()))
                            .unwrap()
                    }
                }

                editor::Message::ResetClicked => return Some(Event::Reset),
                editor::Message::StopClicked => return Some(Event::Stop),
                editor::Message::RunClicked => return Some(Event::Run),

                _ => {}
            },

            Message::SettingsClicked => return Some(Event::ToSettings),
            Message::BackClicked => return Some(Event::ToLessonSelect),

            Message::NextLessonClicked => {
                // Only allow the user to progress to the next lesson if the outputs match the list of outputs defined in the lesson
                if self
                    .output
                    .iter()
                    .map(|x| x.parse::<i16>().unwrap_or(0))
                    .collect::<Vec<i16>>()
                    == self.lesson.body.slides[self.slide].outputs.items
                    && self.error == String::new()
                {
                    // Complete the lesson if there are no more slides
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
                pane_grid(&self.panes, |pane, state, _is_maximized| {
                    let focused = self.pane_focused == Some(pane);

                    // Convert Pane to static string
                    let title = match state {
                        Pane::Editor => "Editor",
                        Pane::StateViewer => "State Viewer",
                        Pane::Terminal => "Terminal",
                        Pane::Lesson => "Lesson",
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
                        Pane::Editor => {
                            editor(&self.content, self.text_size, None).map(Message::Editor)
                        }
                        Pane::StateViewer => {
                            state_viewer(&self.computer.lock().unwrap()).map(Message::StateViewer)
                        }
                        Pane::Terminal => {
                            terminal(&self.output, &self.error).map(Message::Terminal)
                        }

                        Pane::Lesson => column![
                            // Show lesson slide content with navigation buttons if the lesson is not completed
                            (!self.completed).then(|| {
                                container(column![
                                    // Turn slide into Element<'_, Message> via parse method in src/backend/lesson_parser.rs
                                    self.lesson.body.slides[self.slide].parse(),
                                    space::vertical(),
                                    row![
                                        button("Back").on_press(Message::BackLessonClicked),
                                        space::horizontal(),
                                        button("Next").on_press(Message::NextLessonClicked)
                                    ]
                                ])
                            }),
                            // ... otherwise show some text indicating the lesson is complete
                            self.completed
                                .then(|| { text("Lesson Completed!").font(Font::Bold).size(24) }),
                        ]
                        // Padding as to not cause elements to appear behind the title bar
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
