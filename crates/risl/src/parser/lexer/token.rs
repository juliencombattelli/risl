use super::span::{Span, SpanSubstr};

/// The integer literal numeric bases supported by the Risl language.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum IntegerBase {
    Bin,
    Oct,
    Dec,
    Hex,
}

/// The data for an lexed integer literal with its value and suffix if any, and its base.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct IntegerLiteral {
    pub base: IntegerBase,
    pub value: Span,
    pub suffix: Span,
}

/// The data for an lexed float literal.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct FloatLiteral {
    pub base: IntegerBase,
    pub integer_part: Span,
    pub fractional_part: Span,
    pub exponent: Span,
    pub suffix: Span,
}

/// The tokens supported by the Risl language.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Token {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Minus,
    Plus,
    Colon,
    Semicolon,
    Slash,
    Backslash,
    Star,
    Ampersand,
    Pipe,
    // One or two character tokens
    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Dot,
    DotDot,
    DotDotEqual,
    // Literals
    Identifier(Span),
    String(Span),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    // Keywords
    And,
    Break,
    Const,
    Continue,
    Else,
    Enum,
    False,
    Fn,
    For,
    If,
    In,
    Let,
    Match,
    Mut,
    Nil,
    Or,
    Pub,
    Return,
    SelfValue,
    SelfType,
    Struct,
    Super,
    This,
    True,
    While,
    // Others
    Whitespace,
    LineComment(Span),
    BlockComment(Span),
    Err(Span),
}

impl Token {
    pub fn is_skippable(&self) -> bool {
        matches!(self, Token::Whitespace)
    }
}

pub struct TokenStr<'src> {
    token: Token,
    source: &'src str,
}

impl<'src> TokenStr<'src> {
    pub fn new(token: Token, source: &'src str) -> Self {
        Self { token, source }
    }
}

impl<'src> std::fmt::Display for TokenStr<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let token = match self.token {
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LeftBrace => "{",
            Token::RightBrace => "}",
            Token::LeftBracket => "[",
            Token::RightBracket => "]",
            Token::Comma => ",",
            Token::Minus => "-",
            Token::Plus => "+",
            Token::Colon => ":",
            Token::Semicolon => ";",
            Token::Slash => "/",
            Token::Backslash => "\\",
            Token::Star => "*",
            Token::Ampersand => "&",
            Token::Pipe => "|",
            Token::Not => "!",
            Token::NotEqual => "!=",
            Token::Equal => "=",
            Token::EqualEqual => "==",
            Token::Greater => ">",
            Token::GreaterEqual => ">=",
            Token::Less => "<",
            Token::LessEqual => "<=",
            Token::Dot => ".",
            Token::DotDot => "..",
            Token::DotDotEqual => "..=",
            Token::Identifier(span) => self.source.substr(span),
            Token::String(span) => self.source.substr(span),
            Token::Integer(integer_literal) => {
                return write!(
                    f,
                    "{:?}, '{}', '{}'",
                    integer_literal.base,
                    self.source.substr(integer_literal.value),
                    self.source.substr(integer_literal.suffix),
                );
            }
            Token::Float(float_literal) => {
                return write!(
                    f,
                    "{{{:?}, '{}', '{}', '{}', '{}'}}",
                    float_literal.base,
                    self.source.substr(float_literal.integer_part),
                    self.source.substr(float_literal.fractional_part),
                    self.source.substr(float_literal.exponent),
                    self.source.substr(float_literal.suffix),
                );
            }
            Token::And => "and",
            Token::Break => "break",
            Token::Const => "const",
            Token::Continue => "continue",
            Token::Else => "else",
            Token::Enum => "enum",
            Token::False => "false",
            Token::Fn => "fn",
            Token::For => "for",
            Token::If => "if",
            Token::In => "in",
            Token::Let => "let",
            Token::Match => "match",
            Token::Mut => "mut",
            Token::Nil => "Nil",
            Token::Or => "or",
            Token::Pub => "pub",
            Token::Return => "return",
            Token::SelfValue => "self",
            Token::SelfType => "Self",
            Token::Struct => "struct",
            Token::Super => "super",
            Token::This => "this",
            Token::True => "true",
            Token::While => "while",
            Token::Whitespace => " ",
            Token::LineComment(span) => self.source.substr(span),
            Token::BlockComment(span) => self.source.substr(span),
            Token::Err(span) => self.source.substr(span),
        };
        write!(f, "{token}")
    }
}
