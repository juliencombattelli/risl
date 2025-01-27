#[derive(PartialEq, Debug, Copy, Clone)]
pub enum IntegerBase {
    Bin,
    Oct,
    Dec,
    Hex,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct IntegerLiteral<'src> {
    base: IntegerBase,
    value: &'src str,
    suffix: &'src str,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Token<'source> {
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
    Identifier(&'source str),
    String(&'source str),
    Integer(IntegerLiteral<'source>),
    Float(&'source str),
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
    Eof,
    Err,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    NoDigitLiteral,
    InvalidDigitLiteral,
    EmptyExponentFloat,
    FloatLiteralUnsupportedBase,
}

pub fn lex(source: &str) -> impl Iterator<Item = Token> {
    let mut lexer = Lexer::new(source);
    std::iter::from_fn(move || lexer.next_token())
}

/// Provide basic iteration capabilities over an unicode character sequence
#[derive(Debug)]
struct Cursor<'src> {
    chars: std::str::Chars<'src>,
    consumed: usize,
}

impl<'src> Cursor<'src> {
    fn new(input: &'src str) -> Self {
        Self {
            chars: input.chars(),
            consumed: 0,
        }
    }

    fn as_str(&self) -> &'src str {
        self.chars.as_str()
    }

    /// Peek the next next character, if any
    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Peek the n-th next character, if any
    fn peek_nth(&self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }

    /// Move to the next character
    /// Does not move the cursor if the next character does not exist
    fn next(&mut self) -> Option<char> {
        let next = self.chars.next();
        if let Some(_) = next {
            self.consumed += 1;
        }
        next
    }

    /// Move to the n-th next character
    /// Does not move the cursor if the n-th character does not exist
    fn next_nth(&mut self, n: usize) -> Option<char> {
        let nth = self.chars.nth(n);
        if let Some(_) = nth {
            self.consumed += n;
        }
        nth
    }

    /// Move to the next character while the predicate returns true for that character
    fn advance_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        loop {
            match self.peek() {
                Some(c) => {
                    if predicate(c) {
                        self.next();
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
    }

    // fn is_eof(&self) -> bool {
    //     self.chars.as_str().is_empty()
    // }
}

fn is_identifier_start(c: char) -> bool {
    unicode_ident::is_xid_start(c) || c == '_'
}

fn is_identifier_continuation(c: char) -> bool {
    unicode_ident::is_xid_continue(c) || c == '_'
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

// Only continuation variant exists to check digits as the start is checked in
// the tokenizer big match statement

fn is_digit_base10_continuation(ch: char) -> bool {
    return ch.is_ascii_digit() || ch == '_';
}

fn is_digit_base2_continuation(ch: char) -> bool {
    return ('0'..='1').contains(&ch) || ch == '_';
}

fn is_digit_base8_continuation(ch: char) -> bool {
    return ('0'..='7').contains(&ch) || ch == '_';
}

fn is_digit_base16_continuation(ch: char) -> bool {
    return ch.is_ascii_hexdigit() || ch == '_';
}

struct Lexer<'src> {
    source: &'src str,
    cursor: Cursor<'src>,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source,
            cursor: Cursor::new(source),
        }
    }

    /// Advance the cursor while the preficate is true and return the substring that was consumed
    fn take_while(&mut self, predicate: impl FnMut(char) -> bool) -> &'src str {
        let start = self.cursor.consumed;
        self.cursor.advance_while(predicate);
        let end = self.cursor.consumed;
        &self.source[start..end]
    }

    fn tokenize_identifier(&'src mut self) -> Token {
        let identifier = self.take_while(is_identifier_continuation);
        Token::Identifier(&identifier)
    }

    fn extract_number_base(&mut self, first_digit: char) -> IntegerBase {
        if first_digit == '0' {
            match self.cursor.peek() {
                Some('b') => {
                    self.cursor.next();
                    IntegerBase::Bin
                }
                Some('o') => {
                    self.cursor.next();
                    IntegerBase::Oct
                }
                Some('x') => {
                    self.cursor.next();
                    IntegerBase::Hex
                }
                _ => IntegerBase::Dec,
            }
        } else {
            IntegerBase::Dec
        }
    }

    fn tokenize_number(&'src mut self, first_digit: char) -> Token {
        let base = self.extract_number_base(first_digit);
        let value = self.take_while(is_digit_base16_continuation);
        let suffix = self.take_while(is_identifier_continuation);

        Token::Integer(IntegerLiteral {
            base,
            value,
            suffix,
        })
    }

    fn next_token(&mut self) -> Option<Token<'src>> {
        None
    }
}

#[cfg(test)]
mod tests;
