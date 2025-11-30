use std::fmt::Display;

use crate::backend::compiler::generator::{InstructionLocation, Location};

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
pub struct InvalidInstruction;

impl Display for InvalidInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}

#[derive(Debug)]
pub enum RuntimeMessage {
    Continue,
    Halt,
    Input,
    Output(Box<str>),
}

impl Computer {
    pub fn reset(&mut self) {
        self.program_counter = 0;
        self.accumulator = 0;
        self.current_instruction_register = 0;
        self.memory_address_register = 0;
    }

    pub fn step(&mut self) -> Result<RuntimeMessage, InvalidInstruction> {
        let Location::Instruction(instruction) = self.memory[self.program_counter as usize] else {
            return Err(InvalidInstruction);
        };

        self.current_instruction_register = instruction.opcode;
        self.memory_address_register = instruction.operand;

        match instruction.opcode {
            0 => return Ok(RuntimeMessage::Halt),

            opcode @ (1 | 2 | 3 | 5) => {
                if let Location::Data(number) = self.memory[instruction.operand as usize] {
                    match opcode {
                        1 | 2 => {
                            self.accumulator += if opcode == 1 { number } else { -number };
                        }
                        3 => {
                            self.memory[instruction.operand as usize] = Location::Data(number);
                        }

                        5 => self.accumulator = number,

                        _ => unreachable!(),
                    }
                } else {
                    return Err(InvalidInstruction);
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
                    return Ok(RuntimeMessage::Continue);
                }
            }

            9 => {
                self.program_counter += 1;

                if instruction.opcode == 1 {
                    return Ok(RuntimeMessage::Input);
                } else {
                    return Ok(RuntimeMessage::Output(
                        format!("{}", self.accumulator).into(),
                    ));
                }
            }

            _ => unreachable!(),
        }

        self.program_counter += 1;
        Ok(RuntimeMessage::Continue)
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
