use std::error::Error;
use std::fmt::Display;

use crate::backend::compiler::generator::Location;
use crate::shared::runtime::Event;

// Represents a little man computer
#[derive(PartialEq, Clone, Debug)]
pub struct Computer {
    pub program_counter: u8,
    pub accumulator: i16,
    pub current_instruction_register: u8,
    pub memory_address_register: u8,
    pub memory_data_register: i16,
    // 100 instruction/data memory locations
    pub memory: [Location; 100],
}

// Create a default computer (all values set to zero)
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

// Possible errors for the virtual machine
#[derive(Debug)]
pub enum InvalidLocation {
    // Error when running into data memory locations (i.e. forgetting to halt before a data instruction)
    ExpectedInstruction,
    // Error when running into an operand that is of the Location::Data(T) type and not an instruction (i.e. ADD 20, where the memory address 20 is an instruction)
    ExpectedData,
}

impl Error for InvalidLocation {}

impl Display for InvalidLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            InvalidLocation::ExpectedInstruction => {
                "Ran into data memory whilst running code, did you forget to halt?"
            }
            InvalidLocation::ExpectedData => {
                "Expected an operand pointing to a data location, got an instruction location.\
                \nDid you use a number instead of an identifier or change the line count of your program?"
            }
        };

        write!(f, "Encountered an error at runtime...\n{text}")
    }
}

impl Computer {
    // Set all values back to zero
    pub fn reset(&mut self) {
        self.program_counter = 0;
        self.accumulator = 0;
        self.current_instruction_register = 0;
        self.memory_address_register = 0;
        self.memory_data_register = 0;
    }

    // Compute one instruction and prepare the program counter for the nmext instruction
    pub fn step(&mut self) -> Result<Event, InvalidLocation> {
        let Location::Instruction(instruction) = self.memory[self.program_counter as usize] else {
            if self.memory[self.program_counter as usize] == Location::Data(0) {
                // Send halt event for empty data memory locations
                return Ok(Event::Halt);
            } else {
                // Return an error if the memory location is data and isn't 0 (i.e. running into data memory)
                return Err(InvalidLocation::ExpectedInstruction);
            }
        };

        self.current_instruction_register = instruction.opcode;
        self.memory_address_register = instruction.operand;

        match instruction.opcode {
            // HLT/COB
            0 => return Ok(Event::Halt),

            // ADD, SUB, STA/STO, and LDA
            opcode @ (1 | 2 | 3 | 5) => {
                if let Location::Data(number) = self.memory[self.memory_address_register as usize] {
                    // Set MDR to value at MAR
                    self.memory_data_register = self.memory[self.memory_address_register as usize]
                        .to_string()
                        .parse()
                        .unwrap_or(0);

                    match opcode {
                        // ADD or SUB, if ADD then add the number as normal, if SUB then add the negated number (equivalent to subtraction)
                        1 | 2 => {
                            self.accumulator += if opcode == 1 { number } else { -number };
                        }

                        // STA, store current value of accumulator
                        3 => {
                            self.memory[self.memory_address_register as usize] =
                                Location::Data(self.accumulator);
                        }

                        // LDA, load accumulator with value in data location
                        5 => self.accumulator = number,

                        // Unreachable due to outer match statement
                        _ => unreachable!(),
                    }
                } else {
                    // Cannot load address that doesn't point to a data location
                    return Err(InvalidLocation::ExpectedData);
                }
            }

            // BRA, BRZ, and BRP
            6 | 7 | 8 => {
                let condition = match instruction.opcode {
                    // BRA, will always succeed
                    6 => true,
                    // BRZ, will only succeed if the accumulator is 0
                    7 => self.accumulator == 0,
                    // BRP, will only succeed if the accumulator is 0 or greater
                    8 => self.accumulator >= 0,
                    // Unreachable due to outer match statement
                    _ => unreachable!(),
                };
                if condition {
                    self.program_counter = instruction.operand;
                    return Ok(Event::Continue);
                }
            }

            // INP or OUT
            9 => {
                self.program_counter += 1;

                if instruction.operand == 1 {
                    // Send input event
                    return Ok(Event::Input);
                } else {
                    // Send output event with accumulator as a string
                    return Ok(Event::Output(format!("{}", self.accumulator).into()));
                }
            }

            _ => unreachable!(),
        }

        // Bump PC for next step call
        self.program_counter += 1;
        Ok(Event::Continue)
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::compiler;
    use crate::shared::vm::Computer;

    // 2.1
    #[test]
    fn virtual_machine() {
        let source = r#"test OUT
        BRA test"#;

        let mut computer = Computer::default();
        computer.memory = compiler::compile(source).unwrap();

        computer.step().unwrap();
        computer.step().unwrap();
        computer.step().unwrap();
        computer.step().unwrap();

        assert_eq!(computer, {
            let mut expected = Computer::default();
            expected.memory = computer.memory.clone();

            expected.current_instruction_register = 6;

            expected
        });
    }
}
