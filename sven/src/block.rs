use crate::{bytes::Bytes, domain::Domain};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Block {
    pub id: usize,
    pub found_at: usize,
    pub val: Val,
    pub domain: Option<Domain>,
    pub bytes: Option<Bytes>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Val {
    Root,
    None,
    Seq,
    Space,
    OpenBracket,
    CloseBracket,
    ExclMark,
    Colon,
    EOL,
}

impl Block {
    #[inline]
    pub fn capture<'capture>(&self, source: &'capture str) -> Option<&'capture str> {
        self.bytes.map(|bytes| bytes.capture(source)).flatten()
    }

    #[inline]
    pub fn root() -> Self {
        Self {
            id: 0,
            found_at: 0,
            val: Val::Root,
            domain: Some(Domain::Root),
            bytes: None,
        }
    }
}

impl Default for Val {
    fn default() -> Self {
        panic!("choose Val manually")
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
