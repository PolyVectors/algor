use crate::backend::compiler::parser::{Instruction, Program};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Location {
    pub opcode: u8,
    pub operand: i16, // "...the Accumulator holds 3 digits and a sign (-999 to 999)"
}

impl Location {
    pub fn new(opcode: u8, operand: i16) -> Location {
        Location { opcode, operand }
    }
}

pub struct Generator {
    program: Program,
}

impl Generator {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    pub fn generate(self) -> [Location; 100] {
        // could potentially use an enumerated map on instructions and try_into to directly collect into code
        let code = [Location::new(0, 0); 100];

        /*
        for (i, instruction) in self.program.instructions.iter().enumerate() {
            ...
        }
        */

        code
    }
}
