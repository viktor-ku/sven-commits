use crate::{domain::Domain, weak_commit::BytesRange};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub id: usize,
    pub found_at: usize,
    pub val: Val,
    pub bytes: BytesRange,
    pub info: Info,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Val {
    Seq,
    Root,
    Space,
    OpenBracket,
    CloseBracket,
    ExclMark,
    Colon,
    EOL,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Info {
    pub domain: Option<Domain>,
}

impl Block {
    #[inline]
    pub fn capture<'a>(&self, source: &'a str) -> &'a str {
        &source[self.bytes.start..self.bytes.end]
    }

    #[inline]
    pub fn total(&self) -> usize {
        self.bytes.total()
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl Into<(usize, usize)> for Block {
    fn into(self) -> (usize, usize) {
        self.bytes.into()
    }
}
