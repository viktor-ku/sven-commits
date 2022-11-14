#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BytesRange {
    /// Starting byte
    pub start: usize,
    /// Ending byte
    pub end: usize,
}

impl BytesRange {
    /// Convenience fn to get the total bytes of the
    /// associated bytes range.
    #[inline]
    pub fn total(&self) -> usize {
        debug_assert!(self.end >= self.start);
        self.end - self.start
    }
}

impl Into<(usize, usize)> for BytesRange {
    fn into(self) -> (usize, usize) {
        (self.start, self.end)
    }
}
