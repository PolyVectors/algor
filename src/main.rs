use std::env;

use algor::backend::config::{self, Config};
use iced::{Element, Settings, Subscription, Task};

use algor::frontend::screen::{self, Screen, settings};
use algor::frontend::util::font::{FAMILY_NAME, Font};

#[derive(Debug)]
enum Message {
    Screen(screen::Message),
    ConfigSaved,
    LessonsDirectoryChanged(Config, String),
}

fn main() -> iced::Result {
    iced::application(Algor::new, Algor::update, Algor::view)
        .settings(Settings {
            fonts: vec![Font::Regular.into(), Font::Bold.into()],
            default_font: iced::Font::with_name(FAMILY_NAME),
            ..Settings::default()
        })
        .title("algor")
        .subscription(Algor::subscription)
        .font(Font::Regular)
        .theme(Algor::iced_theme)
        .run()
}

struct Algor {
    screen: Screen,
    // TODO: this probably should be an (a)rc
    config: Config,
}

impl Default for Algor {
    fn default() -> Self {
        Self {
            screen: Screen::Menu(screen::menu::State),
            config: settings::State::with_screen(Box::new(Screen::Menu(screen::menu::State)))
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
                            self.screen = Screen::Settings(self.config.clone().into())
                        }
                        screen::Event::ToSandbox => {
                            self.screen = Screen::Sandbox(screen::sandbox::State::default())
                        }
                        screen::Event::GoBack(screen) => {
                            self.screen = *screen;
                        }
                    }
                }
            }
            Message::LessonsDirectoryChanged(mut state, directory) => {
                state.lessons_directory = directory;
                self.screen = Screen::Settings(state.into());
            }
            Message::ConfigSaved => {}
        }
        Task::none()
    }

    fn iced_theme(&self) -> iced::Theme {
        self.config.theme.clone().into()
    }
}
