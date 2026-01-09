use std::{fs, io};

use crate::{
    backend::lessons::Parser,
    frontend::{
        screen::lesson_view,
        util::{font::Font, widgets::separator},
    },
};

use iced::{
    Color, Element, Length,
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
            lessons: lessons.map_err(
                |_| "Encountered an error while opening directory...\nFailed to read from lessons directory, are you sure the directory exists?",
            ),
        }
    }

    pub fn get_lessons(directory: String) -> io::Result<Vec<lesson_view::State>> {
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

                    lesson_view::State::new(
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

impl State {
    pub fn update(&self, message: Message) -> Option<Event> {
        match message {
            Message::BackClicked => Some(Event::ToMenu),
            Message::StartButtonClicked(lesson) => Some(Event::ToLessonView(lesson)),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        println!("{:?}", self.lessons);

        column![
            text("Lessons").font(Font::Bold).size(32),
            separator::horizontal(),
            // TODO: figure out a way to clone less
            column(
                self.lessons
                    .clone()
                    .map(|lessons| lessons
                        .iter()
                        .map(|lesson| column![
                            row![
                                text(lesson.title.clone()).font(Font::Bold).size(24),
                                space::horizontal(),
                                button("Start")
                                    .on_press(Message::StartButtonClicked(lesson.clone()))
                            ],
                            text(lesson.path.clone()).font(Font::Italic)
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
