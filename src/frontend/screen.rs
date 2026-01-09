use iced::Element;

use crate::backend::config::Config;

pub mod lesson_select;
pub mod lesson_view;
pub mod menu;
pub mod sandbox;
pub mod settings;

#[derive(Debug)]
pub enum Message {
    Menu(menu::Message),
    Settings(settings::Message),
    Sandbox(sandbox::Message),
    LessonSelect(lesson_select::Message),
    LessonView(lesson_view::Message),
}

pub enum Event {
    SetConfig(Config),
    PickLessonsDirectory(settings::State),
    ToSettings,
    GoBack(Box<Screen>),
    ToSandbox,
    ToLessonSelect,
    Run,
    Stop,
    Reset,
    SubmitInput(String),
}

#[derive(Debug, Clone)]
pub enum Screen {
    Menu(menu::State),
    LessonSelect(lesson_select::State),
    LessonView(lesson_view::State),
    Settings(settings::State),
    Sandbox(sandbox::State),
}

impl Screen {
    pub fn view(&self) -> Element<'_, Message> {
        match self {
            Screen::Menu(state) => state.view().map(Message::Menu),
            Screen::Settings(state) => state.view().map(Message::Settings),
            Screen::Sandbox(state) => state.view().map(Message::Sandbox),
            Screen::LessonSelect(state) => state.view().map(Message::LessonSelect),
            Screen::LessonView(state) => state.view().map(Message::LessonView),
        }
    }

    // TODO: maybe do a macro
    pub fn update(&mut self, message: Message) -> Option<Event> {
        match self {
            Screen::Menu(state) => {
                if let Message::Menu(message) = message {
                    if let Some(event) = state.update(message) {
                        match event {
                            menu::Event::ToLessonSelect => return Some(Event::ToLessonSelect),
                            menu::Event::ToSettings => return Some(Event::ToSettings),
                            menu::Event::ToSandbox => return Some(Event::ToSandbox),
                        }
                    }
                }
            }
            Screen::Settings(state) => {
                if let Message::Settings(message) = message {
                    if let Some(event) = state.update(message) {
                        match event {
                            settings::Event::SetConfig(state) => {
                                return Some(Event::SetConfig(state));
                            }
                            settings::Event::PickLessonsDirectory(state) => {
                                return Some(Event::PickLessonsDirectory(state));
                            }
                            settings::Event::GoBack(screen) => {
                                return Some(Event::GoBack(screen));
                            }
                        }
                    }
                }
            }
            Screen::Sandbox(state) => {
                if let Message::Sandbox(message) = message {
                    if let Some(event) = state.update(message) {
                        match event {
                            sandbox::Event::Run => return Some(Event::Run),
                            sandbox::Event::Stop => return Some(Event::Stop),
                            sandbox::Event::Reset => return Some(Event::Reset),

                            sandbox::Event::SubmitInput(input) => {
                                return Some(Event::SubmitInput(input));
                            }

                            sandbox::Event::ToMenu => {
                                *self = Screen::Menu(menu::State {});
                            }
                            sandbox::Event::ToSettings => return Some(Event::ToSettings),
                        }
                    }
                }
            }
            Screen::LessonSelect(state) => {
                if let Message::LessonSelect(message) = message {
                    if let Some(event) = state.update(message) {
                        match event {
                            lesson_select::Event::ToLessonView(lesson) => {
                                *self = Screen::LessonView(lesson)
                            }
                            lesson_select::Event::ToMenu => {
                                *self = Screen::Menu(menu::State {});
                            }
                        }
                    }
                }
            }
            Screen::LessonView(state) => {
                if let Message::LessonView(message) = message {
                    if let Some(event) = state.update(message) {
                        match event {
                            lesson_view::Event::ToLessonSelect => {
                                return Some(Event::ToLessonSelect);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
