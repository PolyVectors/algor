use std::{fs, io};

use crate::{
    backend::lessons::Parser,
    frontend::util::{font::Font, widgets::separator},
};

use iced::{
    Color, Element, Length,
    widget::{button, column, space, text},
};

#[derive(Debug, Clone)]
pub struct Lesson {
    title: String,
    path: String,
    slide_count: u8,
}

impl Lesson {
    pub fn new(title: String, path: String, slide_count: u8) -> Self {
        Lesson {
            title,
            path,
            slide_count,
        }
    }

    pub fn get_lessons(directory: String) -> io::Result<Vec<Self>> {
        match fs::read_dir(directory) {
            Ok(entries) => Ok(entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| !entry.path().is_dir())
                .map(|entry| {
                    let title = Parser::new(entry.path())
                        .unwrap()
                        .parse_head()
                        .unwrap_or_default()
                        .title;

                    Lesson::new(
                        title,
                        entry
                            .path()
                            .into_os_string()
                            .into_string()
                            .unwrap_or_default(),
                        0,
                    )
                })
                .collect()),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackClicked,
}

pub enum Event {
    ToMenu,
}

#[derive(Debug, Clone)]
pub struct State {
    lessons: Result<Vec<Lesson>, &'static str>,
}

impl State {
    pub fn new(lessons: io::Result<Vec<Lesson>>) -> Self {
        State {
            lessons: lessons.map_err(
                |_| "Encountered an error while opening directory...\nFailed to read from lessons directory, are you sure the directory exists?",
            ),
        }
    }
}

impl State {
    pub fn update(&self, message: Message) -> Option<Event> {
        match message {
            Message::BackClicked => Some(Event::ToMenu),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        println!("{:?}", self.lessons);

        column![
            text("Lessons").font(Font::Bold).size(32),
            separator::horizontal(),
            column(
                self.lessons
                    .clone()
                    .map(|lessons| lessons
                        .iter()
                        .map(|lesson| text(lesson.title.clone()).into())
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
