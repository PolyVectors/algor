use std::{
    env,
    fs::{self, ReadDir},
    io,
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
    ConfigSaved,
    LessonsDirectoryChanged(settings::State, String),
    Runtime(runtime::Event),
    Step(Instant),
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
    lesson_viewer: Arc<Mutex<Computer>>,
    running: Option<Running>,
    input_needed: bool,
}

struct Algor {
    screen: Screen,
    // TODO: this probably should be an (a)rc
    config: Config,
    computers: Computers,
    sender: Option<Arc<Mutex<Sender<Input>>>>,
}

impl Default for Algor {
    fn default() -> Self {
        Self {
            screen: Screen::Menu(screen::menu::State),
            config: settings::State::from_screen(Box::new(Screen::Menu(screen::menu::State)))
                .into(),
            computers: Computers {
                sandbox: Arc::new(Mutex::new(Computer::default())),
                lesson_viewer: Arc::new(Mutex::new(Computer::default())),
                running: None,
                input_needed: false,
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
                            self.screen = Screen::Settings(settings::State::new(
                                self.config.clone(),
                                Box::new(self.screen.clone()),
                            ));
                        }
                        screen::Event::GoBack(screen) => {
                            self.screen = *screen;
                        }
                        screen::Event::ToSandbox => {
                            self.screen = Screen::Sandbox(screen::sandbox::State::new(
                                self.computers.sandbox.clone(),
                                // TODO: dont unwrap
                                self.sender.clone().unwrap(),
                            ))
                        }
                        screen::Event::ToLessonSelect => {
                            // TODO: dont unwrap
                            let lessons = screen::lesson_select::State::get_lessons(
                                self.config.lessons_directory.clone(),
                                self.computers.lesson_viewer.clone(),
                                self.sender.clone().unwrap(),
                            );
                            self.screen =
                                Screen::LessonSelect(screen::lesson_select::State::new(lessons))
                        }
                        screen::Event::Run => {
                            if let Screen::Sandbox(_) = self.screen {
                                self.computers.running = Some(Running::Sandbox);
                            }
                        }

                        screen::Event::Stop => self.computers.running = None,

                        screen::Event::Reset => {
                            self.computers.running = None;

                            if let Screen::Sandbox(state) = &mut self.screen {
                                state.error = String::new();
                                state.output = Vec::new();
                            }

                            if let Some(sender) = &mut self.sender
                                && let Ok(mut sender) = sender.lock()
                            {
                                sender.try_send(Input::Reset).unwrap();
                            }
                        }
                        screen::Event::SubmitInput(input) => {
                            if self.computers.input_needed
                                && let Some(sender) = &mut self.sender
                                && let Ok(mut sender) = sender.lock()
                            {
                                // TODO: dont unwrap
                                sender.try_send(Input::SetInput(input)).unwrap();

                                self.computers.running = Some(match self.screen {
                                    Screen::Sandbox(_) => Running::Sandbox,
                                    Screen::LessonView(_) => Running::Lesson,
                                    _ => unreachable!(),
                                });
                            }

                            self.computers.input_needed = false;
                        }
                    }
                }
            }
            Message::Runtime(event) => match event {
                runtime::Event::Ready(sender) => self.sender = Some(Arc::new(Mutex::new(sender))),

                // TODO: macro?
                runtime::Event::UpdateState(computer) => {
                    // TODO: should be a match statement when lesson viewer is done
                    if let Screen::Sandbox(state) = &mut self.screen {
                        state.computer = computer;
                    }
                }
                runtime::Event::SetError(error) => {
                    if let Screen::Sandbox(state) = &mut self.screen {
                        state.error = error;
                    }
                }
                runtime::Event::Output(output) => {
                    if let Screen::Sandbox(state) = &mut self.screen {
                        state.output.push(output)
                    }
                }
                runtime::Event::Input => {
                    self.computers.running = None;
                    self.computers.input_needed = true;

                    if let Screen::Sandbox(state) = &mut self.screen {
                        state.output.push("Waiting for input...".into());
                    }
                }

                runtime::Event::Halt => self.computers.running = None,
                runtime::Event::Continue => {}
            },
            Message::LessonsDirectoryChanged(mut state, directory) => {
                state.lessons_directory = directory;
                self.screen = Screen::Settings(state);
            }
            Message::ConfigSaved => {}
            Message::Step(_) => {
                if let Some(sender) = &mut self.sender
                    && let Ok(mut sender) = sender.lock()
                {
                    if matches!(self.screen, Screen::Sandbox(_) | Screen::LessonSelect(_)) {
                        sender.try_send(Input::Step).unwrap();
                    } else {
                        self.computers.running = None;
                    }
                }
            }
        }
        Task::none()
    }

    fn iced_theme(&self) -> iced::Theme {
        self.config.theme.clone().into()
    }
}
