use crate::backend::compiler::generator::{InstructionLocation, Location};

#[derive(PartialEq, Debug)]
pub struct Computer {
    program_counter: u8,
    accumulator: i16,
    current_instruction_register: u8,
    memory_address_register: u8,
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
pub struct InvalidInstruction;

pub enum Task {
    Continue,
    Halt,
    Input,
    Output(Box<str>),
}

impl Computer {
    pub fn step(&mut self) -> Result<Task, InvalidInstruction> {
        if let Location::Instruction(instruction) = self.memory[self.program_counter as usize] {
            self.current_instruction_register = instruction.opcode;
            self.memory_address_register = instruction.operand;

            // TODO: condense
            match instruction.opcode {
                0 => return Ok(Task::Halt),

                opcode @ (1 | 2) => {
                    if let Location::Data(number) = self.memory[instruction.operand as usize] {
                        self.accumulator += if opcode == 1 { number } else { -number };
                    } else {
                        return Err(InvalidInstruction);
                    }
                }

                3 => {
                    self.memory[instruction.operand as usize] = Location::Data(self.accumulator);
                }
                5 => {
                    if let Location::Data(number) = self.memory[instruction.operand as usize] {
                        self.accumulator = number
                    } else {
                        return Err(InvalidInstruction);
                    }
                }

                6 => {
                    self.program_counter = instruction.operand;
                    return Ok(Task::Continue);
                }
                7 => {
                    if self.accumulator == 0 {
                        self.program_counter = instruction.operand;
                        return Ok(Task::Continue);
                    }
                }
                8 => {
                    if self.accumulator >= 0 {
                        self.program_counter = instruction.operand;
                        return Ok(Task::Continue);
                    }
                }

                9 => {
                    if instruction.opcode == 1 {
                        self.program_counter += 1;
                        return Ok(Task::Input);
                    } else {
                        self.program_counter += 1;
                        return Ok(Task::Output(format!("{}", self.accumulator).into()));
                    }
                }

                _ => unreachable!(),
            }
        } else {
            return Err(InvalidInstruction);
        }

        self.program_counter += 1;
        Ok(Task::Continue)
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::compiler;
    use crate::backend::virtual_machine::Computer;

    #[test]
    //2.1
    fn virtual_machine() {
        let source = r#"test OUT
        BRA test"#;

        let mut computer = Computer::default();
        computer.memory = compiler::compile(source).unwrap();

        computer.step();
        computer.step();
        computer.step();

        assert_eq!(computer, Computer::default());
    }
}
