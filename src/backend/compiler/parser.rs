use std::collections::HashMap;
use std::rc::Rc;

use crate::backend::compiler::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum NumberOrIdentifier {
    Number(i16),
    Identifier(String),
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
    Data(String, i16),
}

#[derive(PartialEq, Debug)]
pub struct Program {
    pub labels: HashMap<String, u8>,
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

#[derive(PartialEq, Debug)]
pub struct InvalidToken {
    pub expected: Vec<Token>,
    pub received: Option<Rc<Token>>,
    // TODO: add line and column
}

const INSTRUCTIONS: [Token; 11] = [
    Token::Halt,
    Token::Add,
    Token::Sub,
    Token::Store,
    Token::Load,
    Token::Branch,
    Token::BranchZero,
    Token::BranchZero,
    Token::BranchPositive,
    Token::Input,
    Token::Output,
];

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().map(|token| Rc::new(token)).collect(),
            position: 0,
            program: Program {
                labels: HashMap::new(),
                instructions: Vec::new(),
            },
        }
    }

    fn expect_newline(&mut self) -> Result<(), InvalidToken> {
        if let Some(token) = self.tokens.get(self.position) {
            let Token::Newline = &**token else {
                Err(InvalidToken {
                    expected: vec![Token::Newline],
                    received: Some(Rc::clone(token)),
                })?
            };
        }
        // TODO: figure out if i ever need this
        /* else {
        Err(InvalidToken {
        expected: vec![Token::Newline],
        received: None,
        })?;
        } */

        Ok(())
    }

    fn parse_no_operand(&mut self) -> Result<(), InvalidToken> {
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

    fn parse_single_operand(&mut self) -> Result<(), InvalidToken> {
        let token = &self.tokens[self.position];

        let next = self.tokens.get(self.position + 1).ok_or(InvalidToken {
            expected: vec![Token::Identifier("".to_string()), Token::Number(0)],
            received: None,
        })?;

        match &**next {
            Token::Identifier(_) | Token::Number(_) => {
                let operand = match &**next {
                    Token::Identifier(identifier) => {
                        NumberOrIdentifier::Identifier(identifier.to_owned())
                    }
                    Token::Number(number) => NumberOrIdentifier::Number(*number),
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

            _ => Err(InvalidToken {
                expected: vec![],
                received: Some(self.tokens.swap_remove(self.position + 1)),
            })?,
        };

        self.position += 2;
        self.expect_newline()?;

        Ok(())
    }

    fn parse_identifier(&mut self, identifier: String) -> Result<(), InvalidToken> {
        let next = self.tokens.get(self.position + 1).ok_or(InvalidToken {
            expected: INSTRUCTIONS.to_vec(),
            received: None,
        })?;

        match &**next {
            Token::Data => {
                if let Some(token) = self.tokens.get(self.position + 2) {
                    match &**token {
                        Token::Number(number) => {
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
                        _ => Err(InvalidToken {
                            expected: vec![Token::Number(0)],
                            received: Some(self.tokens.swap_remove(self.position + 2)),
                        })?,
                    }
                } else {
                    self.program
                        .instructions
                        .push(Instruction::Data(identifier, 0));
                    self.position += 2;
                }
            }

            Token::Identifier(_) | Token::Number(_) => Err(InvalidToken {
                expected: INSTRUCTIONS.to_vec(),
                received: Some(self.tokens.swap_remove(self.position)),
            })?,

            Token::Newline => Err(InvalidToken {
                expected: {
                    let mut expected = INSTRUCTIONS.to_vec();
                    expected.push(Token::Data);
                    expected
                },
                received: None,
            })?,

            _ => {
                self.program
                    .labels
                    .insert(identifier, self.program.instructions.len() as u8);
                self.position += 1;
            }
        }

        Ok(())
    }

    pub fn parse(mut self) -> Result<Program, InvalidToken> {
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
                Token::Identifier(identifier) => self.parse_identifier(identifier.to_string())?,

                Token::Newline => self.position += 1,

                _ => Err(InvalidToken {
                    expected: INSTRUCTIONS.to_vec(),
                    received: Some(self.tokens.swap_remove(self.position)),
                })?,
            }
        }

        Ok(self.program)
    }
}
