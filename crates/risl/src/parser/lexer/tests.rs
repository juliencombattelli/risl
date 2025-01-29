use crate::parser::lexer::IntegerBase;
use crate::parser::lexer::IntegerLiteral;

use super::Lexer;
use super::Span;
use super::Token;

#[test]
fn tokenize_identifier() {
    let mut lexer = Lexer::new("Hello other");
    let first_char = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_identifier(first_char);
    assert_eq!(result, Token::Identifier(Span::new(0, 5)));
}

#[test]
fn tokenize_number_decimal() {
    let mut lexer = Lexer::new("123456 other");
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Integer(IntegerLiteral {
            base: IntegerBase::Dec,
            value: Span::new(0, 6),
            suffix: Span::new(6, 6),
        })
    );
}

#[test]
fn tokenize_number_decimal_with_suffix() {
    let mut lexer = Lexer::new("123456suffix other");
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Integer(IntegerLiteral {
            base: IntegerBase::Dec,
            value: Span::new(0, 6),
            suffix: Span::new(6, 12),
        })
    );
}

#[test]
fn tokenize_number_binary() {
    let mut lexer = Lexer::new("0b123456 other");
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Integer(IntegerLiteral {
            base: IntegerBase::Bin,
            value: Span::new(2, 8),
            suffix: Span::new(8, 8),
        })
    );
}

#[test]
fn tokenize_number_binary_with_suffix() {
    let mut lexer = Lexer::new("0b123456suffix other");
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Integer(IntegerLiteral {
            base: IntegerBase::Bin,
            value: Span::new(2, 8),
            suffix: Span::new(8, 14),
        })
    );
}

#[test]
fn tokenize_number_octal() {
    let mut lexer = Lexer::new("0o123456 other");
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Integer(IntegerLiteral {
            base: IntegerBase::Oct,
            value: Span::new(2, 8),
            suffix: Span::new(8, 8),
        })
    );
}

#[test]
fn tokenize_number_octal_with_suffix() {
    let mut lexer = Lexer::new("0o123456suffix other");
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Integer(IntegerLiteral {
            base: IntegerBase::Oct,
            value: Span::new(2, 8),
            suffix: Span::new(8, 14),
        })
    );
}

#[test]
fn tokenize_number_hexadecimal() {
    let mut lexer = Lexer::new("0x123456 other");
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Integer(IntegerLiteral {
            base: IntegerBase::Hex,
            value: Span::new(2, 8),
            suffix: Span::new(8, 8),
        })
    );
}

#[test]
fn tokenize_number_hexadecimal_with_suffix() {
    let mut lexer = Lexer::new("0x123456suffix other");
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Integer(IntegerLiteral {
            base: IntegerBase::Hex,
            value: Span::new(2, 8),
            suffix: Span::new(8, 14),
        })
    );
}
