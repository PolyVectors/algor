use serde::Deserialize;

#[derive(Clone, Debug)]
pub enum Message {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Head {
    pub title: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum SlideMember {
    #[serde(rename = "p")]
    Paragraph(String),
    #[serde(rename = "h1")]
    HeaderOne(String),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Slide {
    #[serde(rename = "#content")]
    slide_member: Vec<SlideMember>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Body {
    #[serde(rename = "slide")]
    slides: Vec<Slide>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
#[serde(rename = "algor-lesson")]
pub struct Lesson {
    pub head: Head,
    pub body: Body,
}
