pub mod generator;
pub mod lexer;
pub mod parser;

#[cfg(test)]
pub mod tests;

use crate::backend::compiler::generator::Location;
use crate::backend::compiler::lexer::Lexer;
use crate::backend::compiler::parser::Parser;
use std;

// Combines the lexer, parser, and code generator, returning machine code that can be placed into RAM, or a generic error if any method fails
pub fn compile(source: &str) -> Result<[Location; 100], Box<dyn std::error::Error>> {
    Ok(<[Location; 100]>::try_from(
        Parser::new(Lexer::new(source).lex()?).parse()?,
    )?)
}
