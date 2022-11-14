pub struct Additive {
    inner: usize,
}

impl Additive {
    #[inline]
    pub fn new() -> Self {
        Self { inner: 0 }
    }

    pub fn stamp(&mut self) -> usize {
        self.inner += 1;
        self.inner
    }
}
