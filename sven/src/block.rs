use crate::{
    bytes::Bytes,
    domain::{Domain, Scope},
};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Block {
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

    /// Used to mark a jump to (a destination, marked by the index, or None if no such destination
    /// has yet been found) from a misplaced block. When showing final result to users, we should
    /// emphasise such portals as expected places for referred blocks
    Portal(Option<usize>),

    /// Like `Settled`, however "Ref" (stands for reference) indicates that this
    /// is the source of a misplaced (Portal) node. When drawing the end result
    /// we should start an arrow pointing to the right place from this block
    Ref(usize),

    /// A simple marker to show that we should be waiting for the block with this status
    /// later
    Promise,
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
            val: Val::Root,
            domain: Domain::Root,
            bytes: None,
            status: Status::Settled,
        }
    }
}

impl Into<Domain> for Val {
    fn into(self) -> Domain {
        match self {
            Val::Root => Domain::Root,
            Val::Seq => Domain::Type,
            Val::Colon => Domain::Colon,
            Val::Space => Domain::Space,
            Val::None => Domain::None,
            Val::OpenBracket => Domain::Scope(Scope::OpenBracket),
            Val::CloseBracket => Domain::Scope(Scope::CloseBracket),
            Val::ExclMark => Domain::Breaking,
            Val::EOL => Domain::None,
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
