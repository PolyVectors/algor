use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

use iced::Element;
use iced::widget::{Column, text};
use iced::widget::column;
use crate::frontend::message::Message;
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

// TODO: make parser struct
pub fn parse<'a>(path: PathBuf) -> Result<Element<'a, Message>, ParserError> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;

    let mut column: Column<'_, Message> = Column::new();
    let mut last_name = String::new();

    for element in parser {
        match element {
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            }) => {
                println!("ELEMENT: {name} {attributes:?} {namespace:?}");
                last_name = name.to_string();
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name: _ }) => {
                depth -= 1;
            }
            Ok(XmlEvent::Characters(str)) => {
                match last_name.as_str() {
                    "text" => column = column.push(text(str)),
                    _ => Err(ParserError::ParserError)?
                }
            }
            _ => {}
        }
    }

    Ok(column.into())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::backend::lessons;
    
    #[test]
    fn parse() {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("src/backend/lessons/resources/test.xml");

        // TODO: weird
        let _ = lessons::parse(dir);
    }
}
