#[derive(Debug, PartialEq, Eq)]
pub enum AtPos {
    Exactly,
    After,
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
    pub fn exactly(target: AtTarget) -> Self {
        Self {
            pos: AtPos::Exactly,
            target,
        }
    }

    #[inline]
    pub fn exactly_token(id: usize) -> Self {
        Self::exactly(AtTarget::Token(id))
    }

    #[inline]
    pub fn exactly_issue(id: usize) -> Self {
        Self::exactly(AtTarget::Issue(id))
    }

    #[inline]
    pub fn after(target: AtTarget) -> Self {
        Self {
            pos: AtPos::After,
            target,
        }
    }

    #[inline]
    pub fn after_token(id: usize) -> Self {
        Self::after(AtTarget::Token(id))
    }

    #[inline]
    pub fn after_issue(id: usize) -> Self {
        Self::after(AtTarget::Issue(id))
    }
}
