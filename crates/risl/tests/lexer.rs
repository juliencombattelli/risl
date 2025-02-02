use risl::parser::lexer::Span;
use risl::parser::lexer::{lex, IntegerBase, IntegerLiteral, Token};

#[allow(unused)]
use risl::parser::lexer::TokenStr;

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
fn lex_line_comment() {
    let source = r"
    let answer =   42; // this is a line comment
    let other_anwer = 43; // an other comment
    ";
    let tokens = lex(source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            // first line
            Token::Identifier(Span::new(5, 8)),
            Token::Identifier(Span::new(9, 15)),
            Token::Equal,
            Token::Integer(IntegerLiteral {
                base: IntegerBase::Dec,
                value: Span::new(20, 22),
                suffix: Span::new(22, 22),
            }),
            Token::Semicolon,
            Token::LineComment(Span::new(26, 49)),
            // second line
            Token::Identifier(Span::new(54, 57)),
            Token::Identifier(Span::new(58, 69)),
            Token::Equal,
            Token::Integer(IntegerLiteral {
                base: IntegerBase::Dec,
                value: Span::new(72, 74),
                suffix: Span::new(74, 74),
            }),
            Token::Semicolon,
            Token::LineComment(Span::new(78, 95)),
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

#[test]
fn lex_invalid_then_ident_then_invalid() {
    let source = "@@@@@hello@@@@@";
    let tokens = lex(source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Err(Span::new(0, 5)),
            Token::Identifier(Span::new(5, 10)),
            Token::Err(Span::new(10, 15)),
        ]
    );
}
