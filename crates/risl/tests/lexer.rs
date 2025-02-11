use risl::parser::context::ParseContext;
use risl::parser::diagnostic::DiagContext;
use risl::parser::emitter::new_emitter_none;
use risl::parser::lexer::Span;
use risl::parser::lexer::{lex, IntegerBase, IntegerLiteral, Token};

#[allow(unused)]
use risl::parser::lexer::TokenStr;

fn stubbed_parse_context() -> ParseContext {
    ParseContext::new(DiagContext::new(new_emitter_none()))
}

#[test]
fn lex_empty() {
    let source = "";
    let context = stubbed_parse_context();
    let tokens = lex(&context, source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![]);
}

#[test]
fn lex_simple_assignment() {
    let source = "let answer =   42;";
    let context = stubbed_parse_context();
    let tokens = lex(&context, source).collect::<Vec<_>>();
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
    let context = stubbed_parse_context();
    let tokens = lex(&context, source).collect::<Vec<_>>();
    assert_eq!(tokens, vec![Token::Err(Span::new(0, 5))]);
}

#[test]
fn lex_ident_then_invalid_then_ident() {
    let source = "hello@@@@@world";
    let context = stubbed_parse_context();
    let tokens = lex(&context, source).collect::<Vec<_>>();
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
    let context = stubbed_parse_context();
    let tokens = lex(&context, source).collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            Token::Err(Span::new(0, 5)),
            Token::Identifier(Span::new(5, 10)),
            Token::Err(Span::new(10, 15)),
        ]
    );
}
