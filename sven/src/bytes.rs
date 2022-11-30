use pest::Span;
use std::ops::{Range, RangeFrom};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Bytes(usize, usize);

impl Bytes {
    #[inline]
    pub fn new(start: usize, end: usize) -> Self {
        debug_assert!(end >= start);
        Self(start, end)
    }

    #[inline]
    pub fn empty_at(start: usize) -> Self {
        start.into()
    }

    #[inline]
    pub fn total(&self) -> usize {
        self.1 - self.0
    }

    #[inline]
    pub fn capture<'capture>(&self, source: &'capture str) -> Option<&'capture str> {
        match self.total() {
            0 => None,
            _ => source.get(self.0..self.1),
        }
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.1
    }
}

impl Into<Bytes> for (usize, usize) {
    #[inline]
    fn into(self) -> Bytes {
        Bytes::new(self.0, self.1)
    }
}

impl Into<Bytes> for usize {
    #[inline]
    fn into(self) -> Bytes {
        Bytes::new(self, self)
    }
}

impl Into<Range<usize>> for Bytes {
    #[inline]
    fn into(self) -> Range<usize> {
        Range {
            start: self.0,
            end: self.1,
        }
    }
}

impl Into<RangeFrom<usize>> for Bytes {
    #[inline]
    fn into(self) -> RangeFrom<usize> {
        RangeFrom { start: self.0 }
    }
}

impl Into<Bytes> for Span<'_> {
    #[inline]
    fn into(self) -> Bytes {
        Bytes::new(self.start(), self.end())
    }
}
