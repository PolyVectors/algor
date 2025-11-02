use std::fmt;

#[repr(u8)]
#[derive(Debug)]
pub enum Token {
    Halt,
    Add,
    Sub,
    Store,
    Load,
    Branch,
    BranchZero,
    BranchPositive,
    Input,
    Output,
    Data,
    Number(i32),
    Identifier(String),
}

pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
}

#[derive(Debug)]
pub struct InvalidCharacter {
    character: char,
    line_column: (usize, usize),
}

impl fmt::Display for InvalidCharacter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid character '{}' while lexing ({}:{})",
            self.character, self.line_column.0, self.line_column.1
        )
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    fn lex_string(&mut self, tokens: &mut Vec<Token>) {
        let mut string = String::new();

        while self.position < self.source.len() - 1 {
            let character = self.source.as_bytes()[self.position] as char;
            match character {
                'A'..'Z' | 'a'..'z' => {
                    string.push(character);
                    self.position += 1;
                }
                _ => break,
            }
        }

        let token = match string.to_uppercase().as_str() {
            "HLT" => Token::Halt,
            _ => Token::Identifier(string),
        };
        tokens.push(token);
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, InvalidCharacter> {
        let mut tokens = Vec::new();

        while self.position < self.source.len() - 1 {
            let character = self.source.as_bytes()[self.position] as char;
            let line = self.source[0:self.position];
            println!("{line}");

            match character {
                'A'..'Z' | 'a'..'z' => self.lex_string(&mut tokens),
                ' ' | '\t' | '\n' => self.position += 1,

                _ => {
                    return Err(InvalidCharacter {
                        character,
                        line_column: (1, self.position + 1),
                    });
                }
            }
        }

        Ok(tokens)
    }
}
