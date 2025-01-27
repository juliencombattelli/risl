use crate::parser::lexer::IntegerBase;
use crate::parser::lexer::IntegerLiteral;

use super::lex;
use super::Lexer;
use super::Span;
use super::Token;

#[test]
fn tokenize_identifier() {
    let mut lexer = Lexer::new("Hello other");
    let result = lexer.tokenize_identifier();
    assert_eq!(result, Token::Identifier(Span::new(0, 5)));
}

#[test]
fn tokenize_number_decimal() {
    let mut lexer = Lexer::new("23456");
    let result = lexer.tokenize_number('1');
    assert_eq!(
        result,
        Token::Integer(IntegerLiteral {
            base: IntegerBase::Dec,
            value: Span::new(0, 5),
            suffix: Span::new(5, 5),
        })
    );
}

#[test]
fn lex_empty() {
    let source = "";
    let tokens = lex(source).collect::<Vec<_>>();
    assert!(tokens.is_empty());
}

#[test]
fn lex_simple_assignment() {
    let source = "let answer = 42;";
    let tokens = lex(source).collect::<Vec<_>>();
    assert!(!tokens.is_empty()); // TODO expected tokens
}
