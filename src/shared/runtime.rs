use iced::futures::Stream;
use iced::futures::channel::mpsc;
use iced::stream;

use crate::backend::compiler::{self, generator::Location};
use crate::shared::vm::Computer;
use std::sync::{Arc, Mutex};

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

#[derive(Debug)]
pub enum Input {
    AssembleClicked(String),
    SetInput(String),
    Step,
    Reset,
}

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
    stream::channel(100, |mut output| async move {
        let (sender, mut receiver) = mpsc::channel(100);
        send_or_panic!(output, Event::Ready(sender));

        let computer = Arc::new(Mutex::new(Computer::default()));

        loop {
            use iced_futures::futures::StreamExt;
            let input = receiver.select_next_some().await;

            match input {
                Input::AssembleClicked(source) => match compiler::compile(&source) {
                    Ok(code) => {
                        if let Ok(mut computer) = computer.lock() {
                            computer.reset();
                            computer.memory = code;
                        }
                    }
                    Err(e) => {
                        send_or_panic!(output, Event::SetError(format!("{e}")));
                    }
                },

                Input::SetInput(input) => {
                    if let Ok(mut computer) = computer.lock() {
                        computer.accumulator = input.parse().unwrap_or_default();
                    }
                }

                Input::Step => {
                    if let Ok(mut computer) = computer.lock() {
                        match computer.step() {
                            Ok(event) => send_or_panic!(output, event),
                            Err(e) => send_or_panic!(output, Event::SetError(format!("{e}"))),
                        }
                    }
                }

                Input::Reset => {
                    if let Ok(mut computer) = computer.lock() {
                        computer.reset();
                        // computer.memory = [Location::Data(0); 100];
                    }
                }
            }

            send_or_panic!(output, Event::UpdateState(Arc::clone(&computer)))
        }
    })
}
