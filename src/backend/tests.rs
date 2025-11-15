#[cfg(test)]
mod compiler {
    use std::collections::HashMap;

    use crate::backend::compiler::{InvalidCharacter, Lexer, Parser, Program, Token};

    #[test]
    // 1.1
    fn lexer_all_tokens() {
        let source = "HLT COB ADD SUB STA STO LDA BRA BRZ BRP INP OUT DAT 1289 ABYZ abyz\n";

        assert_eq!(
            Lexer::new(source).lex(),
            Ok(vec![
                Token::Halt,
                Token::Halt,
                Token::Add,
                Token::Sub,
                Token::Store,
                Token::Store,
                Token::Load,
                Token::Branch,
                Token::BranchZero,
                Token::BranchPositive,
                Token::Input,
                Token::Output,
                Token::Data,
                Token::Number(1289),
                Token::Identifier("ABYZ".to_string()),
                Token::Identifier("abyz".to_string()),
                Token::Newline
            ])
        );
    }

    #[test]
    fn lexer_invalid_chararcter() {
        let source = "LDA 10\n?";
        let source2 = "LDA 10\nSTA ?";

        assert_eq!(
            Lexer::new(source).lex(),
            Err(InvalidCharacter {
                character: '?',
                line_column: (2, 1)
            })
        );

        assert_eq!(
            Lexer::new(source2).lex(),
            Err(InvalidCharacter {
                character: '?',
                line_column: (2, 5)
            })
        );
    }

    #[test]
    fn parser_all_instructions() {
        let source = r#"loop ADD 10
        "#;

        assert_eq!(
            Parser::new(Lexer::new(source).lex().unwrap()).parse(),
            Ok(Program {
                labels: HashMap::new(),
                memory: [0; 100],
                instructions: Vec::new()
            })
        );
    }
}
