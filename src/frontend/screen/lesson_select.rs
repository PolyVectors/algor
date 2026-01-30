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

#[derive(Debug, Clone)]
pub struct State {
    lessons: Result<Vec<lesson_view::State>, &'static str>,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartButtonClicked(lesson_view::State),
    BackClicked,
}

pub enum Event {
    ToMenu,
    ToLessonView(lesson_view::State),
}

impl State {
    pub fn new(lessons: io::Result<Vec<lesson_view::State>>) -> Self {
        State {
            lessons: lessons.map_err(|_| {
                "Encountered an error while opening directory...\n\
                Failed to read from lessons directory, are you sure the directory exists?"
            }),
        }
    }

    pub fn get_lessons(
        directory: String,
        computer: Arc<Mutex<Computer>>,
        sender: Arc<Mutex<Sender<Input>>>,
        text_size: u32,
    ) -> io::Result<Vec<lesson_view::State>> {
        fs::read_dir(directory).map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| !entry.path().is_dir())
                .filter_map(|entry| {
                    let result = serde_xml_rs::from_reader(fs::File::open(entry.path()).unwrap());

                    if let Err(e) = &result {
                        println!(
                            "Error while parsing lesson XML at path {}...\n{e}",
                            entry.path().display()
                        );
                    }
                    result.ok()
                })
                .map(|lesson: Lesson| {
                    lesson_view::State::new(lesson, computer.clone(), sender.clone(), text_size)
                })
                .collect()
        })
    }
}

impl State {
    pub fn update(&self, message: Message) -> Option<Event> {
        match message {
            Message::BackClicked => Some(Event::ToMenu),
            Message::StartButtonClicked(lesson) => Some(Event::ToLessonView(lesson)),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        column![
            text("Lessons").font(Font::Bold).size(32),
            separator::horizontal(),
            // TODO: figure out a way to clone less
            column(
                self.lessons
                    .clone()
                    .map(|lessons| lessons
                        .iter()
                        .map(|state| column![
                            row![
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
                                space::horizontal(),
                                button("Start")
                                    .on_press(Message::StartButtonClicked(state.clone()))
                            ],
                            text(format!("{} slide(s)", state.lesson.body.slides.len()))
                                .font(Font::Italic)
                        ]
                        .spacing(8)
                        .into())
                        .collect::<Vec<_>>())
                    .unwrap_or_else(|e| vec![
                        text(e)
                            .style(|_| text::Style {
                                color: Some(Color::from_rgb(1f32, 0f32, 0f32))
                            })
                            .into()
                    ])
            ),
            space::vertical(),
            button("Back").on_press(Message::BackClicked)
        ]
        .height(Length::Fill)
        .width(Length::Fill)
        .spacing(16)
        .padding(12)
        .into()
    }
}
