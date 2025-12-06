// TODO: eventually split into individual editor, sandbox, and terminal components so it can be reused in the lesson viewer

use crate::{
    frontend::{
        app::Algor,
        app::Screen,
        handlers::messages::Message,
        utils::{
            style::{self, terminal, terminal_err, terminal_out},
            widgets::horizontal_separator,
        },
    },
    shared::vm::Computer,
};
use iced::{
    Alignment, Background, Border, Element, Length, Padding, alignment,
    border::Radius,
    widget::{
        button, column, container, horizontal_space, pane_grid, rich_text, row, scrollable, span,
        text, text_editor, text_input,
    },
};

use std::sync::{Arc, Mutex};

pub enum SandboxPane {
    Editor,
    StateViewer,
    Terminal,
}

impl Algor {
    pub fn sandbox(&self) -> Element<'_, Message> {
        column![
            container(
                pane_grid(&self.sandbox_panes, |pane, state, is_maximized| {
                    // TODO: implement From<Pane> for &str
                    let focused = self.sandbox_pane_focused == Some(pane);

                    let title = match state {
                        SandboxPane::Editor => "Editor",
                        SandboxPane::StateViewer => "State Viewer",
                        SandboxPane::Terminal => "Terminal",
                    };

                    let title_bar = pane_grid::TitleBar::new(
                        container(text(title)).padding([4, 8]),
                    )
                    .style(if focused {
                        style::title_bar_focused
                    } else {
                        style::title_bar_unfocused
                    });

                    pane_grid::Content::new(match state {
                        SandboxPane::Editor => container(column![
                            container(
                                column![
                                    row![
                                        button("Open").on_press(Message::Todo),
                                        button("Save").on_press(Message::Todo),
                                        horizontal_space(),
                                        button("Assemble").on_press(Message::AssembleClicked),
                                        button("Run").on_press(Message::RunClicked),
                                        button("Stop").on_press(Message::StopClicked),
                                        button("Reset").on_press(Message::Reset)
                                    ]
                                    .spacing(4),
                                    text_editor(&self.editor_content)
                                        .height(Length::Fill)
                                        .on_action(Message::EditorInputChanged)
                                        .highlight("py", iced::highlighter::Theme::Base16Ocean)
                                ]
                                .spacing(6)
                                .align_x(alignment::Horizontal::Right)
                            )
                            .style(|theme: &iced::Theme| container::Style {
                                background: Some(Background::Color(theme.palette().background)),
                                ..Default::default()
                            })
                            .padding(6),
                            container(
                                text_input("Input...", &self.input)
                                    .on_input(Message::InputChanged)
                                    .on_submit(Message::InputSubmitted)
                            )
                            // TODO: add to utils
                            .style(|theme: &iced::Theme| container::Style {
                                background: Some(Background::Color(theme.palette().background)),
                                ..Default::default()
                            })
                            .width(Length::Fill)
                            .padding(6)
                        ])
                        .padding(Padding {
                            top: 0f32,
                            right: 2f32,
                            bottom: 2f32,
                            left: 2f32,
                        })
                        .width(Length::Fill)
                        .height(Length::Fill),

                        SandboxPane::StateViewer => container(
                            scrollable(
                                column![
                                    text("CPU:"),
                                    horizontal_separator(),
                                    row![
                                        column![
                                            // TODO: this could be a macro
                                            text(if let Some(computer) = &self.computer {
                                                format!(
                                                    "{:02}",
                                                    computer.lock().unwrap().program_counter // TODO: stupid
                                                )
                                            } else {
                                                "00".to_string()
                                            }),
                                            text("PC").size(12)
                                        ]
                                        .align_x(alignment::Horizontal::Center)
                                        .width(Length::Fixed(45f32)),
                                        column![
                                            // TODO: this could be a macro
                                            text(
                                                if let Some(computer) = &self.computer
                                                    && let Ok(computer) = computer.lock()
                                                {
                                                    format!("{:04}", computer.accumulator)
                                                } else {
                                                    "0000".to_string()
                                                }
                                            ),
                                            text("ACC").size(12)
                                        ]
                                        .align_x(alignment::Horizontal::Center)
                                        .width(Length::Fixed(45f32)),
                                        column![
                                            // TODO: this could be a macro
                                            text(if let Some(computer) = &self.computer {
                                                format!(
                                                    "{:01}",
                                                    computer
                                                        .lock()
                                                        .unwrap()
                                                        .current_instruction_register // TODO: stupid
                                                )
                                            } else {
                                                "0".to_string()
                                            }),
                                            text("CIR").size(12)
                                        ]
                                        .align_x(alignment::Horizontal::Center)
                                        .width(Length::Fixed(45f32)),
                                        column![
                                            // TODO: this could be a macro
                                            text(if let Some(computer) = &self.computer {
                                                format!(
                                                    "{:02}",
                                                    computer
                                                        .lock()
                                                        .unwrap()
                                                        .memory_address_register // TODO: stupid
                                                )
                                            } else {
                                                "00".to_string()
                                            }),
                                            text("MAR").size(12)
                                        ]
                                        .align_x(alignment::Horizontal::Center)
                                        .width(Length::Fixed(45f32)),
                                        column![
                                            // TODO: this could be a macro
                                            text(if let Some(computer) = &self.computer {
                                                format!(
                                                    "{:04}",
                                                    computer.lock().unwrap().memory_data_register // TODO: stupid
                                                )
                                            } else {
                                                "0000".to_string()
                                            }),
                                            text("MDR").size(12)
                                        ]
                                        .align_x(alignment::Horizontal::Center)
                                        .width(Length::Fixed(45f32))
                                    ]
                                    .spacing(16),
                                    text("RAM:"),
                                    horizontal_separator(),
                                    row(Arc::clone(
                                        &self
                                            .computer
                                            .as_ref()
                                            .unwrap_or(&Arc::new(Mutex::new(Computer::default())))
                                    ) // TODO: fix, slow and stupid, use let if/match and an array of zeros otherwise
                                    .lock()
                                    .unwrap() // TODO: stupid
                                    .memory
                                    .iter()
                                    .enumerate()
                                    .map(|(i, value)| column![
                                        rich_text![
                                            span(format!("{value}"))
                                                .underline(i as u8 == self.underlined)
                                        ],
                                        text(format!("{i}")).size(8)
                                    ]
                                    .width(Length::Fixed(45f32))
                                    .align_x(alignment::Horizontal::Center)
                                    .into()))
                                    .spacing(16)
                                    .wrap()
                                ]
                                .padding(6)
                                .spacing(16),
                            )
                            .style(|theme: &iced::Theme, _| {
                                let palette = theme.extended_palette();

                                let rail = scrollable::Rail {
                                    background: None,
                                    border: Border {
                                        ..Default::default()
                                    },
                                    scroller: scrollable::Scroller {
                                        border: Border {
                                            ..Default::default()
                                        },
                                        color: palette.secondary.base.color,
                                    },
                                };

                                scrollable::Style {
                                    container: container::Style {
                                        background: Some(Background::Color(
                                            theme.palette().background,
                                        )),
                                        border: Border {
                                            width: 0f32,
                                            radius: Radius::from(2),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    vertical_rail: rail,
                                    horizontal_rail: rail,
                                    gap: None,
                                }
                            })
                            .width(Length::Fill)
                            .height(Length::Fill),
                        )
                        .padding(Padding {
                            top: 0f32,
                            right: 2f32,
                            bottom: 2f32,
                            left: 2f32,
                        })
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(Alignment::Center),

                        SandboxPane::Terminal => container(
                            scrollable(
                                column![
                                    column(self.output.iter().map(|output| {
                                        text(&**output).style(terminal_out).into()
                                    })),
                                    text(&self.error).style(terminal_err)
                                ]
                                .padding(6)
                                .spacing(16),
                            )
                            .style(terminal)
                            .width(Length::Fill)
                            .height(Length::Fill),
                        )
                        .padding(2)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(Alignment::Center),
                    })
                    .style(if focused {
                        style::grid_pane_focused
                    } else {
                        style::grid_pane_unfocused
                    })
                    .title_bar(title_bar)
                })
                .spacing(8)
                .on_click(Message::SandboxPaneClicked)
                .on_drag(Message::SandboxPaneDragged)
                .on_resize(10, Message::SandboxPaneResized)
            )
            .padding([8, 0]),
            row![
                button("Back").on_press(Message::SetScreen(Screen::Menu)),
                horizontal_space(),
                button("Settings").on_press(Message::SetScreen(Screen::Settings)),
            ]
        ]
        .padding(12)
        .into()
    }
}
