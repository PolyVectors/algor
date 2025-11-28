use crate::backend::compiler::generator::{InstructionLocation, Location};

#[derive(PartialEq, Debug)]
pub struct Computer {
    program_counter: u8,
    accumulator: i16,
    current_instruction_register: u8,
    memory_address_register: u8,
    pub memory: [Location; 100],
}

impl Computer {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            accumulator: 0,
            current_instruction_register: 0,
            memory_address_register: 0,
            memory: [Location::Data(0); 100],
        }
    }

    pub fn step(&mut self) {
        self.program_counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::compiler::generator::Location;
    use crate::backend::compiler::lexer::Lexer;
    use crate::backend::compiler::parser::Parser;
    use crate::backend::virtual_machine::Computer;

    #[test]
    //2.1
    fn virtual_machine() {
        let source = r#"loop ADD num
        BRA loop
        num DAT 999"#;

        let memory = <[Location; 100]>::try_from(
            Parser::new(Lexer::new(source).lex().unwrap())
                .parse()
                .unwrap(),
        )
        .unwrap();

        let mut computer = Computer::new();
        computer.memory = memory;

        assert_eq!(computer, Computer::new());
    }
}
