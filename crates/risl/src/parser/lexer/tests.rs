use crate::parser::lexer::token::TokenStr;
use crate::parser::lexer::FloatLiteral;
use crate::parser::lexer::IntegerBase;
use crate::parser::lexer::IntegerLiteral;

use super::Lexer;
use super::Span;
use super::Token;

#[test]
fn token_str_one_char() {
    let source = "";
    let token = TokenStr::new(Token::Ampersand, source);
    assert_eq!(format!("{}", token), "&");
}

#[test]
fn token_str_two_char() {
    let source = "";
    let token = TokenStr::new(Token::NotEqual, source);
    assert_eq!(format!("{}", token), "!=");
}

#[test]
fn token_str_three_char() {
    let source = "";
    let token = TokenStr::new(Token::DotDotEqual, source);
    assert_eq!(format!("{}", token), "..=");
}

#[test]
fn token_str_identifier() {
    let source = "Hello, world!";
    let token = TokenStr::new(Token::Identifier(Span::new(7, 12)), source);
    assert_eq!(format!("{}", token), "world");
}

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

#[test]
fn tokenize_number_float_full() {
    let source = "0x123456.0E-3suffix other";
    let mut lexer = Lexer::new(source);
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Float(FloatLiteral {
            base: IntegerBase::Hex,
            integer_part: Span::new(2, 8),
            fractional_part: Span::new(9, 10),
            exponent: Span::new(11, 13),
            suffix: Span::new(13, 19),
        })
    );
}

#[test]
fn tokenize_number_float_integer_exponent() {
    let source = "123456E+2 other";
    let mut lexer = Lexer::new(source);
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Float(FloatLiteral {
            base: IntegerBase::Dec,
            integer_part: Span::new(0, 6),
            fractional_part: Span::new(6, 6),
            exponent: Span::new(7, 9),
            suffix: Span::new(9, 9),
        })
    );
}

#[test]
fn tokenize_number_float_integer_exponent_with_base() {
    let source = "0b123456E2 other";
    let mut lexer = Lexer::new(source);
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Float(FloatLiteral {
            base: IntegerBase::Bin,
            integer_part: Span::new(2, 8),
            fractional_part: Span::new(8, 8),
            exponent: Span::new(9, 10),
            suffix: Span::new(10, 10),
        })
    );
}

#[test]
fn tokenize_number_float_integer_exponent_with_hex_base() {
    let source = "0x123456E2 other";
    let mut lexer = Lexer::new(source);
    let first_digit = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_number(first_digit);
    assert_eq!(
        result,
        Token::Integer(IntegerLiteral {
            base: IntegerBase::Hex,
            value: Span::new(2, 10),
            suffix: Span::new(10, 10),
        })
    );
}
