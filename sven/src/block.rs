use crate::{bytes::Bytes, domain::Domain};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub id: Option<usize>,
    pub val: Val,
    pub domain: Domain,
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
            id: Some(0),
            val: Val::Root,
            domain: Domain::Root,
            bytes: None,
        }
    }
}

impl PartialOrd for Block {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Block {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.id, other.id) {
            (Some(me_id), Some(other_id)) => me_id.cmp(&other_id),
            _ => self.domain.cmp(&other.domain),
        }
    }
}
