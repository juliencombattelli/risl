use crate::parser::context::ParseContext;
use crate::parser::diagnostic::DiagContext;
use crate::parser::emitter::new_emitter_none;
use crate::parser::lexer::token::TokenStr;
use crate::parser::lexer::FloatLiteral;
use crate::parser::lexer::IntegerBase;
use crate::parser::lexer::IntegerLiteral;

use super::Lexer;
use super::Span;
use super::Token;

fn stubbed_parse_context() -> ParseContext {
    ParseContext::new(DiagContext::new(new_emitter_none()))
}

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
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, "Hello other");
    let first_char = lexer.cursor.next().unwrap();
    let result = lexer.tokenize_identifier(first_char);
    assert_eq!(result, Token::Identifier(Span::new(0, 5)));
}

#[test]
fn tokenize_number_decimal() {
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, "123456 other");
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
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, "123456suffix other");
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
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, "0b123456 other");
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
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, "0b123456suffix other");
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
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, "0o123456 other");
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
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, "0o123456suffix other");
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
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, "0x123456 other");
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
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, "0x123456suffix other");
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
    let context = stubbed_parse_context();
    let source = "0x123456.0E-3suffix other";
    let mut lexer = Lexer::new(&context, source);
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
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, source);
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
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, source);
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
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, source);
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

#[test]
fn tokenize_line_comment() {
    let source = r"
    let answer =   42; // this is a line comment
    let other_anwer = 43; // an other comment
    ";
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, source);
    let mut tokens = vec![];
    while let Some(c) = lexer.cursor.next() {
        match lexer.parse_token(c) {
            Token::Whitespace => continue,
            token => tokens.push(token),
        }
    }
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
fn tokenize_block_comment_inline() {
    let source = "let answer = /* the answer */ 42;";
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, source);
    let mut tokens = vec![];
    while let Some(c) = lexer.cursor.next() {
        match lexer.parse_token(c) {
            Token::Whitespace => continue,
            token => tokens.push(token),
        }
    }
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
fn tokenize_block_comment_inline_nested() {
    let source = "let answer = /* /* the /**/ /* */ answer */*/ 42;";
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, source);
    let mut tokens = vec![];
    while let Some(c) = lexer.cursor.next() {
        match lexer.parse_token(c) {
            Token::Whitespace => continue,
            token => tokens.push(token),
        }
    }
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
fn tokenize_block_comment_multiline() {
    let source = r"
    /*
     * The answer
     */
    let answer = 42;";
    let context = stubbed_parse_context();
    let mut lexer = Lexer::new(&context, source);
    let mut tokens = vec![];
    while let Some(c) = lexer.cursor.next() {
        match lexer.parse_token(c) {
            Token::Whitespace => continue,
            token => tokens.push(token),
        }
    }
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
