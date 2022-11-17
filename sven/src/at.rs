#[derive(Debug, PartialEq, Eq)]
pub enum AtPos {
    After,
    Before,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AtTarget {
    Root,
    Token(usize),
    Issue(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct At {
    pub pos: AtPos,
    pub target: AtTarget,
}

impl At {
    #[inline]
    pub fn start() -> Self {
        Self {
            pos: AtPos::After,
            target: AtTarget::Root,
        }
    }

    #[inline]
    pub fn after(target: AtTarget) -> Self {
        Self {
            pos: AtPos::After,
            target,
        }
    }

    #[inline]
    pub fn before(target: AtTarget) -> Self {
        Self {
            pos: AtPos::Before,
            target,
        }
    }
}
