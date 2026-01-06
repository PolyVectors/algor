// TODO: merge with runtime

use std::error::Error;
use std::fmt::Display;

use crate::backend::compiler::generator::{InstructionLocation, Location};
use crate::shared::runtime::Event;

#[derive(PartialEq, Clone, Debug)]
pub struct Computer {
    pub program_counter: u8,
    pub accumulator: i16,
    pub current_instruction_register: u8,
    pub memory_address_register: u8,
    pub memory_data_register: i16,
    pub memory: [Location; 100],
}

impl Default for Computer {
    fn default() -> Self {
        Self {
            program_counter: 0,
            accumulator: 0,
            current_instruction_register: 0,
            memory_address_register: 0,
            memory_data_register: 0,
            memory: [Location::Data(0); 100],
        }
    }
}

// TODO: add info and impl struct, add line and column number
#[derive(Debug)]
pub enum InvalidLocation {
    ExpectedInstruction,
    InvalidOpcode,
}

impl Error for InvalidLocation {}

impl Display for InvalidLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            InvalidLocation::ExpectedInstruction => {
                "Ran into data memory whilst running code; did you forget to halt?"
            }
            _ => "TODO",
        };

        write!(f, "Encountered an error at runtime...\n{text}")
    }
}

impl Computer {
    pub fn reset(&mut self) {
        self.program_counter = 0;
        self.accumulator = 0;
        self.current_instruction_register = 0;
        self.memory_address_register = 0;
        self.memory_data_register = 0;
    }

    // TODO: reverse step feature?
    pub fn step(&mut self) -> Result<Event, InvalidLocation> {
        let Location::Instruction(instruction) = self.memory[self.program_counter as usize] else {
            if self.memory[self.program_counter as usize] == Location::Data(0) {
                return Ok(Event::Halt);
            } else {
                return Err(InvalidLocation::ExpectedInstruction);
            }
        };

        self.current_instruction_register = instruction.opcode;
        self.memory_address_register = instruction.operand;

        match instruction.opcode {
            0 => return Ok(Event::Halt),

            opcode @ (1 | 2 | 3 | 5) => {
                if let Location::Data(number) = self.memory[self.memory_address_register as usize] {
                    self.memory_data_register = self.memory[self.memory_address_register as usize]
                        .to_string()
                        .parse()
                        .unwrap_or(0);

                    match opcode {
                        1 | 2 => {
                            self.accumulator += if opcode == 1 { number } else { -number };
                        }
                        3 => {
                            self.memory[self.memory_address_register as usize] =
                                Location::Data(self.accumulator);
                        }

                        5 => self.accumulator = number,

                        _ => unreachable!(),
                    }
                } else {
                    return Err(InvalidLocation::InvalidOpcode);
                }
            }

            6 | 7 | 8 => {
                let condition = match instruction.opcode {
                    6 => true,
                    7 => self.accumulator == 0,
                    8 => self.accumulator >= 0,
                    _ => unreachable!(),
                };
                if condition {
                    self.program_counter = instruction.operand;
                    return Ok(Event::Continue);
                }
            }

            9 => {
                self.program_counter += 1;

                if instruction.operand == 1 {
                    return Ok(Event::Input);
                } else {
                    return Ok(Event::Output(format!("{}", self.accumulator).into()));
                }
            }

            _ => unreachable!(),
        }

        self.program_counter += 1;
        Ok(Event::Continue)
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::compiler;
    use crate::shared::vm::Computer;

    //2.1
    #[test]
    fn virtual_machine() {
        let source = r#"test OUT
        BRA test"#;

        let mut computer = Computer::default();
        computer.memory = compiler::compile(source).unwrap();

        computer.step().unwrap();
        computer.step().unwrap();
        computer.step().unwrap();

        assert_eq!(computer, Computer::default());
    }
}
