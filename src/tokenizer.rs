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
// Based on https://brunocalza.me/writing-a-simple-lexer-in-rust/
pub mod manual_loop {
    use super::{Error, Token};

    pub fn tokenize(source: &str) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut iter = source.char_indices().peekable();
        let mut line: u64 = 0;
        let mut column: u64 = 0;

        while let Some((index, ch)) = iter.next() {
            column += 1;
            match ch {
                ch if ch.is_whitespace() => match ch {
                    '\n' => {
                        line += 1;
                        column = 1;
                    }
                    _ => {}
                },
                // Single-character tokens
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '{' => tokens.push(Token::LeftBrace),
                '}' => tokens.push(Token::RightBrace),
                '[' => tokens.push(Token::LeftBracket),
                ']' => tokens.push(Token::RightBracket),
                ',' => tokens.push(Token::Comma),
                '.' => tokens.push(Token::Dot),
                '-' => tokens.push(Token::Minus),
                '+' => tokens.push(Token::Plus),
                ':' => tokens.push(Token::Colon),
                ';' => tokens.push(Token::Semicolon),
                '/' => tokens.push(Token::Slash),
                '\\' => tokens.push(Token::Backslash),
                '*' => tokens.push(Token::Star),
                '&' => tokens.push(Token::Ampersand),
                '|' => tokens.push(Token::Pipe),
                // One or two character tokens
                '!' => match iter.peek() {
                    Some((_, '=')) => {
                        iter.next();
                        tokens.push(Token::NotEqual)
                    }
                    _ => tokens.push(Token::Not),
                },
                '=' => match iter.peek() {
                    Some((_, '=')) => {
                        iter.next();
                        tokens.push(Token::EqualEqual)
                    }
                    _ => tokens.push(Token::Equal),
                },
                '>' => match iter.peek() {
                    Some((_, '=')) => {
                        iter.next();
                        tokens.push(Token::GreaterEqual)
                    }
                    _ => tokens.push(Token::Greater),
                },
                '<' => match iter.peek() {
                    Some((_, '=')) => {
                        iter.next();
                        tokens.push(Token::LessEqual)
                    }
                    _ => tokens.push(Token::Less),
                },
                // Literals
                '1'..='9' => {
                    let start_index = index;
                    // Extract number literals
                    // _ is accepted as digit separator
                    // integers:
                    //   can be prefixed by a base (0x, 0o or 0b)
                    //   can be suffixed by a type ({u,i}{8,16,32,64})
                    // floats:
                    //   {integer part}.{decimal part}
                    //   e-notation: 1e6, 7.6e-4
                    //   can be suffixed by a type (f{32,64})
                    match iter
                        .by_ref()
                        .take_while(|&(_index, ch)| /*TODO add all cases*/ ch.is_ascii_digit())
                        .last()
                    {
                        Some((index, _ch)) => {
                            // The iterator is only taking valid chars for numeric literals to the conversion will not fail
                            let n: i64 = source[start_index..=index].parse().unwrap();
                            tokens.push(Token::Number(n));
                        }
                        _ => {
                            return Err(Error {
                                what: String::from("Invalid numeric literal"),
                                line,
                                column,
                            })
                        }
                    }
                }
                ch if ch.is_alphabetic() || ch == '_' => {
                    // TODO add identifiers handling
                }
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
            assert_eq!(
                result,
                Ok(vec![
                    Token::Comma,
                    Token::Not,
                    Token::LeftParen,
                    Token::RightParen,
                    Token::NotEqual,
                    Token::Eof
                ])
            );
        }
    }
}

// TODO (2) implement an iterator adapter for &str (and String?)
// let tokens = "let answer = 42;".tokenize().collect::Vec<_>();
// Pros: Only an iterator on the input is returned, the user can collect the
//       tokens however he wants, implementation is more compact than (1)
// Cons: Interface is a bit more complex than (1), but still easy to use
pub mod iterator {
    use super::Error;
    use super::Token;

    pub fn tokenize<'a>(source: &'a str) -> impl Iterator<Item = Token> + use<'a> {
        source
            .char_indices()
            .map(|(_index, _char)| Token::Ampersand)
    }
}

// TODO (3) implement a lexer based on https://crates.io/crates/logos for performance comparison
// See https://alic.dev/blog/fast-lexing
