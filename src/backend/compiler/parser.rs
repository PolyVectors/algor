use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};
use std::rc::Rc;

use crate::backend::compiler::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum NumberOrIdentifier {
    Number(i16),
    Identifier(Rc<str>),
}

// Create the Instruction enum, similar to the Token enum, but bundling together the opcode and operand(s)
#[derive(PartialEq, Debug)]
pub enum Instruction {
    Halt,
    Add(NumberOrIdentifier),
    Sub(NumberOrIdentifier),
    Store(NumberOrIdentifier),
    Load(NumberOrIdentifier),
    Branch(NumberOrIdentifier),
    BranchZero(NumberOrIdentifier),
    BranchPositive(NumberOrIdentifier),
    Input,
    Output,
    Data(Rc<str>, i16),
}

#[derive(PartialEq, Debug)]
pub struct Program {
    pub labels: HashMap<Rc<str>, u8>,
    // TODO: explicitly limit to 100 instructions
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Rc<Token>>,
    // TODO: this could be a u8
    position: usize,
    program: Program,
}

// TODO: add line and column
#[derive(PartialEq, Debug)]
pub struct InvalidToken {
    pub expected: Vec<Token>,
    pub received: Option<Rc<Token>>,
}

impl Display for InvalidToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expected = match self.expected.len() {
            0 => "Expected nothing".to_string(),
            1 => format!("Expected {}", self.expected[0]),

            _ => format!(
                "Expected: {}",
                self.expected
                    .iter()
                    .enumerate()
                    .fold(String::new(), |acc, (i, x)| if i == 0 {
                        x.to_string()
                    } else if i == self.expected.len() - 1 {
                        format!("{acc} or {x}")
                    } else {
                        format!("{acc}, {x}")
                    })
            ),
        };

        let received = if let Some(token) = &self.received {
            token.to_string()
        } else {
            "nothing".to_string()
        };

        write!(f, "{expected}, received {received}")
    }
}

impl Error for InvalidToken {}

#[derive(PartialEq, Debug)]
pub enum ParserError {
    InvalidToken(InvalidToken),
    NumberOutOfRange(i16),
    AddressOutOfRange(i16),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            ParserError::InvalidToken(invalid_token) => invalid_token.to_string(),
            ParserError::NumberOutOfRange(number) => {
                format!(
                    "Number `{number}` out of range, expected a number between -999 and 999 inclusive"
                )
            }
            ParserError::AddressOutOfRange(address) => {
                format!(
                    "Address `{address}` out of range, expected a number between 0 and 100 exclusive"
                )
            }
        };

        write!(f, "Encountered an error while parsing...\n{text}")
    }
}

impl Error for ParserError {}

const INSTRUCTIONS: [Token; 10] = [
    Token::Halt,
    Token::Add,
    Token::Sub,
    Token::Store,
    Token::Load,
    Token::Branch,
    Token::BranchZero,
    Token::BranchPositive,
    Token::Input,
    Token::Output,
];

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().map(Rc::new).collect(),
            position: 0,
            program: Program {
                labels: HashMap::new(),
                instructions: Vec::new(),
            },
        }
    }

    fn expect_newline(&mut self) -> Result<(), ParserError> {
        if let Some(token) = self.tokens.get(self.position) {
            let Token::Newline = &**token else {
                Err(ParserError::InvalidToken(InvalidToken {
                    expected: vec![Token::Newline],
                    received: Some(Rc::clone(token)),
                }))?
            };
        }

        Ok(())
    }

    fn parse_no_operand(&mut self) -> Result<(), ParserError> {
        let token = &self.tokens[self.position];

        let instruction = match &**token {
            Token::Halt => Instruction::Halt,
            Token::Input => Instruction::Input,
            Token::Output => Instruction::Output,
            _ => unreachable!(),
        };

        self.program.instructions.push(instruction);

        self.position += 1;
        self.expect_newline()?;

        Ok(())
    }

    fn parse_single_operand(&mut self) -> Result<(), ParserError> {
        let token = &self.tokens[self.position];

        let next = self
            .tokens
            .get(self.position + 1)
            .ok_or(ParserError::InvalidToken(InvalidToken {
                expected: vec![Token::Identifier("".into()), Token::Number(0)],
                received: None,
            }))?;

        match &**next {
            Token::Identifier(_) | Token::Number(_) => {
                let operand = match &**next {
                    Token::Identifier(identifier) => {
                        NumberOrIdentifier::Identifier(Rc::clone(identifier))
                    }
                    Token::Number(address) => {
                        if address < &100 && address > &0 {
                            NumberOrIdentifier::Number(*address)
                        } else {
                            Err(ParserError::AddressOutOfRange(*address))?
                        }
                    }
                    _ => unreachable!(),
                };

                let instruction = match &**token {
                    Token::Add => Instruction::Add(operand),
                    Token::Sub => Instruction::Sub(operand),
                    Token::Store => Instruction::Store(operand),
                    Token::Load => Instruction::Load(operand),
                    Token::Branch => Instruction::Branch(operand),
                    Token::BranchZero => Instruction::BranchZero(operand),
                    Token::BranchPositive => Instruction::BranchPositive(operand),
                    _ => unreachable!(),
                };

                self.program.instructions.push(instruction);
            }

            _ => Err(ParserError::InvalidToken(InvalidToken {
                expected: vec![Token::Identifier("".into()), Token::Number(0)],
                // TODO: this could error if loading from a file with no newline, fix
                received: Some(self.tokens.swap_remove(self.position + 1)),
            }))?,
        };

        self.position += 2;
        self.expect_newline()?;

        Ok(())
    }

    fn parse_identifier(&mut self, identifier: Rc<str>) -> Result<(), ParserError> {
        let next = self
            .tokens
            .get(self.position + 1)
            .ok_or(ParserError::InvalidToken(InvalidToken {
                expected: INSTRUCTIONS.to_vec(),
                received: None,
            }))?;

        match &**next {
            Token::Data => {
                if let Some(token) = self.tokens.get(self.position + 2) {
                    match &**token {
                        Token::Number(number) => {
                            if number >= &1000 || number <= &-1000 {
                                // TODO: fix error type
                                Err(ParserError::NumberOutOfRange(*number))?;
                            }
                            self.program
                                .instructions
                                .push(Instruction::Data(identifier, *number));
                            self.position += 3;
                        }
                        Token::Newline => {
                            self.program
                                .instructions
                                .push(Instruction::Data(identifier, 0));
                            self.position += 2;
                        }
                        _ => Err(ParserError::InvalidToken(InvalidToken {
                            expected: vec![Token::Number(0)],
                            received: Some(self.tokens.swap_remove(self.position + 2)),
                        }))?,
                    }
                } else {
                    self.program
                        .instructions
                        .push(Instruction::Data(identifier, 0));
                    self.position += 2;
                }
            }

            Token::Identifier(_) | Token::Number(_) => {
                Err(ParserError::InvalidToken(InvalidToken {
                    expected: INSTRUCTIONS.to_vec(),
                    received: Some(self.tokens.swap_remove(self.position)),
                }))?
            }

            Token::Newline => Err(ParserError::InvalidToken(InvalidToken {
                expected: {
                    let mut expected = INSTRUCTIONS.to_vec();
                    expected.push(Token::Data);
                    expected
                },
                received: None,
            }))?,

            _ => {
                self.program
                    .labels
                    .insert(identifier, self.program.instructions.len() as u8);
                self.position += 1;
            }
        }

        Ok(())
    }

    pub fn parse(mut self) -> Result<Program, ParserError> {
        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];

            match &**token {
                Token::Add
                | Token::Sub
                | Token::Store
                | Token::Load
                | Token::Branch
                | Token::BranchZero
                | Token::BranchPositive => self.parse_single_operand()?,

                Token::Halt | Token::Input | Token::Output => self.parse_no_operand()?,
                Token::Identifier(identifier) => self.parse_identifier(Rc::clone(identifier))?,

                Token::Newline => self.position += 1,

                _ => Err(ParserError::InvalidToken(InvalidToken {
                    expected: INSTRUCTIONS.to_vec(),
                    received: Some(self.tokens.swap_remove(self.position)),
                }))?,
            }
        }

        Ok(self.program)
    }
}
