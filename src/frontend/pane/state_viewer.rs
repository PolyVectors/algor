use iced::{
    Alignment, Element, Font, Length, Padding, alignment,
    widget::{column, container, rich_text, row, scrollable, span, text},
};

use crate::{
    frontend::{pane::style, util::widgets::separator},
    shared::vm::Computer,
};

#[derive(Debug, Clone)]
pub enum Message {}

pub fn state_viewer<'a>(computer: &Computer) -> Element<'a, Message> {
    container(
        scrollable(
            column![
                text("CPU:"),
                separator::horizontal(),
                row![
                    column![
                        text(format!("{:02}", computer.program_counter)),
                        text("PC").size(12)
                    ]
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fixed(45f32)),
                    column![
                        text(format!("{:04}", computer.accumulator)),
                        text("ACC").size(12)
                    ]
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fixed(45f32)),
                    column![
                        text(format!("{:01}", computer.current_instruction_register)),
                        text("CIR").size(12)
                    ]
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fixed(45f32)),
                    column![
                        text(format!("{:02}", computer.memory_address_register)),
                        text("MAR").size(12)
                    ]
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fixed(45f32)),
                    column![
                        text(format!("{:04}", computer.memory_data_register)),
                        text("MDR").size(12)
                    ]
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fixed(45f32))
                ]
                .spacing(16),
                text("RAM:"),
                separator::horizontal(),
                row(computer.memory.iter().enumerate().map(|(i, value)| column![
                    rich_text![
                        span::<(), Font>(format!("{value}"))
                            .underline(i as u8 == computer.program_counter)
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
        .style(style::state_viewer)
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
    .align_x(Alignment::Center)
    .into()
}
