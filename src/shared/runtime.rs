use iced::futures::Stream;
use iced::futures::channel::mpsc;
use iced::stream;

use crate::backend::virtual_machine::Computer;
use std::sync::{Arc, Mutex};

pub enum Event {
    Ready(mpsc::Sender<Input>),
    UpdateState(Arc<Mutex<Computer>>),
}

#[derive(Debug)]
pub enum Input {
    RunClicked,
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
                Input::RunClicked => output
                    .try_send(Event::UpdateState(Arc::clone(&computer)))
                    .unwrap(), // TODO: stupid
            }
        }
    })
}
