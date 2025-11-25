//TODO: move into src/backend/compiler/test

#[cfg(test)]
mod compiler {
    use std::collections::HashMap;

    use crate::backend::compiler::{
        generator::{Location, Operand},
        lexer::{InvalidCharacter, Lexer, Token},
        parser::{InvalidToken, Parser, Program},
    };

    #[test]
    // 1.1.1
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
                Token::Identifier("ABYZ".into()),
                Token::Identifier("abyz".into()),
                Token::Newline
            ])
        );
    }

    #[test]
    // 1.1.2
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
    //1.2.1
    fn parser_all_instructions() {
        let source = r#"HLT
        COB
        ADD 1289
        SUB ABYZ
        STA ABYZ
        STO 1289
        yzab LDA ABYZ
        BRA yzab
        BRZ yzab
        BRP yzab
        INP
        OUT
        ABYZ DAT 1289
        YZAB DAT
        "#;

        assert_eq!(
            Parser::new(Lexer::new(source).lex().unwrap()).parse(),
            Ok(Program {
                labels: HashMap::new(),
                instructions: Vec::new(),
            })
        );
    }

    #[test]
    //1.2.2
    fn parser_too_many_tokens() {
        let source = "HLT 1289";

        assert_eq!(
            Parser::new(Lexer::new(source).lex().unwrap()).parse(),
            Err(InvalidToken {
                expected: vec![Token::Newline],
                received: Some(Token::Number(1289).into())
            })
        );
    }

    #[test]
    //1.2.3
    fn parser_not_enough_tokens() {
        let source = "ADD";

        assert_eq!(
            Parser::new(Lexer::new(source).lex().unwrap()).parse(),
            Err(InvalidToken {
                expected: vec![Token::Identifier("".into()), Token::Number(0)],
                received: None
            })
        );
    }

    // TODO: formatting test, invalid identifier test

    #[test]
    //1.3.1
    fn generator_all_instructions() {
        let source = r#"loop ADD one
        BRA loop
        one DAT 1"#;

        assert_eq!(
            <[Location; 100]>::try_from(
                Parser::new(Lexer::new(source).lex().unwrap())
                    .parse()
                    .unwrap()
            )
            .unwrap(),
            [Location::new(0, Operand::Number(0)); 100]
        );
    }
}
