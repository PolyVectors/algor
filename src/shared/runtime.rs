use iced::futures::Stream;
use iced::futures::channel::mpsc;
use iced::stream;

use crate::backend::{
    compiler,
    virtual_machine::{Computer, InvalidLocation},
};
use std::sync::{Arc, Mutex};

pub enum Event {
    Ready(mpsc::Sender<Input>),
    UpdateState(Arc<Mutex<Computer>>),
    SetError(InvalidLocation),
    Continue,
    Halt,
    Output(Box<str>),
    Input,
}

#[derive(Debug)]
pub enum Input {
    RunClicked,
    AssembleClicked(String),
}

pub fn run() -> impl Stream<Item = Event> {
    stream::channel(100, |mut output| async move {
        let (sender, mut receiver) = mpsc::channel(100);
        output.try_send(Event::Ready(sender)).expect("test"); // TODO: stupid

        let computer = Arc::new(Mutex::new(Computer::default()));

        loop {
            use iced_futures::futures::StreamExt;

            let input = receiver.select_next_some().await;

            match input {
                Input::RunClicked => {
                    if let Ok(mut computer) = computer.lock() {
                        if let Err(e) = computer.step() {
                            output.try_send(Event::SetError(e)).unwrap() // TODO: stupid
                        };
                        println!("{computer:?}");
                    }
                }

                Input::AssembleClicked(source) => {
                    if let Ok(code) = compiler::compile(&source) {
                        computer.lock().unwrap().memory = code; // TODO: stupid
                    }
                }
            }

            output
                .try_send(Event::UpdateState(Arc::clone(&computer)))
                .unwrap() // TODO: stupid
        }
    })
}
