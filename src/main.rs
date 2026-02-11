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

// The enum that defines the type of events that can happen during the execution of the program
#[derive(Debug)]
enum Message {
    // These are messages relating to specific sections/menus (screens) of the application
    Screen(screen::Message),
    // The message that gets bubbled up when the user saves their config
    ConfigSaved,
    // The message that gets bubbled up when the user changes the lessons directory via the browse button
    LessonsDirectoryChanged(settings::State, String),

    // The messages that get bubbleed up when the user tries to open/save an LMC file
    SetContent(sandbox::State, Option<String>),
    SaveContent(sandbox::State, Option<String>),

    // Messages relating to the execution of code in the virtual machine
    Runtime(runtime::Event),

    // A message that occurs every time the state of the virtual machine is updated
    #[allow(dead_code)]
    Step(Instant),
}

/* The entry point of the application:
- Specifies the new (constructor), update (message/event handler), and view (UI displayed to user) functions to be that of the implementations in the main struct (Algor)
- Loads the fonts and sets the default font family
- Sets the title of the application shown in the OS's window manager to "algor"
- Indicates the subscription function to be that of the implementation in the main struct (for bidirectional communication between the backend and frontend)
- Sets the default font style to Regular (non-bold and non-italic)
- Sets the theme function to the iced_them method of the Algor struct
- Runs the application */
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

// Enum representing what computer is running
enum Running {
    Sandbox,
    Lesson,
}

// The sandbox and lesson viewer computers with information about what computer is running (None for no computers running) and information for if input is required (as to prevent the user from changing the value in the accumulator at any time)
struct Computers {
    sandbox: Arc<Mutex<Computer>>,
    lesson_viewer: Arc<Mutex<Computer>>,
    running: Option<Running>,
    input_needed: bool,
}

// The main struct of the program, represents all of the important data stored in RAM
struct Algor {
    // The current menu the user is in
    screen: Screen,
    // The user's configuration
    config: Config,
    // The virtual machines of the Sandbox and Lesson View screens
    computers: Computers,
    // The shared interior-mutable interface for asynchronous, biderectional communication between the frontend and backend
    sender: Option<Arc<Mutex<Sender<Input>>>>,
}

// Allow the ability to create a new program
impl Default for Algor {
    fn default() -> Self {
        Self {
            screen: Screen::Menu(screen::menu::State),
            config: Config::default(),
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
    // Create a new program by deserialising the user config and falling back to the default config if necessary (see src/backend/config.rs)
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

    // Show the current screen
    fn view(&self) -> Element<'_, Message> {
        self.screen.view().map(Message::Screen)
    }

    // Updates the state of the virtual machine
    fn subscription(&self) -> Subscription<Message> {
        // Initialise the sender
        let run = Subscription::run(runtime::run).map(Message::Runtime);

        // Send an update message at the rate of the run speed if there is a computer running
        let step = if let Some(_) = self.computers.running {
            time::every(self.config.run_speed.into()).map(Message::Step)
        } else {
            Subscription::none()
        };

        // Bundle the subscriptions to be ran at the same time
        Subscription::batch(vec![run, step])
    }

    // Updates the state of the application
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Screen-related messages
            Message::Screen(message) => {
                if let Some(event) = self.screen.update(message) {
                    match event {
                        // Save the config to the default path (dependant on the operating system)
                        screen::Event::SetConfig(config) => {
                            self.config = config;

                            let mut path = env::home_dir().unwrap();
                            path.push(config::CONFIG_PATH);

                            /* Use a task to run an asynchronous function that saves the config, ensuring the program doesn't freeze while the file is being serialised and written to disk
                            If the function wasn't asynchronous, the program could hang and cause a "... is not responding" popup on Windows and potentially macOS and potentially cause a "Hall of Mirrors" effect when other windows pass above this window on other unix-like operating systems due to the framebuffer not updating properly */
                            return Task::perform(self.config.clone().save(path), |_| {
                                Message::ConfigSaved
                            });
                        }

                        // Pick the lessons directory and send another message with the path
                        screen::Event::PickLessonsDirectory(state) => {
                            // Ditto SetConfig comment but while the user is searching for a directory instead
                            return Task::perform(settings::browse_directory(), move |directory| {
                                Message::LessonsDirectoryChanged(state.clone(), directory)
                            });
                        }

                        // Open from a *.lmc file, taking ownership of the path returned
                        screen::Event::OpenLMC(state) => {
                            return Task::perform(editor::open_lmc(), move |path| {
                                Message::SetContent(state.clone(), path)
                            });
                        }
                        // Save to a *.lmc file, taking ownership of the path returned
                        screen::Event::SaveLMC(state) => {
                            return Task::perform(editor::save_lmc(), move |path| {
                                Message::SaveContent(state.clone(), path)
                            });
                        }

                        // Change to the settings screen, saving the current screen for the functionality of the "Back" button
                        screen::Event::ToSettings => {
                            self.screen = Screen::Settings(settings::State::new(
                                self.config.clone(),
                                Box::new(self.screen.clone()),
                            ));
                        }

                        // Change to the Sandbox screen, passing along a "clone" (incrementing atomic reference count) of the sender and computer, sending the editor font size too
                        screen::Event::ToSandbox => {
                            self.screen = Screen::Sandbox(screen::sandbox::State::new(
                                self.computers.sandbox.clone(),
                                self.sender.clone().unwrap(),
                                self.config.editor_font_size,
                            ))
                        }

                        // Ditto Sandbox comment, also sending along the lesson directory
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

                        // Clear the output (and input index for Lesson View) and set the running computer
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

                        // Set running computer to None
                        screen::Event::Stop => self.computers.running = None,

                        // Reset error and outputs (and input index for Lesson View), send reset input to runtime
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

                        // Send input event to runtime, reset input needed flag
                        screen::Event::SubmitInput(input) => {
                            if self.computers.input_needed
                                && let Some(sender) = &mut self.sender
                                && let Ok(mut sender) = sender.lock()
                            {
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

            // Virtual machine runtime related messages
            Message::Runtime(event) => match event {
                // Set the sender when the program begins to run
                runtime::Event::Ready(sender) => self.sender = Some(Arc::new(Mutex::new(sender))),

                // Update the state of the computer (i.e. registers and RAM)
                runtime::Event::UpdateState(computer) => match &mut self.screen {
                    Screen::LessonView(state) => state.computer = computer,
                    Screen::Sandbox(state) => state.computer = computer,
                    _ => unreachable!(),
                },

                // Set the error value in the terminal pane
                runtime::Event::SetError(error) => match &mut self.screen {
                    Screen::LessonView(state) => {
                        self.computers.running = None;
                        state.error = error
                    }
                    Screen::Sandbox(state) => state.error = error,
                    _ => unreachable!(),
                },

                // Add to the list of outputs in the terminal pane
                runtime::Event::Output(output) => match &mut self.screen {
                    Screen::LessonView(state) => state.output.push(output),
                    Screen::Sandbox(state) => state.output.push(output),
                    _ => unreachable!(),
                },

                // Either give an input value or ask the user for an input
                runtime::Event::Input => {
                    self.computers.input_needed = true;

                    // If the user is in a lesson, automatically feed in the inputs from the lesson, otherwise send a message to the user asking for input in the terminal pane
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

                // Stop the execution of code
                runtime::Event::Halt => self.computers.running = None,

                // Default event, do nothing
                runtime::Event::Continue => {}
            },

            // Message received when the user clicks "Browse" to change the directory of where the program searches for lessons
            Message::LessonsDirectoryChanged(mut state, directory) => {
                // Set the lessons directory to the new directory
                state.lessons_directory = directory;
                // Change the screen to the same screen but with the modified directory
                self.screen = Screen::Settings(state);
            }

            // Message recieved when the user clicks "Open" to browse a list of lessons
            Message::SetContent(mut state, path) => {
                // Ditto LessonsDirectoryChanged comment but with text editor content only unded the condition that the file could be read to disk
                if let Some(path) = path
                    && let Ok(text) = fs::read_to_string(path)
                {
                    state.content = text_editor::Content::with_text(text.as_str());
                    self.screen = Screen::Sandbox(state);
                }
            }

            // Message recieved when the user clicks "Save" to write to an LMC file
            Message::SaveContent(state, path) => {
                // Ditto SetContent comment but with a writing operation instead of a reading one
                if let Some(path) = path
                    && let Ok(mut file) = fs::File::create(path)
                {
                    if let Err(e) = file.write_all(state.content.text().as_bytes()) {
                        panic!("Failed to write to file: {e}");
                    }
                }
            }

            // No information needs to be relayed once the config is saved but iced requires me to handle it
            Message::ConfigSaved => {}

            // Message recieved while the program is running
            Message::Step(_) => {
                // Only run when we have the ability for bidirectional communication (cannot be ensured at compile-tim)
                if let Some(sender) = &mut self.sender
                    && let Ok(mut sender) = sender.lock()
                {
                    // If we are in the Sandox or Lesson View screen send a message to advance the state of the virtual machine, otherwise the user has quit out of the screen so stop running the VM
                    if matches!(self.screen, Screen::Sandbox(_) | Screen::LessonView(_)) {
                        sender.try_send(Input::Step).unwrap();
                    } else {
                        self.computers.running = None;
                    }
                }
            }
        }

        // Send an empty task if no cases match
        Task::none()
    }

    // Set the theme to the theme in the user's config as an iced theme
    fn iced_theme(&self) -> iced::Theme {
        self.config.theme.clone().into()
    }
}
