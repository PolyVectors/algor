use iced::Element;

pub mod menu;
pub mod settings;

#[derive(Debug)]
pub enum Message {
    Menu(menu::Message),
    Settings(settings::Message),
}

pub enum Event {
    SetSettings(settings::State),
}

#[derive(Debug, Clone)]
pub enum Screen {
    Menu(menu::State),
    LessonSelect,
    LessonView,
    Sandbox,
    Settings(settings::State),
}

impl Screen {
    pub fn view<'a>(&self) -> Element<'a, Message> {
        match self {
            Screen::Menu(state) => state.view().map(Message::Menu),
            Screen::Settings(state) => state.view().map(Message::Settings),
            _ => todo!(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match self {
            Screen::Menu(state) => {
                if let Message::Menu(message) = message {
                    if let Some(event) = state.update(message) {
                        match event {
                            menu::Event::SetScreen(screen) => *self = screen,
                        }
                    }
                }
            }
            Screen::Settings(state) => {
                if let Message::Settings(message) = message {
                    if let Some(event) = state.update(message) {
                        match event {
                            settings::Event::SetScreen(screen) => {
                                *self = screen;
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
