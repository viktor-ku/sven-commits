#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Bytes(pub usize, pub usize);

impl Bytes {
    #[inline]
    pub fn capture<'a>(&self, source: &'a str) -> Option<&'a str> {
        source.get(self.0..self.1)
    }
}
