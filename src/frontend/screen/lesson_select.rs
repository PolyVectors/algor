use std::{
    fs, io,
    sync::{Arc, Mutex},
};

use crate::{
    backend::lesson_parser::Lesson,
    frontend::{
        screen::lesson_view,
        util::{font::Font, widgets::separator},
    },
    shared::{runtime::Input, vm::Computer},
};

use iced::{
    Color, Element, Length,
    futures::channel::mpsc::Sender,
    widget::{button, column, row, space, text},
};

// Messages specific to the lesson select screen
#[derive(Debug, Clone)]
pub enum Message {
    StartButtonClicked(lesson_view::State),
    BackClicked,
}

// Events specific to the lesson select screen
pub enum Event {
    ToMenu,
    ToLessonView(lesson_view::State),
}

// A list of lesson viewer screen states, or an error message
#[derive(Debug, Clone)]
pub struct State {
    lessons: Result<Vec<lesson_view::State>, &'static str>,
}

impl State {
    /* Convert I/O error from get_lesssons associated function to user-friendly string and wrap lessons around state
    As this code is not chained together with other fallible code, I don't need to implement Error and can have the error type just be a static string */
    pub fn new(lessons: io::Result<Vec<lesson_view::State>>) -> Self {
        State {
            lessons: lessons.map_err(|_| {
                "Encountered an error while opening directory...\n\
                Failed to read from lessons directory, are you sure the directory exists?"
            }),
        }
    }

    // Get a list of lessons from a directory. if a lesson fails to be parsed (i.e. isn't valid XML, or isn't XML), print an error in the terminal for the author of the lesson (teacher) to debug
    pub fn get_lessons(
        directory: String,
        computer: Arc<Mutex<Computer>>,
        sender: Arc<Mutex<Sender<Input>>>,
        text_size: u32,
    ) -> io::Result<Vec<lesson_view::State>> {
        // Try and read from the directory, if this fails, the map method won't run and the error will be bubbled up to the constructor
        fs::read_dir(directory).map(|entries| {
            // Start by reading the directory
            entries
                // Take out any entries that cannot be read (i.e. files where the user lacks sufficient permissions, etc.)
                .filter_map(|entry| entry.ok())
                // Take out any entries that are directories, this search operation is not recursive
                .filter(|entry| !entry.path().is_dir())
                // Take out any entries that are not valid XML, if there is valid XML, replace the file information with the parsed Lesson struct
                .filter_map(|entry| {
                    let result = serde_xml_rs::from_reader(fs::File::open(entry.path()).unwrap());

                    // Debug message for lesson author
                    if let Err(e) = &result {
                        eprintln!(
                            "Error while parsing lesson XML at path {}...\n{e}",
                            entry.path().display()
                        );
                    }
                    result.ok()
                })
                // Turn Lesson struct into lesson viewer screen state
                .map(|lesson: Lesson| {
                    lesson_view::State::new(lesson, computer.clone(), sender.clone(), text_size)
                })
                // Convert iterator into Vec<lesson_view::State>
                .collect()
        })
    }
}

impl State {
    pub fn update(&self, message: Message) -> Option<Event> {
        match message {
            // When the user clicks back, return an event to send them to the menu
            Message::BackClicked => Some(Event::ToMenu),
            // Whem the user clicks start on a lesson, send them to that lesson
            Message::StartButtonClicked(lesson) => Some(Event::ToLessonView(lesson)),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            // Title text
            text("Lessons").font(Font::Bold).size(32),
            separator::horizontal(),
            column(
                self.lessons
                    .clone()
                    .map(|lessons| lessons
                        .iter()
                        // For every lesson, show lesson information and a start button
                        .map(|state| column![
                            row![
                                // Show the lesson title in large bold text
                                text(
                                    state
                                        .clone()
                                        .lesson
                                        .head
                                        .title
                                        .unwrap_or(String::from("Untitled Lesson"))
                                )
                                .font(Font::Bold)
                                .size(24),
                                // Leave the widest possible horizontal gap between the previous and next element
                                space::horizontal(),
                                // Start button bundling in the lesson state as a tuple struct
                                button("Start")
                                    .on_press(Message::StartButtonClicked(state.clone()))
                            ],
                            // Show the amount of slides
                            text(format!("{} slide(s)", state.lesson.body.slides.len()))
                                .font(Font::Italic)
                        ]
                        .spacing(8)
                        .into())
                        .collect::<Vec<_>>())
                    // If there was an error opening the directory, show it in red text instead of showing the list of columns
                    .unwrap_or_else(|e| vec![
                        text(e)
                            .style(|_| text::Style {
                                color: Some(Color::from_rgb(1f32, 0f32, 0f32))
                            })
                            .into()
                    ])
            )
            .spacing(16),
            // Leave as much vertical space between the previous and next element
            space::vertical(),
            // Back button (bottom left corner)
            button("Back").on_press(Message::BackClicked)
        ]
        .height(Length::Fill)
        .width(Length::Fill)
        .spacing(16)
        .padding(12)
        .into()
    }
}
