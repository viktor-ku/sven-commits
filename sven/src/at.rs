use crate::{find_issues::Issue, weak_commit::parse_header::Token};

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
    pub fn start() -> Self {
        Self {
            pos: AtPos::After,
            target: AtTarget::Root,
        }
    }
}
