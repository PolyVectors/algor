use std::env;
use std::sync::{Arc, RwLock};

use algor::backend::config::{self, Config};
use iced::{Element, Settings, Subscription, Task};
use iced_aw::iced_fonts;

use algor::frontend::screen::{self, Screen, settings};
use algor::frontend::util::font::{FAMILY_NAME, Font};

#[derive(Debug)]
enum Message {
    Screen(screen::Message),
    ConfigSaved,
    LessonsDirectoryChanged(Config, String),
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
    screen: Arc<RwLock<Screen>>,
    // TODO: this probably should be an rc
    config: Config,
}

impl Default for Algor {
    fn default() -> Self {
        Self {
            screen: Arc::new(RwLock::new(Screen::Menu(screen::menu::State))),
            config: settings::State::with_screen(Arc::new(RwLock::new(Screen::Menu(
                screen::menu::State,
            ))))
            .into(),
        }
    }
}

impl Algor {
    fn new() -> (Self, Task<Message>) {
        let mut path = env::home_dir().unwrap();
        path.push(config::CONFIG_PATH);

        (
            Self {
                config: Config::try_from(path).unwrap_or_default().into(),
                ..Default::default()
            },
            Task::none(),
        )
    }

    fn view(&self) -> Element<'_, Message> {
        match self.screen.read() {
            Ok(screen) => screen.view().map(Message::Screen),
            Err(e) => panic!("Failed to get screen for reading when viewing due to {e}"),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Screen(message) => {
                // TODO: unwrap
                let cloned = Arc::clone(&self.screen);
                let mut screen = cloned.write().unwrap();

                if let Some(event) = screen.update(message) {
                    match event {
                        screen::Event::SetConfig(config) => {
                            self.config = config;

                            let mut path = env::home_dir().unwrap();
                            path.push(config::CONFIG_PATH);

                            return Task::perform(self.config.clone().save(path), |_| {
                                Message::ConfigSaved
                            });
                        }
                        screen::Event::PickLessonsDirectory(state) => {
                            return Task::perform(settings::browse_directory(), move |directory| {
                                Message::LessonsDirectoryChanged(state.clone(), directory)
                            });
                        }
                        screen::Event::ToSettings => {
                            *screen = Screen::Settings(self.config.clone().into())
                        }
                        screen::Event::ToSandbox => {
                            *screen = Screen::Sandbox(screen::sandbox::State::default())
                        }
                        screen::Event::GoBack(screen) => {
                            self.screen = screen;
                        }
                    }
                }
            }
            Message::LessonsDirectoryChanged(mut state, directory) => {
                state.lessons_directory = directory;

                let mut screen = self.screen.write().unwrap();
                *screen = Screen::Settings(state.into());
            }
            Message::ConfigSaved => {}
        }
        Task::none()
    }

    fn iced_theme(&self) -> iced::Theme {
        self.config.theme.clone().into()
    }
}
