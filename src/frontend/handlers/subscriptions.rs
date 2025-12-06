use crate::backend::config::RunSpeed;
use crate::frontend::{app::Algor, handlers::messages::Message};
use crate::shared::runtime::{self, Event};
use iced::Subscription;
use iced::time;
use std::time::Duration;

impl Algor {
    pub fn subscription(&self) -> Subscription<Message> {
        let run = Subscription::run(runtime::run).map(|event| match event {
            Event::Ready(sender) => Message::Ready(sender),
            Event::UpdateState(state) => Message::UpdateState(state),
            Event::SetError(e) => Message::SetError(format!("{e}")),
            Event::Continue => Message::None,
            Event::Halt => Message::Halt,
            Event::Output(output) => Message::AppendOutput(output),
            Event::Input => Message::AskInput,
        });

        let step = if self.running {
            time::every(match self.run_speed.unwrap_or_default() {
                RunSpeed::Slow => Duration::from_millis(1000),
                RunSpeed::Medium => Duration::from_millis(250),
                RunSpeed::Fast => Duration::from_millis(100),
                RunSpeed::Instant => Duration::from_millis(0),
            })
            .map(Message::Step)
        } else {
            Subscription::none()
        };

        Subscription::batch(vec![run, step])
    }
}
