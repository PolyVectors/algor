use iced::{Element, widget::column};

use crate::shared::vm::Computer;

#[derive(Debug, Clone)]
pub enum Message {}

pub enum Event {
    SetComputer(Computer),
}

#[derive(Debug, Clone)]
pub struct State;

impl Default for State {
    fn default() -> Self {
        Self
    }
}

impl State {
    pub fn update(&mut self, message: Message) -> Option<Event> {
        None
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        column![].into()
    }
}
