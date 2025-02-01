/// A basic cursor providing iteration capabilities over an unicode character sequence.
#[derive(Debug)]
pub struct Cursor<'src> {
    chars: std::str::Chars<'src>,
    pub consumed: usize,
}

impl<'src> Cursor<'src> {
    /// Creates a new cursor for the given source string.
    pub fn new(source: &'src str) -> Self {
        Self {
            chars: source.chars(),
            consumed: 0,
        }
    }

    /// Returns the remaining string.
    pub fn as_str(&self) -> &'src str {
        self.chars.as_str()
    }

    /// Peeks the next next character, if any.
    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Peeks the n-th next character, if any.
    pub fn peek_nth(&self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }

    /// Moves to the next character.
    /// Does not move the cursor if the next character does not exist.
    pub fn next(&mut self) -> Option<char> {
        let next = self.chars.next();
        if let Some(_) = next {
            self.consumed += 1;
        }
        next
    }

    /// Moves to the n-th next character.
    /// Does not move the cursor if the n-th character does not exist.
    pub fn next_nth(&mut self, n: usize) -> Option<char> {
        let nth = self.chars.nth(n);
        if let Some(_) = nth {
            self.consumed += n;
        }
        nth
    }

    /// Moves to the next character while the predicate returns true for that character.
    pub fn advance_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
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
