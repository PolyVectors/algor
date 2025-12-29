use iced::Element;

pub mod menu;
pub mod sandbox;
pub mod settings;

#[derive(Debug)]
pub enum Message {
    Menu(menu::Message),
    Settings(settings::Message),
}

pub enum Event {
    SetSettings(settings::State),
    ToSettings,
    ToSandbox,
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
    pub fn view<'a>(&self) -> Element<'a, Message> {
        match self {
            Screen::Menu(state) => state.view().map(Message::Menu),
            Screen::Settings(state) => state.view().map(Message::Settings),
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
                            // TODO: annoying clone
                            settings::Event::GoBack => {
                                *self = *state.last_screen.clone();
                                return None;
                            }
                            settings::Event::SetSettings(state) => {
                                return Some(Event::SetSettings(state));
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
