use crate::{bytes::Bytes, domain::Domain};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub id: usize,
    pub found_at: usize,
    pub val: Val,
    pub domain: Option<Domain>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Val {
    Seq(Bytes),
    Root,
    Space,
    OpenBracket,
    CloseBracket,
    ExclMark,
    Colon,
    EOL,
}

impl Block {
    #[inline]
    pub fn capture<'a>(&self, source: &'a str) -> Option<&'a str> {
        match self.val {
            Val::Seq(bytes) => bytes.capture(source),
            _ => None,
        }
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
