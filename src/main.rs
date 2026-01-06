use std::{
    env,
    sync::{Arc, Mutex},
    time::Instant,
};

use algor::shared::runtime::Input;
use algor::shared::vm::Computer;
use algor::{
    backend::config::{self, Config},
    shared::runtime,
};

use iced::futures::channel::mpsc::Sender;
use iced::{Element, Settings, Subscription, Task, time};

use algor::frontend::screen::{self, Screen, settings};
use algor::frontend::util::font::{FAMILY_NAME, Font};

#[derive(Debug)]
enum Message {
    Screen(screen::Message),
    Runtime(runtime::Event),
    Step(Instant),
    ConfigSaved,
    LessonsDirectoryChanged(settings::State, String),
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

enum Running {
    Sandbox,
    Lesson,
}

struct Computers {
    sandbox: Arc<Mutex<Computer>>,
    lesson: Arc<Mutex<Computer>>,
    running: Option<Running>,
}

struct Algor {
    screen: Screen,
    // TODO: this probably should be an (a)rc
    config: Config,
    computers: Computers,
    sender: Option<Sender<Input>>,
}

impl Default for Algor {
    fn default() -> Self {
        Self {
            screen: Screen::Menu(screen::menu::State),
            config: settings::State::with_last_screen(Box::new(Screen::Menu(screen::menu::State)))
                .into(),
            computers: Computers {
                sandbox: Arc::new(Mutex::new(Computer::default())),
                lesson: Arc::new(Mutex::new(Computer::default())),
                running: None,
            },
            sender: None,
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
        let run = Subscription::run(runtime::run).map(Message::Runtime);

        let step = if let Some(_) = self.computers.running {
            time::every(self.config.run_speed.into()).map(Message::Step)
        } else {
            Subscription::none()
        };

        Subscription::batch(vec![run, step])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Screen(message) => {
                if let Some(event) = self.screen.update(message) {
                    match event {
                        screen::Event::SetConfig(config) => {
                            self.config = config;

                            // TODO: stop unwrapping
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
                            self.screen = Screen::Settings(settings::State::from_config(
                                self.config.clone(),
                                Box::new(self.screen.clone()),
                            ));
                        }
                        screen::Event::ToSandbox => {
                            self.screen = Screen::Sandbox(screen::sandbox::State::from_computer(
                                self.computers.sandbox.clone(),
                            ))
                        }
                        screen::Event::GoBack(screen) => {
                            self.screen = *screen;
                        }
                    }
                }
            }
            Message::Runtime(event) => match event {
                runtime::Event::Ready(sender) => self.sender = Some(sender),
                _ => {}
            },
            Message::LessonsDirectoryChanged(mut state, directory) => {
                state.lessons_directory = directory;
                self.screen = Screen::Settings(state);
            }
            Message::ConfigSaved => {}
            Message::Step(_) => {}
        }
        Task::none()
    }

    fn iced_theme(&self) -> iced::Theme {
        self.config.theme.clone().into()
    }
}
