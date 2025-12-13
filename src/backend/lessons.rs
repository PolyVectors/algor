use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

use crate::frontend::font::Font;
use crate::frontend::message::Message;
use crate::frontend::widgets::horizontal_separator;
use iced::Element;
use iced::widget::column;
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

pub struct Parser<'a> {
    position: usize,
    depth: i32,
    reader: EventReader<BufReader<File>>,
    last_name: String,
    lesson: Column<'a, Message>,
}

impl<'a> Parser<'a> {
    pub fn new(path: PathBuf) -> Result<Self, ParserError> {
        let file = File::open(path)?;
        let file = BufReader::new(file);

        Ok(Self {
            position: 0,
            depth: 0,
            reader: EventReader::new(file),
            last_name: String::new(),
            lesson: Column::new(),
        })
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
                        self.lesson = self.lesson.push(horizontal_separator());
                    }

                    self.last_name = name.to_string();
                    self.depth += 1;
                }
                Ok(XmlEvent::EndElement { name: _ }) => {
                    self.depth -= 1;
                }
                Ok(XmlEvent::Characters(str)) => match self.last_name.as_str() {
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
