use iced::futures::Stream;
use iced::futures::channel::mpsc;
use iced::stream;

use crate::backend::compiler::{self, generator::Location};
use crate::shared::vm::Computer;
use std::sync::{Arc, Mutex};

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
    Step,
    Reset,
}

pub fn run() -> impl Stream<Item = Event> {
    stream::channel(100, |mut output| async move {
        let (sender, mut receiver) = mpsc::channel(100);
        output.try_send(Event::Ready(sender)).unwrap(); // TODO: stupid

        let computer = Arc::new(Mutex::new(Computer::default()));

        loop {
            use iced_futures::futures::StreamExt;
            let input = receiver.select_next_some().await;

            match input {
                Input::Step => {
                    if let Ok(mut computer) = computer.lock() {
                        match computer.step() {
                            Ok(event) => {
                                output.try_send(event).unwrap() // TODO: stupid
                            }
                            Err(e) => {
                                output.try_send(Event::SetError(format!("{e}"))).unwrap(); // TODO: stupid
                            }
                        }
                    }
                }

                Input::Reset => {
                    computer.lock().unwrap().reset(); // TODO: stupid
                    computer.lock().unwrap().memory = [Location::Data(0); 100]; // TODO: stupid
                }

                Input::AssembleClicked(source) => {
                    match compiler::compile(&source) {
                        Ok(code) => {
                            computer.lock().unwrap().reset(); // TODO: stupid
                            computer.lock().unwrap().memory = code; // TODO: stupid
                        }
                        Err(e) => output.try_send(Event::SetError(format!("{e}"))).unwrap(), // TODO: stupid
                    }
                }
            }

            output
                .try_send(Event::UpdateState(Arc::clone(&computer)))
                .unwrap() // TODO: stupid
        }
    })
}
