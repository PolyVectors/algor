use std;

use crate::backend::compiler::generator::Location;
use crate::backend::compiler::lexer::Lexer;
use crate::backend::compiler::parser::Parser;

pub fn compile(source: &str) -> Result<[Location; 100], Box<dyn std::error::Error>> {
    Ok(<[Location; 100]>::try_from(
        Parser::new(Lexer::new(source).lex()?).parse()?,
    )?)
}
