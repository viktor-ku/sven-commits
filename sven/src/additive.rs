pub struct Additive {
    pub val: usize,
    pub step: usize,
}

impl Additive {
    #[inline]
    pub fn new() -> Self {
        Self { val: 0, step: 1 }
    }

    pub fn stamp(&mut self) -> usize {
        let x = self.val;
        self.val += self.step;
        x
    }
}
