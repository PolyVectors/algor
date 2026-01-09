use std::{fs, io};

use crate::frontend::util::{font::Font, widgets::separator};

use iced::{
    Element, Length,
    widget::{button, column, space, text},
};

#[derive(Debug, Clone)]
pub enum Message {
    BackClicked,
}

pub enum Event {
    ToMenu,
}

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
}

#[derive(Debug, Clone)]
pub struct State {
    lessons: Result<Vec<Lesson>, &'static str>,
}

impl State {
    pub fn new(lessons: io::Result<Vec<Lesson>>) -> Self {
        State {
            lessons: lessons.map_err(
                |_| "Failed to read from lessons directory.\nAre you sure the directory exists?",
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
