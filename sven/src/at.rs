#[derive(Debug, PartialEq, Eq)]
pub enum At {
    Exact(usize),
    After(usize),
    Before(usize),
}

impl At {
    #[inline]
    pub fn start() -> Self {
        Self::Exact(1)
    }
}
