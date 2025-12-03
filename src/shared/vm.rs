// TODO: merge with runtime

use std::error::Error;
use std::fmt::Display;

use crate::backend::compiler::generator::{InstructionLocation, Location};
use crate::shared::runtime::Event;

#[derive(PartialEq, Debug)]
pub struct Computer {
    pub program_counter: u8,
    pub accumulator: i16,
    pub current_instruction_register: u8,
    pub memory_address_register: u8,
    pub memory: [Location; 100],
}

impl Default for Computer {
    fn default() -> Self {
        Self {
            program_counter: 0,
            accumulator: 0,
            current_instruction_register: 0,
            memory_address_register: 0,
            memory: [Location::Data(0); 100],
        }
    }
}

// TODO: add info and impl struct, add line and column number
#[derive(Debug)]
pub struct InvalidLocation;

impl Error for InvalidLocation {}

impl Display for InvalidLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}

impl Computer {
    pub fn reset(&mut self) {
        self.program_counter = 0;
        self.accumulator = 0;
        self.current_instruction_register = 0;
        self.memory_address_register = 0;
    }

    pub fn step(&mut self) -> Result<Event, InvalidLocation> {
        let Location::Instruction(instruction) = self.memory[self.program_counter as usize] else {
            return Err(InvalidLocation);
        };

        self.current_instruction_register = instruction.opcode;
        self.memory_address_register = instruction.operand;

        match instruction.opcode {
            0 => return Ok(Event::Halt),

            opcode @ (1 | 2 | 3 | 5) => {
                if let Location::Data(number) = self.memory[self.memory_address_register as usize] {
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
                    return Err(InvalidLocation);
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

                if instruction.opcode == 1 {
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
    use crate::backend::virtual_machine::Computer;

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
