use crate::backend::compiler::parser::{Instruction, Operand, Program};
use std::error::Error;
use std::fmt::{self, Display};
use std::rc::Rc;

// An instruction has an opcode and an operand (e.g. STA as the opcode and 10 as the operand)
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct InstructionLocation {
    pub opcode: u8,
    pub operand: u8,
}

// Not necessary but makes code cleaner
impl InstructionLocation {
    pub fn new(opcode: u8, operand: u8) -> Self {
        Self { opcode, operand }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Location {
    Instruction(InstructionLocation),
    Data(i16), // Really a number between -999 and 999 inclusive
}

// This is used in the frontend for displaying memory locations
impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Instruction(instruction) => {
                write!(
                    f,
                    // This format specifier forces the number to take up 4 digits (e.g. 0 -> 0000, 10 -> 0010)
                    "{:04}",
                    // This puts the opcode in the hundreds place and the operand in the tens and ones place
                    instruction.opcode as i16 * 100 + <i16>::from(instruction.operand)
                )
            }
            Location::Data(number) => write!(f, "{:04}", number),
        }
    }
}

// The only possible error while generating code, occurs when the user uses an identifier that isn't defined later in the code
#[derive(Debug)]
pub struct InvalidIdentifier {
    pub identifier: Rc<str>,
}

// User-friendly error message
impl Display for InvalidIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Encountered an error during code generation...\nInvalid identifier `{}`",
            self.identifier
        )
    }
}

// Ditto impl Error for InvalidCharacter {} comment
impl Error for InvalidIdentifier {}

fn get_operand(instruction: &Instruction, program: &Program) -> Result<u8, InvalidIdentifier> {
    match instruction {
        Instruction::Branch(operand)
        | Instruction::BranchZero(operand)
        | Instruction::BranchPositive(operand) => match operand {
            Operand::Number(number) => Ok(*number as u8),
            Operand::Identifier(identifier) => {
                let label = program.labels.get(identifier).ok_or(InvalidIdentifier {
                    identifier: identifier.to_owned(),
                })?;
                Ok(*label)
            }
        },

        Instruction::Add(operand)
        | Instruction::Sub(operand)
        | Instruction::Store(operand)
        | Instruction::Load(operand) => match operand {
            Operand::Number(number) => Ok(*number as u8),
            Operand::Identifier(identifier) => {
                // Assume there is no identifier
                let mut number = None;

                program
                    .instructions
                    .iter()
                    .enumerate()
                    .for_each(|(i, instruction)| {
                        if let Instruction::Data(label, _) = instruction
                            && label == identifier
                        {
                            // Get the memory address from the position in the program
                            number = Some(i as u8)
                        }
                    });

                // Return the number or bubble up an error
                number.ok_or(InvalidIdentifier {
                    identifier: Rc::clone(identifier),
                })
            }
        },

        _ => unreachable!(),
    }
}

// A macro that makes turning instructions into machine code easier, used over a function as macros are expaned at compile time, thus there is no stack overhead
macro_rules! instruction_location {
    ($a:expr,$b:expr,$c:expr) => {{ Location::Instruction(InstructionLocation::new($a, get_operand($b, $c)?)) }};
}

// Since code generation requires no attributes, there is no point using a struct and I can take advantage of the Rust standard library traits
impl TryFrom<Program> for [Location; 100] {
    type Error = InvalidIdentifier;

    fn try_from(program: Program) -> Result<Self, Self::Error> {
        let mut code = [Location::Data(0); 100];

        // Use a for loop to avoid dynamic allocations
        for (i, instruction) in program.instructions.iter().enumerate() {
            let location = match instruction {
                Instruction::Halt => Location::Instruction(InstructionLocation::new(0, 0)),
                Instruction::Add(_) => instruction_location!(1, instruction, &program),
                Instruction::Sub(_) => instruction_location!(2, instruction, &program),
                Instruction::Store(_) => instruction_location!(3, instruction, &program),
                // re: code 4, "This code is unused and gives an error."
                Instruction::Load(_) => instruction_location!(5, instruction, &program),
                Instruction::Branch(_) => instruction_location!(6, instruction, &program),
                Instruction::BranchZero(_) => instruction_location!(7, instruction, &program),
                Instruction::BranchPositive(_) => instruction_location!(8, instruction, &program),
                Instruction::Input => Location::Instruction(InstructionLocation::new(9, 1)),
                Instruction::Output => Location::Instruction(InstructionLocation::new(9, 2)),
                Instruction::Data(_, number) => Location::Data(*number),
            };

            code[i] = location;
        }

        Ok(code)
    }
}
