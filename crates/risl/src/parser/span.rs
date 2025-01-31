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
    /// Creates a new span from two start and end byte indices.
    ///
    /// # Panics
    ///
    /// Panics if the passed byte indices cannot be stored in an u32.
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

pub trait SpanMerger {
    fn merge(&mut self, span: Span);
}

impl SpanMerger for Span {
    fn merge(&mut self, span: Span) {
        debug_assert!(self.end < span.end);
        self.end = span.end
    }
}

impl SpanMerger for Option<Span> {
    fn merge(&mut self, span: Span) {
        if let Some(ref mut self_span) = self {
            self_span.merge(span)
        } else {
            *self = Some(span)
        }
    }
}

pub trait SpanSubstr {
    fn substr(&self, span: Span) -> &str;
}

impl SpanSubstr for &str {
    fn substr(&self, span: Span) -> &str {
        &self[(span.start as usize)..(span.end as usize)]
    }
}
