use crate::{bytes::Bytes, domain::Domain};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Block {
    pub id: Option<usize>,
    pub val: Val,
    pub domain: Domain,
    pub bytes: Option<Bytes>,
    pub status: Status,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    /// Used for any block, unsigned block means it has not been
    /// processed yet, no unsigned blocks should be used after the analysis
    Unsigned,

    /// Used for any block that has been found to be meaningful in the context of
    /// the conventional commit (e.g. it's a colon, or a type, or scope, etc.)
    Settled,

    /// Used to mark a block as missing from the input, these blocks are going
    /// to be inserted into the blocks set during the analysis. Having block with _missing_
    /// status does not make them invalid, however when rendering results to the user
    /// these blocks should be emphasised as such
    Missing,

    /// Used to mark a block as extra, or not useful in the context of the conventional commit,
    /// as well as missing blocks, these blocks are also used in the final result to show
    /// users that something in their input was not expected
    Extra,

    /// Used to mark a jump to (a destination) from a misplaced block. When showing final
    /// result to users, we should emphasise such portals as expected places for referred
    /// blocks
    Portal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
            status: Status::Settled,
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

impl Default for Val {
    fn default() -> Self {
        Self::None
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::Unsigned
    }
}
