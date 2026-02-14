use rfd::AsyncFileDialog;

use iced::{
    Element, Length, Padding, alignment,
    widget::{button, column, container, row, space, text_editor, text_input},
};

use crate::frontend::pane::style;

// Gets the path of a file asynchronously
pub async fn open_lmc() -> Option<String> {
    Some(
        AsyncFileDialog::new()
            .set_title("Pick LMC file...")
            // Only allow opening files with the .lmc or .asm extension
            .add_filter("LMC", &["lmc", "asm"])
            .pick_file()
            // If at any point an operation fails to continue, return a None value that will be ignored
            .await?
            .path()
            .to_str()
            .to_owned()?
            .to_owned(),
    )
}

// Gets the path of a file (with the ability to select a file name) asynchronously
pub async fn save_lmc() -> Option<String> {
    Some(
        AsyncFileDialog::new()
            .set_title("Save LMC file...")
            // Ditto extension comment
            .add_filter("LMC", &["lmc", "asm"])
            .save_file()
            // Ditto open_lmc message
            .await?
            .path()
            .to_str()
            .to_owned()?
            .to_owned(),
    )
}

// Message files specific to the editor pane, conveted to screen-specific messages using the map method
#[derive(Debug, Clone)]
pub enum Message {
    OpenClicked,
    SaveClicked,
    AssembleClicked,
    RunClicked,
    StopClicked,
    ResetClicked,
    // Event for when any action is performed in a text editor
    ContentChanged(text_editor::Action),
    InputChanged(String),
    InputSubmitted,
}

pub fn editor<'a>(
    editor_content: &'a text_editor::Content,
    text_size: u32,
    input_content: Option<&String>,
) -> Element<'a, Message> {
    container(column![
        container(
            column![
                row![
                    // Only show open and save options if there is an input reference provided (i.e. in sandbox mode)
                    input_content.is_some().then(|| {
                        container(
                            row![
                                button("Open").on_press(Message::OpenClicked),
                                button("Save").on_press(Message::SaveClicked),
                                space::horizontal()
                            ]
                            .spacing(4),
                        )
                    }),
                    button("Assemble").on_press(Message::AssembleClicked),
                    button("Run").on_press(Message::RunClicked),
                    button("Stop").on_press(Message::StopClicked),
                    button("Reset").on_press(Message::ResetClicked)
                ]
                .spacing(4),
                text_editor(editor_content)
                    .size(text_size)
                    .height(Length::Fill)
                    .on_action(Message::ContentChanged)
                    // Use python syntax highlighting as an approximation of assembly
                    .highlight("py", iced::highlighter::Theme::Base16Ocean)
            ]
            .spacing(6)
            .align_x(alignment::Horizontal::Right)
        )
        .style(style::solid_background)
        .padding(6),
        // Ditto save and show options comment but with the input box instead
        input_content.is_some().then(|| {
            container(
                text_input("Input...", input_content.unwrap_or(&String::new()))
                    .on_input(Message::InputChanged)
                    .on_submit(Message::InputSubmitted),
            )
            .style(style::solid_background)
            .width(Length::Fill)
            .padding(6)
        })
    ])
    // Padding as to not interfere with the title bar
    .padding(Padding {
        top: 0f32,
        right: 2f32,
        bottom: 2f32,
        left: 2f32,
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
