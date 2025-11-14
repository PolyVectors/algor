use std::{collections::HashMap, fmt};

// Define the tokens that the lexer will generate
#[repr(u8)] // Represent a token as a single byte for optimisation
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

    // Take ownership of the struct and l`oop through the input and turn it into a list of tokens
    pub fn lex(mut self) -> Result<Vec<Token>, InvalidCharacter> {
        while self.position < self.source.len() {
            let character = self.source.as_bytes()[self.position] as char;

            /* If the character is the alphabet, call the lex_string() method
            If it is a number, call the lex_number() method
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

// Create the Instruction enum, similar to the Token enum, but bundling together the opcode and operand(s)
#[derive(PartialEq, Debug)]
pub enum Instruction {
    Halt,
    Add(usize),
    Sub(usize),
    Store(usize),
    Load(usize),
    Branch(usize),
    BranchZero(usize),
    BranchPositive(usize),
    Input,
    Output,
    Data(usize, usize),
}

#[derive(PartialEq, Debug)]
pub struct Program {
    pub labels: HashMap<usize, usize>,
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
    expected: Token,
    received: Token,
    // TODO: add line and column
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            program: Program {
                labels: HashMap::new(),
                instructions: Vec::new(),
            },
        }
    }

    fn parse_no_operands(&mut self) {
        let instruction = match self.tokens[self.position] {
            Token::Halt => Instruction::Halt,
            Token::Input => Instruction::Input,
            Token::Output => Instruction::Output,
            _ => unreachable!(),
        };

        self.program.instructions.push(instruction);
        self.position += 1;
    }

    fn parse_single_operand(&mut self) {}

    fn parse_label_or_data(&mut self) {
        if let Token::Identifier(identifier) = &self.tokens[self.position] {
            println!("{identifier}");
        }
    }

    pub fn parse(mut self) -> Result<Program, InvalidToken> {
        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];

            match token {
                Token::Identifier(_) => self.parse_label_or_data(),
                Token::Halt | Token::Input | Token::Output => self.parse_no_operands(),

                Token::Add
                | Token::Sub
                | Token::Store
                | Token::Load
                | Token::Branch
                | Token::BranchZero
                | Token::BranchPositive => self.parse_single_operand(),

                _ => {}
            }

            if self.tokens[self.position] != Token::Newline {
                return Err(InvalidToken {
                    expected: Token::Newline,
                    received: self.tokens[self.position].clone(),
                });
            }
        }

        Ok(self.program)
    }
}
