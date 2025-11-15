use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    rc::{self, Rc},
};

// Define the tokens that the lexer will generate
#[derive(PartialEq, Clone, Debug)] // Implement the ability to compare two tokens for testing
pub enum Token {
    Halt,           // HLT, COB
    Add,            // ADD
    Sub,            // SUB
    Store,          // STA, STO
    Load,           // LDA
    Branch,         // BRA
    BranchZero,     // BRZ
    BranchPositive, // BRP
    Input,          // INP
    Output,         // OUT
    Data,           // DAT

    Number(usize), // A 64-bit or 32-bit unsigned integer (depends on operating system and/or processor architecture)
    Identifier(String), // A mutable string
    Newline,       // A newline (\n or potentially \r\n on windows)
}

// The lexer struct and the attributes associated with it
pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
    tokens: Vec<Token>,
}

// The error the lexer will throw
#[derive(PartialEq, Debug)]
pub struct InvalidCharacter {
    pub character: char,
    pub line_column: (usize, usize),
}

// Implements the trait that will show the error to the user in a readable format
impl fmt::Display for InvalidCharacter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid character '{}' while lexing ({}:{})",
            self.character, self.line_column.0, self.line_column.1
        )
    }
}

// Implements the methods associated with the Lexer struct
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
            tokens: Vec::new(),
        }
    }

    // Loop through the input and add a string value (either an identifier (Token::Identifier(string)) or an instruction token (Token::Halt, Token::Add, etc.) to the list of tokens
    fn lex_string(&mut self) {
        let mut string = String::new();

        // Loop through the input until we reach a non-character, appending each character to the string variable along the way
        // TODO: possibly could use iter.find() and some sort of slicing to avoid a loop
        while self.position < self.source.len() {
            let character = self.source.as_bytes()[self.position] as char;
            match character {
                'A'..='Z' | 'a'..='z' => {
                    string.push(character);
                    self.position += 1;
                }
                _ => break,
            }
        }

        // Match the input to the correct instruction token, otherwise, create an identifier
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

        // Push the token onto the list
        self.tokens.push(token);
    }

    // Loop through the input and add a number (Token::Number(usize)) to the list of tokens
    fn lex_number(&mut self) {
        let mut number = String::new();

        // Loop through the input until we reach a non-digit, appending each character to the number variable along the way
        // TODO: ditto lex_string
        while self.position < self.source.len() {
            let character = self.source.as_bytes()[self.position] as char;
            match character {
                '0'..='9' => {
                    number.push(character);
                    self.position += 1;
                }
                _ => break,
            }
        }

        // Turn the number into a usize and push it onto the list
        self.tokens.push(Token::Number(number.parse().unwrap_or(0)))
    }

    // Take ownership of the struct and loop through the input and turn it into a list of tokens
    pub fn lex(mut self) -> Result<Vec<Token>, InvalidCharacter> {
        while self.position < self.source.len() {
            let character = self.source.as_bytes()[self.position] as char;

            /* If the character is the alphabet, call the lex_string method
            If it is a number, call the lex_number method
            If it is a newline (\n), add a newline token and increment the position
            If it is whitespace, increase the position
            Otherwise, there must be an invalid character, bubble up the error */

            match character {
                'A'..='Z' | 'a'..='z' => self.lex_string(),
                '0'..='9' => self.lex_number(),

                '\n' => {
                    self.tokens.push(Token::Newline);
                    self.position += 1;
                }
                ' ' | '\t' => self.position += 1,

                _ => {
                    // Calculate the line number by slicing the input from the beginning to the current position and counting the newlines
                    let line = &self.source[0..self.position]
                        .chars()
                        .filter(|c| *c == '\n')
                        .count()
                        + 1;

                    /* Calculate the column number by subtracting the length of the input from the beginning to the current position from the position of the last newline or 0
                    If there is no "last newline", counterintuitively default to 0 (I don't default to 1 as that could cause an integer underflow in the calculation, i.e. 0 - 1 )*/
                    let column = &self.source[0..self.position].len()
                        - &self.source[0..self.position].rfind('\n').unwrap_or(0);

                    return Err(InvalidCharacter {
                        character,
                        // Account for the default 0 value (see previous comment) by returning 1 (as in first character of the column)
                        line_column: (line, if column == 0 { 1 } else { column }),
                    });
                }
            }
        }

        // Consume the struct and return the tokens
        Ok(self.tokens)
    }
}

#[derive(PartialEq, Debug)]
pub enum AddressOrNumber {
    Address(usize),
    Number(usize),
}

// Create the Instruction enum, similar to the Token enum, but bundling together the opcode and operand(s)
#[derive(PartialEq, Debug)]
pub enum Instruction {
    Halt,
    Add(AddressOrNumber),
    Sub(AddressOrNumber),
    Store(AddressOrNumber),
    Load(AddressOrNumber),
    Branch(AddressOrNumber),
    BranchZero(AddressOrNumber),
    BranchPositive(AddressOrNumber),
    Input,
    Output,
}

#[derive(PartialEq, Debug)]
pub struct Program {
    pub labels: HashMap<String, usize>,
    pub memory: [usize; 100],
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    program: Program,
}

#[derive(PartialEq, Debug)]
pub struct InvalidToken {
    expected: Vec<Token>,
    received: Option<Token>,
    // TODO: add line and column
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            program: Program {
                labels: HashMap::new(),
                memory: [0; 100],
                instructions: Vec::new(),
            },
        }
    }

    fn expect_newline(&mut self) -> Result<(), InvalidToken> {
        if let Some(token) = self.tokens.get(self.position) {
            let Token::Newline = token else {
                Err(InvalidToken {
                    expected: vec![Token::Newline],
                    received: Some(self.tokens.swap_remove(self.position)),
                })?
            };
        } else {
            Err(InvalidToken {
                expected: vec![Token::Newline],
                received: None,
            })?;
        }

        Ok(())
    }

    fn parse_no_operands(&mut self) -> Result<(), InvalidToken> {
        let token = &self.tokens[self.position];

        let instruction = match token {
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
            expected: vec![],
            received: None,
        })?;

        match next {
            Token::Identifier(_) | Token::Number(_) => {
                let operand = match next {
                    Token::Identifier(identifier) => {
                        // TODO: this is fucked
                        AddressOrNumber::Address(*self.program.labels.get(identifier).unwrap())
                    }
                    Token::Number(number) => AddressOrNumber::Number(*number),
                    _ => unreachable!(),
                };

                let instruction = match token {
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
            expected: vec![], // TODO: create INSTRUCTION_TOKENS constant vector/array
            received: None,
        })?;

        // TODO: this calculation will not work as the instructions haven't been parsed yet
        let position = if self.program.instructions.len() != 0 {
            self.program.instructions.len()
        } else {
            0
        };

        self.program.labels.insert(identifier, position);

        match next {
            Token::Data => {
                if let Some(token) = self.tokens.get(self.position + 2)
                    && let Token::Number(number) = token
                {
                    self.program.memory[position] = *number;
                    self.position += 1;
                }

                self.position += 2;
            }

            Token::Identifier(_) | Token::Number(_) => {
                Err(InvalidToken {
                    expected: vec![], // TODO: ditto constant vector
                    received: Some(self.tokens.swap_remove(self.position)),
                })?
            }

            _ => {
                self.position += 1;
            }
        }

        Ok(())
    }

    pub fn parse(mut self) -> Result<Program, InvalidToken> {
        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];

            /* TODO: is it possible to avoid this clone?
            Rc<Vec<Token>> doesn't work due to no mutability for swap_remove,
            Rc<Cell<Vec<Token>>> doesn't work due to Vec not implementing Copy,
            Rc<RefCell<Vec<Token>>> doesn't work as multible borrows (one for loop, and one for inside the function)
            */
            if let Token::Identifier(identifier) = token.clone() {
                self.parse_identifier(identifier)?
            }

            self.position += 1;
        }

        self.position = 0;

        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];

            match token {
                Token::Add
                | Token::Sub
                | Token::Store
                | Token::Load
                | Token::Branch
                | Token::BranchZero
                | Token::BranchPositive => self.parse_single_operand()?,

                Token::Halt | Token::Input | Token::Output => self.parse_no_operands()?,

                Token::Newline | Token::Identifier(_) => self.position += 1,

                _ => Err(InvalidToken {
                    expected: vec![], // TODO: ditto constant vector
                    received: Some(self.tokens.swap_remove(self.position)),
                })?,
            }
        }

        Ok(self.program)
    }
}
