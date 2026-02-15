use iced::futures::Stream;
use iced::futures::channel::mpsc;
use iced::stream;

use crate::backend::compiler::generator::Location;
use crate::backend::compiler::{self};
use crate::shared::vm::Computer;
use std::sync::{Arc, Mutex};

// Events received during runtime execution
#[derive(Debug)]
pub enum Input {
    AssembleClicked(String),
    SetInput(String),
    Step,
    Reset,
}

// Events sent back during runtime execution
#[derive(Debug)]
pub enum Event {
    Ready(mpsc::Sender<Input>),
    UpdateState(Arc<Mutex<Computer>>),
    SetError(String),
    Continue,
    Halt,
    Output(Box<str>),
    Input,
}

// A macro that expands out to an if let block, crashing the thread at runtime if there is an error sending back an event
macro_rules! send_or_panic {
    ($a:expr,$b:expr) => {{
        if let Err(e) = $a.try_send($b) {
            panic!(
                "Encountered a fatal error whilst trying to send event to subscription handler: {e}"
            );
        }
    }};
}

pub fn run() -> impl Stream<Item = Event> {
    // Create a new channel and take ownership of all variables in the function into the closure (for future maintainability)
    stream::channel(100, |mut output: mpsc::Sender<Event>| async move {
        let (sender, mut receiver) = mpsc::channel(100);
        send_or_panic!(output, Event::Ready(sender));

        // Create a new computer and wrap it in an atomically references counted mutex
        let computer = Arc::new(Mutex::new(Computer::default()));

        loop {
            use iced_futures::futures::StreamExt;
            let input = receiver.select_next_some().await;

            // Attempt to receive interior mutability for the computer, otherwise try again on the next loop cycle
            let Ok(mut inner_computer) = computer.lock() else {
                continue;
            };

            match input {
                Input::AssembleClicked(source) => match compiler::compile(&source) {
                    // Case for no compiler errors
                    Ok(code) => {
                        // Reset the computer and assign machine code to memory
                        inner_computer.reset();
                        inner_computer.memory = code;
                    }
                    Err(e) => {
                        // If there is a compiler error, send it back as a string to be displayed in the terminal widget
                        send_or_panic!(output, Event::SetError(format!("{e}")));
                    }
                },

                Input::SetInput(input) => {
                    // Parse and set input asynchronously
                    inner_computer.accumulator = input.parse().unwrap_or_default();
                }

                Input::Step => match inner_computer.step() {
                    // Send the event from the virtual machine
                    Ok(event) => send_or_panic!(output, event),
                    // Send the error message from the virtual machine
                    Err(e) => send_or_panic!(output, Event::SetError(format!("{e}"))),
                },

                Input::Reset => {
                    // Reset registers and memory
                    inner_computer.reset();
                    inner_computer.memory = [Location::Data(0); 100];
                }
            }

            // Send back a copy of the updated state of the computer after an input
            send_or_panic!(output, Event::UpdateState(Arc::clone(&computer)))
        }
    })
}
