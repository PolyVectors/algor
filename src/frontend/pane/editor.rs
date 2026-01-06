use iced::{
    Background, Element, Length, Padding, alignment,
    widget::{button, column, container, row, space, text_editor, text_input},
};

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
    input_content: &String,
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
                text_editor(editor_content)
                    .height(Length::Fill)
                    .on_action(Message::ContentChanged)
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
            text_input("Input...", input_content)
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
    .height(Length::Fill)
    .into()
}
