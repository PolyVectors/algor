use iced::{
    Element, alignment,
    widget::{button, column, container, pane_grid, row, space, text, text_editor, text_input},
};

use crate::frontend::pane::{editor::editor, style};
use crate::shared::vm::Computer;

#[derive(Debug, Clone)]
pub enum Message {
    PaneClicked(pane_grid::Pane),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    BackClicked,
    SettingsClicked,
}

pub enum Event {
    SetComputer(Computer),
}

#[derive(Debug, Clone)]
pub enum Pane {
    Editor,
    StateViewer,
    Terminal,
}

#[derive(Debug, Clone)]
pub struct State {
    panes: pane_grid::State<Pane>,
    pane_focused: Option<pane_grid::Pane>,
    editor_content: text_editor::Content,
    input_content: String,
}

impl Default for State {
    fn default() -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::Editor);

        panes.split(pane_grid::Axis::Vertical, pane, Pane::StateViewer);
        panes.split(pane_grid::Axis::Horizontal, pane, Pane::Terminal);

        Self {
            panes,
            pane_focused: None,
            editor_content: text_editor::Content::new(),
            input_content: String::new(),
        }
    }
}

impl State {
    pub fn update(&mut self, message: Message) -> Option<Event> {
        None
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            container(
                pane_grid(&self.panes, |pane, state, is_maximized| {
                    pane_grid::Content::new(match state {
                        Pane::Editor => text("test"),
                        Pane::StateViewer => todo!(),
                        Pane::Terminal => todo!(),
                    })
                })
                .spacing(8)
                .on_click(Message::PaneClicked)
                .on_drag(Message::PaneDragged)
                .on_resize(10, Message::PaneResized)
            )
            .padding([8, 0]),
            row![
                button("Back").on_press(Message::BackClicked),
                space::horizontal(),
                button("Settings").on_press(Message::SettingsClicked),
            ]
        ]
        .padding(12)
        .into()
    }
}
