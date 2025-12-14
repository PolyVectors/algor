use iced::{Element, Settings, Subscription, Task};
use iced_aw::iced_fonts;

use algor::frontend::screen::{self, Screen, settings};
use algor::frontend::utils::font::{FAMILY_NAME, Font};

#[derive(Debug)]
enum Message {
    Screen(screen::Message),
}

fn main() -> iced::Result {
    iced::application("algor", Algor::update, Algor::view)
        .settings(Settings {
            fonts: vec![Font::Regular.into(), Font::Bold.into()],
            default_font: iced::Font::with_name(FAMILY_NAME),
            ..Settings::default()
        })
        .subscription(Algor::subscription)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .theme(Algor::iced_theme)
        .run_with(Algor::new)
}

struct Algor {
    screen: Screen,
    settings: settings::State,
}

impl Default for Algor {
    fn default() -> Self {
        Self {
            screen: Screen::Menu(screen::menu::State),
            settings: settings::State::default(),
        }
    }
}

impl Algor {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn view(&self) -> Element<'_, Message> {
        self.screen.view().map(Message::Screen)
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Screen(message) => {
                if let Some(event) = self.screen.update(message) {
                    match event {
                        screen::Event::SetSettings(state) => self.settings = state,
                    }
                }
            }
        }
        Task::none()
    }

    fn iced_theme(&self) -> iced::Theme {
        self.settings.theme.clone().into()
    }
}
