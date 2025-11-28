use crate::backend::compiler::parser::{Instruction, NumberOrIdentifier, Program};
use std::fmt::{self, Display};
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct InstructionLocation {
    opcode: u8,
    operand: u8,
}

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

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Instruction(instruction) => {
                write!(
                    f,
                    "{:0>3}",
                    instruction.opcode as i16 * 100 + <i16>::from(instruction.operand)
                )
            }
            Location::Data(number) => write!(f, "{number}"),
        }
    }
}

#[derive(Debug)]
pub struct InvalidIdentifier {
    pub identifier: Rc<str>,
}

fn get_operand(instruction: &Instruction, program: &Program) -> Result<u8, InvalidIdentifier> {
    match instruction {
        Instruction::Branch(number_or_identifier)
        | Instruction::BranchZero(number_or_identifier)
        | Instruction::BranchPositive(number_or_identifier) => match number_or_identifier {
            NumberOrIdentifier::Number(number) => Ok(*number as u8), // TODO: error if > 255
            NumberOrIdentifier::Identifier(identifier) => {
                let label = program.labels.get(identifier).ok_or(InvalidIdentifier {
                    identifier: identifier.to_owned(),
                })?;

                Ok(*label)
            }
        },

        Instruction::Add(number_or_identifier)
        | Instruction::Sub(number_or_identifier)
        | Instruction::Store(number_or_identifier)
        | Instruction::Load(number_or_identifier) => match number_or_identifier {
            NumberOrIdentifier::Number(number) => Ok(*number as u8), // TODO: ditto error
            NumberOrIdentifier::Identifier(identifier) => {
                let mut number = None;

                for (i, instruction) in program.instructions.iter().enumerate() {
                    if let Instruction::Data(label, _) = instruction
                        && label == identifier
                    {
                        number = Some(i as u8);
                    }
                }

                Ok(number.unwrap())
            }
        },

        _ => unreachable!(),
    }
}

macro_rules! instruction_location {
    ($a:expr,$b:expr,$c:expr) => {{ Location::Instruction(InstructionLocation::new($a, get_operand($b, $c)?)) }};
}

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
