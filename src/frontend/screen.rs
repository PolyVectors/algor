use iced::Element;

use crate::backend::config::Config;

pub mod menu;
pub mod sandbox;
pub mod settings;

#[derive(Debug)]
pub enum Message {
    Menu(menu::Message),
    Settings(settings::Message),
    Sandbox(sandbox::Message),
}

pub enum Event {
    SetConfig(Config),
    PickLessonsDirectory(Config),
    ToSettings,
    ToSandbox,
    GoBack(Box<Screen>),
}

#[derive(Debug, Clone)]
pub enum Screen {
    Menu(menu::State),
    LessonSelect,
    LessonView,
    Settings(settings::State),
    Sandbox(sandbox::State),
}

impl Screen {
    pub fn view(&self) -> Element<'_, Message> {
        match self {
            Screen::Menu(state) => state.view().map(Message::Menu),
            Screen::Settings(state) => state.view().map(Message::Settings),
            Screen::Sandbox(state) => state.view().map(Message::Sandbox),
            _ => todo!(),
        }
    }

    // TODO: maybe do a macro
    pub fn update(&mut self, message: Message) -> Option<Event> {
        match self {
            Screen::Menu(state) => {
                if let Message::Menu(message) = message {
                    if let Some(event) = state.update(message) {
                        match event {
                            menu::Event::ToLessonView => {
                                *self = Screen::LessonView;
                            }
                            // TODO: maybe map these two
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
            _ => todo!(),
        }
        None
    }
}
