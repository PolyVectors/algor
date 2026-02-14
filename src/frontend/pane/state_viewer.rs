use iced::{
    Alignment, Element, Font, Length, Padding, alignment,
    widget::{column, container, rich_text, row, scrollable, span, text},
};

use crate::{
    frontend::{pane::style, util::widgets::separator},
    shared::vm::Computer,
};

// No messages required but provide a mapping for future maintainability
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
                        // Force PC register display to be 2 digits wide
                        text(format!("{:02}", computer.program_counter)),
                        text("PC").size(12)
                    ]
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fixed(45f32)),
                    column![
                        // Force accumulator register display to be 4 digits wide (as to be mindful for plus and minus sign prefixes)
                        text(format!("{:04}", computer.accumulator)),
                        text("ACC").size(12)
                    ]
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fixed(45f32)),
                    column![
                        // CIR register only needs to be one digit wide as there are less than 10 opcodes in the LMC ISA
                        text(format!("{:01}", computer.current_instruction_register)),
                        text("CIR").size(12)
                    ]
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fixed(45f32)),
                    column![
                        // 100 memory addresses starting from 0 (i.e. highest address is 99) so only 2 digits are required
                        text(format!("{:02}", computer.memory_address_register)),
                        text("MAR").size(12)
                    ]
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fixed(45f32)),
                    column![
                        // Ditto accumulator comment
                        text(format!("{:04}", computer.memory_data_register)),
                        text("MDR").size(12)
                    ]
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fixed(45f32))
                ]
                .spacing(16),
                text("RAM:"),
                separator::horizontal(),
                // Display all memory locations and addresses, underline the location currently in use by the program counter
                row(computer.memory.iter().enumerate().map(|(i, value)| column![
                    // Use rich text for underline feature
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
        // Use a style that forces background to render for elements with scrollbars (ditto src/frontend/pane/editor.rs solid_background comment)
        .style(style::solid_background_scrollable)
        .width(Length::Fill)
        .height(Length::Fill),
    )
    // Padding to avoid elements rendering under title bar
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
