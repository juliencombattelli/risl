#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Token<'source> {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Backslash,
    Star,

    // One or two character tokens
    Ampersand,
    Pipe,
    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(&'source str),
    String(&'source str),
    Number(i64),

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
}

#[derive(Eq, PartialEq, Debug)]
pub struct Error {
    pub what: String,
    pub line: u64,
    pub column: u64,
}

// TODO (1) tokenize using a manual loop and return a token list
// let tokens = tokenize("let answer = 42;");
// Pros: Interface is simple, implementation is verbose but simple
// Cons: Vec<> usage is forced
mod manual_loop {
    use super::{Error, Token};
    use std::iter::{self, from_fn};

    pub fn tokenize(source: &str) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut iter = source.chars().peekable();
        let mut line: u64 = 0;
        let mut column: u64 = 0;

        while let Some(ch) = iter.next() {
            column += 1;
            match ch {
                ch if ch.is_whitespace() => match ch {
                    '\n' => {
                        line += 1;
                        column = 1;
                    }
                    _ => {}
                },
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '!' => match iter.peek() {
                    Some('=') => {
                        iter.next();
                        tokens.push(Token::NotEqual)
                    }
                    _ => tokens.push(Token::Not),
                },
                '=' => match iter.peek() {
                    Some('=') => {
                        iter.next();
                        tokens.push(Token::EqualEqual)
                    }
                    _ => tokens.push(Token::Not),
                },
                '1'..='9' => {
                    let n: i64 = iter::once(ch)
                        .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    tokens.push(Token::Number(n));
                }
                ch if ch.is_alphabetic() => {}
                _ => {
                    return Err(Error {
                        what: String::from("Syntax error"),
                        line,
                        column,
                    })
                }
            }
        }
        tokens.push(Token::Eof);
        Ok(tokens)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn tokenizer() {
            let result = tokenize("Hello, world! This (is) a test != working");
            // TODO intentionnally wrong
            assert_eq!(result, Ok(vec![Token::Ampersand]));
        }
    }
}

// TODO (2) implement an iterator adapter for &str (and String?)
// let tokens = "let answer = 42;".tokenize().collect::Vec<_>();
// Pros: Only an iterator on the input is returned, the user can collect the
//       tokens however he wants, implementation is more compact than (1)
// Cons: Interface is a bit more complex than (1), but still easy to use
mod iterator {
    use super::Error;
    use super::Token;

    pub fn tokenize<'a>(source: &'a str) -> impl Iterator<Item = Token> + use<'a> {
        source
            .char_indices()
            .map(|(_index, _char)| Token::Ampersand)
    }
}
