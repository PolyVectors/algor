use crate::backend::compiler::parser::{Instruction, NumberOrIdentifier, Program};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Operand {
    Address(u8),
    Number(i16),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Location {
    pub opcode: u8,
    pub operand: Operand, // "...the Accumulator holds 3 digits and a sign (-999 to 999)"
}

impl Location {
    pub fn new(opcode: u8, operand: Operand) -> Location {
        Location { opcode, operand }
    }
}

fn get_operand(program: &Program, instruction: &Instruction) -> Operand {
    match instruction {
        Instruction::Halt | Instruction::Input | Instruction::Output => unreachable!(),

        Instruction::Branch(number_or_identifier)
        | Instruction::BranchZero(number_or_identifier)
        | Instruction::BranchPositive(number_or_identifier) => match number_or_identifier {
            NumberOrIdentifier::Number(number) => Operand::Number(*number),
            NumberOrIdentifier::Identifier(identifier) => {
                // TODO: this should return an error maybe InvalidIdentifier
                let label = program.labels.get(identifier).unwrap();
                Operand::Address(*label)
            }
        },

        Instruction::Add(number_or_identifier)
        | Instruction::Sub(number_or_identifier)
        | Instruction::Store(number_or_identifier)
        | Instruction::Load(number_or_identifier) => match number_or_identifier {
            NumberOrIdentifier::Number(number) => Operand::Number(*number),
            NumberOrIdentifier::Identifier(identifier) => {
                let mut number = None;

                for (i, instruction) in program.instructions.iter().enumerate() {
                    if let Instruction::Data(label, _) = instruction
                        && label == identifier
                    {
                        number = Some(i as u8);
                    }
                }

                // TODO:: ditto InvalidIdentifier error
                Operand::Address(number.unwrap())
            }
        },

        _ => unreachable!(),
    }
}

// TODO: change into TryFrom
impl From<Program> for [Location; 100] {
    fn from(program: Program) -> Self {
        // TODO: this could be an enumerated map on program
        let mut code = [Location::new(0, Operand::Number(0)); 100];

        for (i, instruction) in program.instructions.iter().enumerate() {
            let location = match instruction {
                Instruction::Halt => Location::new(0, Operand::Number(0)),
                Instruction::Add(_) => Location::new(1, get_operand(&program, instruction)),
                Instruction::Branch(_) => Location::new(6, get_operand(&program, instruction)),
                Instruction::Data(_, number) => Location::new(0, Operand::Number(*number)),
                _ => todo!(),
            };
            code[i] = location;
        }

        code
    }
}
