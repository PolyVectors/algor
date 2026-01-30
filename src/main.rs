use std::{
    env, fs,
    io::Write,
    sync::{Arc, Mutex},
    time::Instant,
};

use algor::{
    backend::config::{self, Config},
    shared::runtime,
};
use algor::{frontend::pane::editor, shared::runtime::Input};
use algor::{frontend::screen::sandbox, shared::vm::Computer};

use iced::{Element, Settings, Subscription, Task, time};
use iced::{futures::channel::mpsc::Sender, widget::text_editor};

use algor::frontend::screen::{self, Screen, settings};
use algor::frontend::util::font::{FAMILY_NAME, Font};

#[derive(Debug)]
enum Message {
    Screen(screen::Message),
    ConfigSaved,
    LessonsDirectoryChanged(settings::State, String),

    SetContent(sandbox::State, Option<String>),
    SaveContent(sandbox::State, Option<String>),

    Runtime(runtime::Event),

    #[allow(dead_code)]
    Step(Instant),
}

fn main() -> iced::Result {
    iced::application(Algor::new, Algor::update, Algor::view)
        .settings(Settings {
            fonts: vec![Font::Regular.into(), Font::Bold.into(), Font::Italic.into()],
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

                        screen::Event::OpenLMC(state) => {
                            return Task::perform(editor::open_lmc(), move |path| {
                                Message::SetContent(state.clone(), path)
                            });
                        }
                        screen::Event::SaveLMC(state) => {
                            return Task::perform(editor::open_lmc(), move |path| {
                                Message::SaveContent(state.clone(), path)
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
                                self.sender.clone().unwrap(),
                                self.config.editor_font_size,
                            ))
                        }
                        screen::Event::ToLessonSelect => {
                            let lessons = screen::lesson_select::State::get_lessons(
                                self.config.lessons_directory.clone(),
                                self.computers.lesson_viewer.clone(),
                                self.sender.clone().unwrap(),
                                self.config.editor_font_size,
                            );
                            self.screen =
                                Screen::LessonSelect(screen::lesson_select::State::new(lessons))
                        }

                        screen::Event::Run => match &mut self.screen {
                            Screen::LessonView(state) => {
                                state.input = 0;
                                state.output = Vec::new();
                                self.computers.running = Some(Running::Lesson)
                            }
                            Screen::Sandbox(state) => {
                                state.output = Vec::new();
                                self.computers.running = Some(Running::Sandbox)
                            }
                            _ => unreachable!(),
                        },
                        screen::Event::Stop => self.computers.running = None,
                        screen::Event::Reset => {
                            self.computers.running = None;

                            match &mut self.screen {
                                Screen::LessonView(state) => {
                                    state.input = 0;
                                    state.error = String::new();
                                    state.output = Vec::new();
                                }
                                Screen::Sandbox(state) => {
                                    state.error = String::new();
                                    state.output = Vec::new();
                                }
                                _ => unreachable!(),
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
                runtime::Event::UpdateState(computer) => match &mut self.screen {
                    Screen::LessonView(state) => state.computer = computer,
                    Screen::Sandbox(state) => state.computer = computer,
                    _ => unreachable!(),
                },
                runtime::Event::SetError(error) => match &mut self.screen {
                    Screen::LessonView(state) => {
                        self.computers.running = None;
                        state.error = error
                    }
                    Screen::Sandbox(state) => state.error = error,
                    _ => unreachable!(),
                },
                runtime::Event::Output(output) => match &mut self.screen {
                    Screen::LessonView(state) => state.output.push(output),
                    Screen::Sandbox(state) => state.output.push(output),
                    _ => unreachable!(),
                },
                runtime::Event::Input => {
                    self.computers.input_needed = true;

                    match &mut self.screen {
                        Screen::LessonView(state) => {
                            if let Ok(mut computer) = state.computer.lock() {
                                computer.accumulator = *state
                                    .lesson
                                    .body
                                    .slides
                                    .get(state.slide)
                                    .unwrap()
                                    .inputs
                                    .items
                                    .get(state.input)
                                    .unwrap_or(&0i16);

                                state.input += 1;
                            }
                        }
                        Screen::Sandbox(state) => {
                            self.computers.running = None;
                            state.output.push("Waiting for input...".into())
                        }
                        _ => unreachable!(),
                    }
                }

                runtime::Event::Halt => self.computers.running = None,
                runtime::Event::Continue => {}
            },

            Message::LessonsDirectoryChanged(mut state, directory) => {
                state.lessons_directory = directory;
                self.screen = Screen::Settings(state);
            }

            Message::SetContent(mut state, path) => {
                if let Some(path) = path {
                    let text = fs::read_to_string(path).unwrap_or_default();

                    state.content = text_editor::Content::with_text(text.as_str());
                    self.screen = Screen::Sandbox(state);
                }
            }

            Message::SaveContent(state, path) => {
                if let Some(path) = path {
                    if let Ok(mut file) = fs::File::create(path) {
                        if let Err(e) = file.write_all(state.content.text().as_bytes()) {
                            panic!("Failed to write to file: {e}");
                        }
                    }
                }
            }

            Message::ConfigSaved => {}

            Message::Step(_) => {
                if let Some(sender) = &mut self.sender
                    && let Ok(mut sender) = sender.lock()
                {
                    if matches!(self.screen, Screen::Sandbox(_) | Screen::LessonView(_)) {
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
