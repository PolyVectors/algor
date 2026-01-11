use iced::{
    Element, Padding,
    widget::{column, container, text},
};
use serde::Deserialize;

use crate::frontend::util::{font::Font, widgets::separator};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Head {
    pub title: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum SlideMember {
    Separator,
    #[serde(rename = "h1")]
    HeaderOne(String),
    #[serde(rename = "h2")]
    HeaderTwo(String),
    #[serde(rename = "h3")]
    HeaderThree(String),
    #[serde(rename = "p")]
    Paragraph(String),
    #[serde(rename = "i")]
    Italics(String),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Inputs {
    #[serde(rename = "li")]
    items: Vec<u16>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Outputs {
    #[serde(rename = "li")]
    items: Vec<u16>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Slide {
    inputs: Inputs,
    outputs: Outputs,
    #[serde(rename = "#content")]
    pub members: Vec<SlideMember>,
}

impl Slide {
    pub fn parse<'a, Message: 'a>(&'a self) -> Element<'a, Message> {
        let mut column = column![];

        for member in self.members.iter() {
            let element: Element<'a, Message> = match member {
                SlideMember::HeaderOne(content) => text(content).font(Font::Bold).size(24).into(),
                SlideMember::HeaderTwo(content) => text(content).font(Font::Bold).size(20).into(),
                SlideMember::HeaderThree(content) => text(content).font(Font::Bold).size(16).into(),
                SlideMember::Paragraph(content) => text(content).into(),
                SlideMember::Italics(content) => text(content).font(Font::Italic).into(),
                SlideMember::Separator => separator::horizontal().into(),
            };

            column = column.push(element);
        }

        column.spacing(6).into()
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Body {
    #[serde(rename = "slide")]
    pub slides: Vec<Slide>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(rename = "algor-lesson")]
pub struct Lesson {
    pub head: Head,
    pub body: Body,
}
