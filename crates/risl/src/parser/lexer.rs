use super::span::Span;
use super::span::SpanMerger;
use super::span::SpanSubstr;

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
    Err(Span),
}

struct TokenStr<'src> {
    token: Token,
    source: &'src str,
}

impl<'src> TokenStr<'src> {
    fn new(token: Token, source: &'src str) -> Self {
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
            Token::Err(span) => self.source.substr(span),
        };
        write!(f, "{token}")
    }
}

/// The error type from the lexer raised for diagnostic purposes.
#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    UnknownToken,
    NoDigitLiteral,
    InvalidDigitLiteral,
    EmptyExponentFloat,
    FloatLiteralUnsupportedBase,
}

/// Iterates over the lexed tokens in the given source file.
pub fn lex(source: &str) -> impl Iterator<Item = Token> + use<'_> {
    let mut lexer = Lexer::new(&source);
    std::iter::from_fn(move || lexer.next_token())
}

/// A basic cursor providing iteration capabilities over an unicode character sequence.
#[derive(Debug)]
struct Cursor<'src> {
    chars: std::str::Chars<'src>,
    consumed: usize,
}

impl<'src> Cursor<'src> {
    /// Creates a new cursor for the given source string.
    fn new(source: &'src str) -> Self {
        Self {
            chars: source.chars(),
            consumed: 0,
        }
    }

    /// Returns the remaining string.
    fn as_str(&self) -> &'src str {
        self.chars.as_str()
    }

    /// Peeks the next next character, if any.
    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Peeks the n-th next character, if any.
    fn peek_nth(&self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }

    /// Moves to the next character.
    /// Does not move the cursor if the next character does not exist.
    fn next(&mut self) -> Option<char> {
        let next = self.chars.next();
        if let Some(_) = next {
            self.consumed += 1;
        }
        next
    }

    /// Moves to the n-th next character.
    /// Does not move the cursor if the n-th character does not exist.
    fn next_nth(&mut self, n: usize) -> Option<char> {
        let nth = self.chars.nth(n);
        if let Some(_) = nth {
            self.consumed += n;
        }
        nth
    }

    /// Moves to the next character while the predicate returns true for that character.
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
}

#[doc(hidden)]
fn is_identifier_start(c: char) -> bool {
    unicode_ident::is_xid_start(c) || c == '_'
}

#[doc(hidden)]
fn is_identifier_continuation(c: char) -> bool {
    unicode_ident::is_xid_continue(c) || c == '_'
}

#[doc(hidden)]
fn is_digit_start(c: char) -> bool {
    // Start of number is always a character between 0 and 9
    return c.is_ascii_digit();
}

// Only continuation variants exist to check digits for a specific base as the
// first digit will always be between 0 and 9

#[doc(hidden)]
fn is_digit_base10_continuation(c: char) -> bool {
    return c.is_ascii_digit() || c == '_';
}

#[doc(hidden)]
fn is_digit_base2_continuation(c: char) -> bool {
    return ('0'..='1').contains(&c) || c == '_';
}

#[doc(hidden)]
fn is_digit_base8_continuation(c: char) -> bool {
    return ('0'..='7').contains(&c) || c == '_';
}

#[doc(hidden)]
fn is_digit_base16_continuation(c: char) -> bool {
    return c.is_ascii_hexdigit() || c == '_';
}

/// The lexer for the Risl language.
struct Lexer<'src> {
    source: &'src str,
    cursor: Cursor<'src>,
    pending_token: Option<Token>,
}

impl<'src> Lexer<'src> {
    /// Creates a lexer for the given source string.
    fn new(source: &'src str) -> Self {
        Self {
            source,
            cursor: Cursor::new(source),
            pending_token: None,
        }
    }

    /// Advances the cursor while the predicate is true and returns the substring that was consumed.
    fn take_while(&mut self, predicate: impl FnMut(char) -> bool) -> Span {
        let start = self.cursor.consumed;
        self.cursor.advance_while(predicate);
        let end = self.cursor.consumed;
        Span::new(start, end)
    }

    /// Extracts the current identifier or keyword.
    fn tokenize_identifier(&mut self, first_char: char) -> Token {
        debug_assert!(is_identifier_start(first_char));
        let mut identifier = self.take_while(is_identifier_continuation);
        // Add the first char of the identifier already consumed
        // Unwraping here is safe as char::len_utf8() is always between 1 and 4 inclusive
        identifier.start -= u32::try_from(first_char.len_utf8()).unwrap();
        Token::Identifier(identifier)
    }

    /// Extracts the base prefix for the current number and return it.
    fn extract_number_base(&mut self, first_digit: char) -> Option<IntegerBase> {
        if first_digit == '0' {
            match self.cursor.peek() {
                Some('b') => {
                    self.cursor.next();
                    Some(IntegerBase::Bin)
                }
                Some('o') => {
                    self.cursor.next();
                    Some(IntegerBase::Oct)
                }
                Some('x') => {
                    self.cursor.next();
                    Some(IntegerBase::Hex)
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn extract_float_exponent(&mut self) -> Span {
        let mut sign = false;
        if let Some(c) = self.cursor.peek() {
            if c == '-' || c == '+' {
                self.cursor.next();
                sign = true;
            }
        }
        let mut exponent = self.take_while(is_digit_base10_continuation);
        if sign {
            exponent.start -= 1;
        }
        exponent
    }

    /// Extracts a number literal, being an integer or a floating-point number.
    fn tokenize_number(&mut self, first_digit: char) -> Token {
        debug_assert!(is_digit_start(first_digit));
        let (base, value) = match self.extract_number_base(first_digit) {
            None => {
                let mut value = self.take_while(is_digit_base10_continuation);
                // Include first_digit if this is not the first char of the base (the 0 in 0b, 0o or 0x)
                // Its size is always one byte as 0..=9 are ascii characters
                value.start -= 1;
                (IntegerBase::Dec, value)
            }
            Some(IntegerBase::Hex) => (
                IntegerBase::Hex,
                self.take_while(is_digit_base16_continuation),
            ),
            Some(base) => (base, self.take_while(is_digit_base10_continuation)),
        };
        if let Some('.') = self.cursor.peek() {
            if let Some(c) = self.cursor.peek_nth(1) {
                if c != '.' && !is_identifier_start(c) {
                    self.cursor.next();
                    let integer_part = value;
                    let fractional_part = self.take_while(is_digit_base10_continuation);
                    let exponent = if let Some('e' | 'E') = self.cursor.peek() {
                        self.cursor.next();
                        self.extract_float_exponent()
                    } else {
                        Span::new_empty(self.cursor.consumed)
                    };
                    let suffix = self.take_while(is_identifier_continuation);
                    return Token::Float(FloatLiteral {
                        base,
                        integer_part,
                        fractional_part,
                        exponent,
                        suffix,
                    });
                }
            }
        } else if base != IntegerBase::Hex {
            if let Some('e' | 'E') = self.cursor.peek() {
                self.cursor.next();
                let integer_part = value;
                let fractional_part = Span::new_empty(self.cursor.consumed);
                let exponent = self.extract_float_exponent();
                let suffix = self.take_while(is_identifier_continuation);
                return Token::Float(FloatLiteral {
                    base,
                    integer_part,
                    fractional_part,
                    exponent,
                    suffix,
                });
            }
        }
        let suffix = self.take_while(is_identifier_continuation);
        Token::Integer(IntegerLiteral {
            base,
            value,
            suffix,
        })
    }

    /// Advances the cursor while whitespace are encountered.
    fn skip_whitespaces(&mut self, first_ws: char) {
        debug_assert!(first_ws.is_whitespace());
        let mut c = first_ws;
        loop {
            match c {
                '\n' => {
                    // FIXME add file position handling
                    // line += 1;
                    // column = 1;
                }
                _ => {}
            }
            match self.cursor.peek() {
                Some(next) if next.is_whitespace() => {
                    c = next;
                    self.cursor.next();
                }
                _ => break,
            }
        }
    }

    /// Takes the current character and advance the cursor until a token is found.
    fn parse_token(&mut self, c: char) -> Token {
        match c {
            // Whitespaces
            c if c.is_whitespace() => {
                self.skip_whitespaces(c);
                Token::Whitespace
            }
            // Single-character tokens
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '-' => Token::Minus,
            '+' => Token::Plus,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '/' => Token::Slash,
            '\\' => Token::Backslash,
            '*' => Token::Star,
            '&' => Token::Ampersand,
            '|' => Token::Pipe,
            // One or two characters tokens
            '!' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    Token::NotEqual
                }
                _ => Token::Not,
            },
            '=' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    Token::EqualEqual
                }
                _ => Token::Equal,
            },
            '>' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    Token::GreaterEqual
                }
                _ => Token::Greater,
            },
            '<' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    Token::LessEqual
                }
                _ => Token::Less,
            },
            // Literals
            c if is_digit_start(c) => self.tokenize_number(c),
            c if is_identifier_start(c) => self.tokenize_identifier(c),
            // Unknown characters
            _ => Token::Err(Span::new(self.cursor.consumed - 1, self.cursor.consumed)),
        }
    }

    /// Returns the next token in the source file.
    /// Returns None if the source file end is reached, iteration is not resumed.
    fn next_token(&mut self) -> Option<Token> {
        if let Some(_) = self.pending_token {
            return self.pending_token.take();
        }
        let mut invalid_token_span: Option<Span> = None;
        loop {
            match self.cursor.next() {
                Some(c) => {
                    let token = match self.parse_token(c) {
                        Token::Whitespace => continue, // Skip whitespaces
                        Token::Err(span) => {
                            // Group consecutive unknown characters
                            invalid_token_span.merge(span);
                            continue;
                        }
                        token => {
                            if let Some(span) = invalid_token_span {
                                // Invalid token extracted at previous iteration
                                // Return it and save current valid token for the next iteration
                                self.pending_token = Some(token);
                                Token::Err(span)
                            } else {
                                token
                            }
                        }
                    };
                    return Some(token);
                }
                None => {
                    // If EOF is reached and an invalid token is pending then return it now
                    // If no invalid token is pending then None is returned immediately
                    return invalid_token_span.and_then(|span| Some(Token::Err(span)));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
