use std::env;
use std::path::PathBuf;

use algor::backend::config::{self, Config};
use iced::{Element, Settings, Subscription, Task};
use iced_aw::iced_fonts;

use algor::frontend::screen::{self, Screen, settings};
use algor::frontend::util::font::{FAMILY_NAME, Font};
use rfd::AsyncFileDialog;

#[derive(Debug)]
enum Message {
    Screen(screen::Message),
    SettingsSaved,
    LessonsDirectoryChanged(settings::State, String),
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

async fn browse_directory() -> String {
    let mut config_dir = env::home_dir().unwrap();
    config_dir.push(config::CONFIG_PATH);

    let config = Config::try_from(config_dir).unwrap_or_default();

    // TODO: too many unwraps, return error and use ? operator
    AsyncFileDialog::new()
        .set_title("Pick lessons directory...")
        .pick_folder()
        .await
        .unwrap_or(PathBuf::from(&config.lessons_directory).into())
        .path()
        .to_str()
        .to_owned()
        .unwrap()
        .to_owned()
}

struct Algor {
    screen: Screen,
    // TODO: this probably should be an rc
    settings: settings::State,
}

impl Default for Algor {
    fn default() -> Self {
        Self {
            screen: Screen::Menu(screen::menu::State),
            settings: settings::State::with_screen(Screen::Menu(screen::menu::State)),
        }
    }
}

impl Algor {
    fn new() -> (Self, Task<Message>) {
        let mut path = env::home_dir().unwrap();
        path.push(config::CONFIG_PATH);

        (
            Self {
                settings: Config::try_from(path).unwrap_or_default().into(),
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
                        screen::Event::SetSettings(state) => {
                            self.settings = state;

                            let mut path = env::home_dir().unwrap();
                            path.push(config::CONFIG_PATH);

                            return Task::perform(
                                <settings::State as Into<Config>>::into(self.settings.clone())
                                    .save(path),
                                |_| Message::SettingsSaved,
                            );
                        }
                        screen::Event::PickLessonsDirectory(state) => {
                            return Task::perform(browse_directory(), move |directory| {
                                Message::LessonsDirectoryChanged(state.clone(), directory)
                            });
                        }
                        screen::Event::ToSettings => {
                            self.screen = Screen::Settings(self.settings.clone())
                        }
                        screen::Event::ToSandbox => {
                            self.screen = Screen::Sandbox(screen::sandbox::State::default())
                        }
                    }
                }
            }
            Message::LessonsDirectoryChanged(mut state, directory) => {
                state.lessons_directory = directory;
                self.screen = Screen::Settings(state.clone())
            }
            Message::SettingsSaved => {}
        }
        Task::none()
    }

    fn iced_theme(&self) -> iced::Theme {
        self.settings.theme.clone().into()
    }
}
