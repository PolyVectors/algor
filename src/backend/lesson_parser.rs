// TODO: switch main parser to serde_xml_rs

use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

use crate::frontend::util::font::Font;
use crate::frontend::util::widgets::separator;
use iced::Element;
use iced::widget::{Column, text};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
pub enum ParserError {
    IOError(io::Error),
    ParserError,
}

impl From<io::Error> for ParserError {
    fn from(e: io::Error) -> Self {
        ParserError::IOError(e)
    }
}

pub struct Head {
    pub title: String,
}

impl Default for Head {
    fn default() -> Self {
        Self {
            title: String::from("Untitled Lesson"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {}

pub struct Parser<'a> {
    position: usize,
    depth: i32,
    reader: EventReader<BufReader<File>>,
    names: Vec<String>,
    lesson: Column<'a, Message>, // should be a Vec<Column<'a, Message>> which represents a slide
}

impl<'a> Parser<'a> {
    pub fn new(path: PathBuf) -> Result<Self, ParserError> {
        let file = File::open(path)?;
        let file = BufReader::new(file);

        Ok(Self {
            position: 0,
            depth: 0,
            reader: EventReader::new(file),
            names: Vec::new(),
            lesson: Column::new(),
        })
    }

    pub fn parse_head(mut self) -> Result<Head, ParserError> {
        let mut head = Head {
            title: String::new(),
        };

        for element in self.reader {
            match element {
                Ok(XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => {
                    self.names.push(name.to_string());
                    self.depth += 1;
                }
                Ok(XmlEvent::EndElement { name: _ }) => {
                    self.depth -= 1;
                }
                Ok(XmlEvent::Characters(str)) => match (
                    self.names
                        .get(self.names.len() - 1)
                        .map(|name| name.as_str()),
                    self.names
                        .get(self.names.len() - 2)
                        .map(|name| name.as_str()),
                ) {
                    (Some("title"), Some("head")) => head.title = str,
                    // _ => Err(ParserError::ParserError)?
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(head)
    }

    pub fn parse(mut self) -> Result<Element<'a, Message>, ParserError> {
        for element in self.reader {
            match element {
                Ok(XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => {
                    if name.to_string() == "separator" {
                        self.lesson = self.lesson.push(separator::horizontal());
                    }

                    self.names.push(name.to_string());
                    self.depth += 1;
                }
                Ok(XmlEvent::EndElement { name: _ }) => {
                    self.depth -= 1;
                }
                Ok(XmlEvent::Characters(str)) => match self.names[self.names.len() - 1].as_str() {
                    "h1" => self.lesson = self.lesson.push(text(str).font(Font::Bold).size(24)),
                    "p" => self.lesson = self.lesson.push(text(str)),
                    "i" => self.lesson = self.lesson.push(text(str).font(Font::Italic)),

                    // _ => Err(ParserError::ParserError)?
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(self.lesson.spacing(8).padding(8).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::lessons;
    use std::path::PathBuf;

    #[test]
    fn parse() {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("src/backend/lessons/resources/test.xml");

        let _ = lessons::Parser::new(dir).unwrap().parse();
    }
}
