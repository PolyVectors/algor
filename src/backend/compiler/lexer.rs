use std::fmt;
use std::rc::Rc;

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

    Number(i16), // A 64-bit or 32-bit unsigned integer (depends on operating system and/or processor architecture)
    Identifier(Rc<str>), // A heap-allocated immutable string
    Newline,     // A newline (\n or potentially \r\n on windows)
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
        // Loop through the input until we reach a non-character, saving the position for later, if there is only characters after, take the position of the last character
        let end = self
            .source
            .as_bytes()
            .iter()
            .skip(self.position)
            .position(|c| !c.is_ascii_alphabetic())
            .unwrap_or(self.source.len() - self.position);

        // Take a string slice of the current position up until the end of the identifier and make it a String struct
        let string: Rc<str> = self.source[self.position..end + self.position].into();

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
        self.position += end;
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
                    let column = self.source[0..self.position].len()
                        - self.source[0..self.position].rfind('\n').unwrap_or(0);

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
