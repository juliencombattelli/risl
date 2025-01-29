pub type ByteIndex = u32;

/// A span corresponding to a substring of the source file being parsed.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Span {
    /// The start of the span in bytes.
    pub start: ByteIndex,
    /// The end of the span in bytes.
    pub end: ByteIndex,
}

impl Span {
    pub fn new<S, E>(start: S, end: E) -> Self
    where
        S: TryInto<ByteIndex>,
        S::Error: std::fmt::Debug,
        E: TryInto<ByteIndex>,
        E::Error: std::fmt::Debug,
    {
        Self {
            start: start.try_into().expect("start out of bounds"),
            end: end.try_into().expect("end out of bounds"),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum IntegerBase {
    Bin,
    Oct,
    Dec,
    Hex,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct IntegerLiteral {
    pub base: IntegerBase,
    pub value: Span,
    pub suffix: Span,
}

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
    Float(Span),
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
    Err(Span),
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    NoDigitLiteral,
    InvalidDigitLiteral,
    EmptyExponentFloat,
    FloatLiteralUnsupportedBase,
}

pub fn lex(source: &str) -> impl Iterator<Item = Token> + use<'_> {
    let mut lexer = Lexer::new(&source);
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

fn is_digit_start(c: char) -> bool {
    return c.is_ascii_digit();
}

// Only continuation variant exists to check digits as the start is checked in
// the tokenizer big match statement

fn is_digit_base10_continuation(c: char) -> bool {
    return c.is_ascii_digit() || c == '_';
}

fn is_digit_base2_continuation(c: char) -> bool {
    return ('0'..='1').contains(&c) || c == '_';
}

fn is_digit_base8_continuation(c: char) -> bool {
    return ('0'..='7').contains(&c) || c == '_';
}

fn is_digit_base16_continuation(c: char) -> bool {
    return c.is_ascii_hexdigit() || c == '_';
}

struct Lexer<'src> {
    source: &'src str,
    cursor: Cursor<'src>,
    pending_token: Option<Token>,
    eof_reached: bool,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source,
            cursor: Cursor::new(source),
            pending_token: None,
            eof_reached: false,
        }
    }

    /// Advance the cursor while the preficate is true and return the substring that was consumed
    fn take_while(&mut self, predicate: impl FnMut(char) -> bool) -> Span {
        let start = self.cursor.consumed;
        self.cursor.advance_while(predicate);
        let end = self.cursor.consumed;
        Span::new(start, end)
    }

    fn tokenize_identifier(&mut self, first_char: char) -> Token {
        let mut identifier = self.take_while(is_identifier_continuation);
        // Add the first char of the identifier already consumed
        // Unwraping here is safe as char::len_utf8() is always between 1 and 4 inclusive
        identifier.start -= u32::try_from(first_char.len_utf8()).unwrap();
        Token::Identifier(identifier)
    }

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

    fn tokenize_number(&mut self, first_digit: char) -> Token {
        debug_assert!(is_digit_start(first_digit));
        let base = self.extract_number_base(first_digit);
        let mut value = self.take_while(is_digit_base16_continuation);
        if base.is_none() {
            // Include first_digit if this is not the first char of the base (the 0 in 0b, 0o or 0x)
            // Its size is always one byte as 0..=9 are ascii characters
            value.start -= 1;
        }
        let suffix = self.take_while(is_identifier_continuation);
        Token::Integer(IntegerLiteral {
            base: base.unwrap_or(IntegerBase::Dec),
            value,
            suffix,
        })
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.pending_token.is_some() {
            let pending_token = self.pending_token;
            self.pending_token = None;
            return pending_token;
        } else if self.eof_reached {
            return None;
        }
        let mut invalid_token_span: Option<Span> = None;
        loop {
            if let Some(c) = self.cursor.next() {
                let token = match c {
                    c if c.is_whitespace() => {
                        match c {
                            '\n' => {
                                // FIXME add file position handling
                                // line += 1;
                                // column = 1;
                            }
                            _ => {}
                        };
                        continue; // Skip whitespaces
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
                    // Gather unknown chars into an invalid token
                    _ => {
                        match invalid_token_span {
                            Some(ref mut span) => span.end += 1,
                            None => {
                                invalid_token_span =
                                    Some(Span::new(self.cursor.consumed - 1, self.cursor.consumed))
                            }
                        }
                        continue;
                    }
                };
                if let Some(span) = invalid_token_span {
                    match token {
                        Token::Err(_) => (),
                        _ => {
                            // Save current valid token for next iteration and
                            // return immediately the invalid one
                            self.pending_token = Some(token);
                            return Some(Token::Err(span));
                        }
                    }
                }
                return Some(token);
            } else {
                self.eof_reached = true;
                if let Some(span) = invalid_token_span {
                    self.pending_token = Some(Token::Eof);
                    return Some(Token::Err(span));
                } else {
                    return Some(Token::Eof);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
