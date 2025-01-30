use risl::parser::lexer::{lex, IntegerBase, IntegerLiteral, Span, Token};

#[test]
fn lex_empty() {
    let source = "";
    let tokens = lex(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![]);
}

#[test]
fn lex_simple_assignment() {
    let source = "let answer =   42;";
    let tokens = lex(source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Span::new(0, 3)),
            Token::Identifier(Span::new(4, 10)),
            Token::Equal,
            Token::Integer(IntegerLiteral {
                base: IntegerBase::Dec,
                value: Span::new(15, 17),
                suffix: Span::new(17, 17),
            }),
            Token::Semicolon,
        ]
    );
}

#[test]
fn lex_invalid() {
    let source = "@@@@@";
    let tokens = lex(source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Token::Err(Span::new(0, 5))]);
}

#[test]
fn lex_ident_then_invalid_then_ident() {
    let source = "hello@@@@@world";
    let tokens = lex(source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Span::new(0, 5)),
            Token::Err(Span::new(5, 10)),
            Token::Identifier(Span::new(10, 15)),
        ]
    );
}
