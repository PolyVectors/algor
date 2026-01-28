use iced::{
    Background, Element, Length, Padding, Theme, alignment,
    widget::{button, column, container, row, space, text_editor, text_input},
};

fn solid_background(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(theme.palette().background)),
        ..Default::default()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenClicked,
    SaveClicked,
    AssembleClicked,
    RunClicked,
    StopClicked,
    ResetClicked,
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
                    button("Open").on_press(Message::OpenClicked),
                    button("Save").on_press(Message::SaveClicked),
                    space::horizontal(),
                    button("Assemble").on_press(Message::AssembleClicked),
                    button("Run").on_press(Message::RunClicked),
                    button("Stop").on_press(Message::StopClicked),
                    button("Reset").on_press(Message::ResetClicked)
                ]
                .spacing(4),
                // TODO: this is the code causing the lag, fix
                text_editor(editor_content)
                    .size(text_size)
                    .height(Length::Fill)
                    .on_action(Message::ContentChanged)
                    .highlight("py", iced::highlighter::Theme::Base16Ocean)
            ]
            .spacing(6)
            .align_x(alignment::Horizontal::Right)
        )
        .style(solid_background)
        .padding(6),
        input_content.is_some().then(|| {
            container(
                text_input("Input...", input_content.unwrap_or(&String::new()))
                    .on_input(Message::InputChanged)
                    .on_submit(Message::InputSubmitted),
            )
            .style(solid_background)
            .width(Length::Fill)
            .padding(6)
        })
    ])
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
