use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};
use std::rc::Rc;

use crate::backend::compiler::lexer::Token;

// Create the enum that defines if an instruction has a number or an identifier as the operand (e.g. LDA 10 would be a number while LDA ONE would be an identifier)
#[derive(PartialEq, Debug)]
pub enum Operand {
    Number(i16),
    Identifier(Rc<str>),
}

// Create the Instruction enum, similar to the Token enum, but bundling together the opcode and operand(s)
#[derive(PartialEq, Debug)]
pub enum Instruction {
    Halt,                    // HLT
    Add(Operand),            // ADD X
    Sub(Operand),            // SUB X
    Store(Operand),          // STA/STO X
    Load(Operand),           // LDA X
    Branch(Operand),         // BRA X
    BranchZero(Operand),     // BRZ X
    BranchPositive(Operand), // BRZ X
    Input,                   // INP
    Output,                  // OUT
    Data(Rc<str>, i16),      // X DAT Y
}

// This struct bundles together the instructions that will be returned from the parsing process and the labels that were defined in the program, useful when checking if an identifier exists later in the compilation process.
#[derive(PartialEq, Debug)]
pub struct Program {
    pub labels: HashMap<Rc<str>, u8>,
    pub instructions: Vec<Instruction>,
}

// The struct with the attributes needed to turn tokens into instructions
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Rc<Token>>,
    position: usize,
    program: Program,
}

// One of the errors that can happen while parsing, indicating a mismatch between the expected token and the token received (e.g. INP OUT breaks the rules of the language as an instruction should always be followed by a number, an identifier, or nothing - not another instruction)
#[derive(PartialEq, Debug)]
pub struct InvalidToken {
    pub expected: Vec<Token>,
    // The received type is an option here as it is a possibility the program ends when an extra operand is needed (e,g. BRA)
    pub received: Option<Rc<Token>>,
}

// Allows for printing out the aforementioned error in a user-friendly way
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
                    // This is a fancy functional way of looping and combining a result from the previous iteration, easily turning a list of results into plain english
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

// Ditto impl Error for InvalidCharacter {} comment
impl Error for InvalidToken {}

// The error type for the parser, this composes the InvalidToken type above and does range checks on numbers and addresses (i.e. any addresses over 99 are invalid as there are only 100 memory locations)
#[derive(PartialEq, Debug)]
pub enum ParserError {
    InvalidToken(InvalidToken),
    NumberOutOfRange(i16),
    AddressOutOfRange(i16),
}

// Ditto impl Display for InvalidToken comment, now with a helpful message for the range check errors
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

// Ditto impl Error for InvalidCharacter {} comment
impl Error for ParserError {}

// A reusable list of all the instructions, useful for avoiding repetition when bubbling up errors
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

// Methods for the Parser struct
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

    // A helper function that will bubble up an error if the current token isn't a newline
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

    // Parse an instruction where no operands are expected (i.e. HLT/COB, INP, COB)
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

    // Parse an instruction where one operand is expected (i.e. every instruction but HLT/COB, INP, COB, and DAT)
    fn parse_single_operand(&mut self) -> Result<(), ParserError> {
        let token = &self.tokens[self.position];

        // Get the next token if it exists, otherwise bubble up an error
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
                    Token::Identifier(identifier) => Operand::Identifier(Rc::clone(identifier)),
                    Token::Number(address) => {
                        // Ensure the address is less than 100 due to the 100 memory location limitation
                        if address < &100 && address > &0 {
                            Operand::Number(*address)
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

            // Throw an error if the token doesn't fall under the Operand category (i.e. an Identifier or Number)
            _ => Err(ParserError::InvalidToken(InvalidToken {
                expected: vec![Token::Identifier("".into()), Token::Number(0)],
                received: Some(self.tokens.swap_remove(self.position + 1)),
            }))?,
        };

        self.position += 2;
        self.expect_newline()?;

        Ok(())
    }

    // Match identifiers (either labels for branches - e.g. loop BRA loop - or labels preceding memory locations - e.g. A DAT 100)
    fn parse_identifier(&mut self, identifier: Rc<str>) -> Result<(), ParserError> {
        // Get the next token if it exists, otherwise bubble up an error
        let next = self
            .tokens
            .get(self.position + 1)
            .ok_or(ParserError::InvalidToken(InvalidToken {
                expected: INSTRUCTIONS.to_vec(),
                received: None,
            }))?;

        match &**next {
            // If the next token is for storing data at a memory location, this matches the second case in the method comment (i.e. labels preceding memory locations)
            Token::Data => {
                // Look ahead for data (a number) to put at the memory location, if there is none or there is an unexpected token, set the data to the default, 0
                if let Some(token) = self.tokens.get(self.position + 2) {
                    match &**token {
                        Token::Number(number) => {
                            // Range check as numbers cannot go over or under 1000
                            if number >= &1000 || number <= &-1000 {
                                Err(ParserError::NumberOutOfRange(*number))?;
                            }
                            self.program
                                .instructions
                                .push(Instruction::Data(identifier, *number));
                            self.position += 3;
                        }

                        // Treat newlines the same as having no token there at all in this case
                        Token::Newline => {
                            self.program
                                .instructions
                                .push(Instruction::Data(identifier, 0));
                            self.position += 2;
                        }

                        // Ditto unexpected token comment
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

            // If the next token is an instruction operand (e.g. A B, A 10, etc.) that breaks the rules of the program so throw an error
            Token::Identifier(_) | Token::Number(_) => {
                Err(ParserError::InvalidToken(InvalidToken {
                    expected: INSTRUCTIONS.to_vec(),
                    received: Some(self.tokens.swap_remove(self.position)),
                }))?
            }

            // If the next token is a newline, the identifier is by itself (e.g. A) which makes no logical sense in an LMC program so throw an error
            Token::Newline => Err(ParserError::InvalidToken(InvalidToken {
                expected: {
                    let mut expected = INSTRUCTIONS.to_vec();
                    expected.push(Token::Data);
                    expected
                },
                received: None,
            }))?,

            // Match statements match the earliest case found, if none of the cases above match, then the next token must be an instruction meaning the current token is a label for branch instructions
            _ => {
                self.program
                    .labels
                    .insert(identifier, self.program.instructions.len() as u8);
                self.position += 1;
            }
        }

        Ok(())
    }

    // Loop through and match tokens, making calls to methods above to parse an entire program
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
