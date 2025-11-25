use crate::backend::compiler::parser::{Instruction, NumberOrIdentifier, Program};
use std::fmt::{self, Display};
use std::rc::Rc;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Operand {
    Address(u8),
    Number(i16),
}

impl From<Operand> for i16 {
    fn from(operand: Operand) -> i16 {
        match operand {
            Operand::Address(address) => address.into(),
            Operand::Number(number) => number,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Location {
    pub opcode: u8,
    pub operand: Operand, // "...the Accumulator holds 3 digits and a sign (-999 to 999)"
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>3}",
            self.opcode as i16 * 100 + <i16>::from(self.operand)
        )
    }
}

impl Location {
    pub fn new(opcode: u8, operand: Operand) -> Location {
        Location { opcode, operand }
    }
}

#[derive(Debug)]
pub struct InvalidIdentifier {
    pub identifier: Rc<str>, // this does not need to be mutable
}

fn get_operand(program: &Program, instruction: &Instruction) -> Result<Operand, InvalidIdentifier> {
    match instruction {
        Instruction::Branch(number_or_identifier)
        | Instruction::BranchZero(number_or_identifier)
        | Instruction::BranchPositive(number_or_identifier) => match number_or_identifier {
            NumberOrIdentifier::Number(number) => Ok(Operand::Number(*number)),
            NumberOrIdentifier::Identifier(identifier) => {
                let label = program.labels.get(identifier).ok_or(InvalidIdentifier {
                    identifier: identifier.to_owned(),
                })?;

                Ok(Operand::Address(*label))
            }
        },

        Instruction::Add(number_or_identifier)
        | Instruction::Sub(number_or_identifier)
        | Instruction::Store(number_or_identifier)
        | Instruction::Load(number_or_identifier) => match number_or_identifier {
            NumberOrIdentifier::Number(number) => Ok(Operand::Number(*number)),
            NumberOrIdentifier::Identifier(identifier) => {
                let mut number = None;

                for (i, instruction) in program.instructions.iter().enumerate() {
                    if let Instruction::Data(label, _) = instruction
                        && label == identifier
                    {
                        number = Some(i as u8);
                    }
                }

                Ok(Operand::Address(number.unwrap()))
            }
        },

        _ => unreachable!(),
    }
}

impl TryFrom<Program> for [Location; 100] {
    type Error = InvalidIdentifier;

    fn try_from(program: Program) -> Result<Self, InvalidIdentifier> {
        let mut code = [Location::new(0, Operand::Number(0)); 100];

        // Use a for loop to avoid dynamic allocations
        for (i, instruction) in program.instructions.iter().enumerate() {
            let location = match instruction {
                Instruction::Halt => Location::new(0, Operand::Number(0)),
                Instruction::Add(_) => Location::new(1, get_operand(&program, instruction)?),
                Instruction::Sub(_) => Location::new(2, get_operand(&program, instruction)?),
                Instruction::Store(_) => Location::new(3, get_operand(&program, instruction)?),
                // re: code 4, "This code is unused and gives an error."
                Instruction::Load(_) => Location::new(5, get_operand(&program, instruction)?),
                Instruction::Branch(_) => Location::new(6, get_operand(&program, instruction)?),
                Instruction::BranchZero(_) => Location::new(7, get_operand(&program, instruction)?),
                Instruction::BranchPositive(_) => {
                    Location::new(7, get_operand(&program, instruction)?)
                }
                Instruction::Input => Location::new(9, Operand::Number(1)),
                Instruction::Output => Location::new(9, Operand::Number(2)),
                Instruction::Data(_, number) => Location::new(0, Operand::Number(*number)),
            };
            code[i] = location;
        }

        Ok(code)
    }
}
