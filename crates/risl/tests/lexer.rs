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
fn lex_block_comment_inline() {
    let source = "let answer = /* the answer */ 42;";
    let tokens = lex(source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Span::new(0, 3)),
            Token::Identifier(Span::new(4, 10)),
            Token::Equal,
            Token::BlockComment(Span::new(15, 27)),
            Token::Integer(IntegerLiteral {
                base: IntegerBase::Dec,
                value: Span::new(30, 32),
                suffix: Span::new(32, 32),
            }),
            Token::Semicolon,
        ]
    );
}

#[test]
fn lex_block_comment_inline_nested() {
    let source = "let answer = /* /* the /**/ /* */ answer */*/ 42;";
    let tokens = lex(source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(Span::new(0, 3)),
            Token::Identifier(Span::new(4, 10)),
            Token::Equal,
            Token::BlockComment(Span::new(15, 43)),
            Token::Integer(IntegerLiteral {
                base: IntegerBase::Dec,
                value: Span::new(46, 48),
                suffix: Span::new(48, 48)
            }),
            Token::Semicolon
        ]
    );
}

#[test]
fn lex_block_comment_multiline() {
    let source = r"
    /*
     * The answer
     */
    let answer = 42;";
    let tokens = lex(source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::BlockComment(Span::new(7, 31)),
            Token::Identifier(Span::new(38, 41)),
            Token::Identifier(Span::new(42, 48)),
            Token::Equal,
            Token::Integer(IntegerLiteral {
                base: IntegerBase::Dec,
                value: Span::new(51, 53),
                suffix: Span::new(53, 53),
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
