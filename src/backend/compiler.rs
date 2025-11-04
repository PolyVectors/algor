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
    Number(i16),
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
                'A'..='Z' | 'a'..='z' => {
                    string.push(character);
                    self.position += 1;
                }
                _ => break,
            }
        }

        let token = match string.to_uppercase().as_str() {
            "HLT" | "COB" => Token::Halt,
            "ADD" => Token::Add,
            "SUB" => Token::Sub,
            "STA" | "STO" => Token::Store,
            "LDA" => Token::Load,
            "BRA" => Token::Branch,
            "BRZ" => Token::BranchZero,
            "BRP" => Token::BranchPositive,
            "INP" => Token::Input,
            "OUT" => Token::Output,
            "DAT" => Token::Data,
            _ => Token::Identifier(string),
        };
        tokens.push(token);
    }

    fn lex_number(&mut self, tokens: &mut Vec<Token>) {
        let mut number = String::new();

        while self.position < self.source.len() - 1 {
            let character = self.source.as_bytes()[self.position] as char;

            match character {
                '0'..='9' => {
                    number.push(character);
                    self.position += 1;
                }
                _ => break,
            }
        }

        tokens.push(Token::Number(number.parse().unwrap_or(0)))
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, InvalidCharacter> {
        let mut tokens = Vec::new();

        while self.position < self.source.len() - 1 {
            let character = self.source.as_bytes()[self.position] as char;
            match character {
                'A'..='Z' | 'a'..='z' => self.lex_string(&mut tokens),
                '0'..='9' => self.lex_number(&mut tokens),
                ' ' | '\t' | '\n' => self.position += 1,

                _ => {
                    let line = &self.source[0..self.position]
                        .chars()
                        .filter(|c| *c == '\n')
                        .count()
                        + 1;
                    let column = &self.source[0..self.position].trim().len()
                        - &self.source[0..self.position].rfind('\n').unwrap_or(0);

                    return Err(InvalidCharacter {
                        character,
                        line_column: (line, if column == 0 { 1 } else { column }),
                    });
                }
            }
        }

        Ok(tokens)
    }
}
