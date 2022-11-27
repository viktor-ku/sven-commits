use crate::weak_commit::BytesRange;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum BlockKind {
    /// Any sequence of any utf8 characters, excluding other kinds of token
    Seq,
    Space,
    OpenBracket,
    CloseBracket,
    ExclMark,
    Colon,
    EOL,
}

#[derive(PartialEq, Eq)]
pub struct Block {
    /// An id for the block created with a space for inserting other
    /// blocks in the middle
    pub id: usize,

    /// Actual index by which this block could be found
    pub at: usize,

    pub kind: BlockKind,

    /// Bytes range taken by the block
    pub bytes: BytesRange,

    #[cfg(debug_assertions)]
    pub source: String,
}

impl Block {
    /// Having the original commit you can easily capture the relevant
    /// str for the current block
    #[inline]
    pub fn capture<'a>(&self, source: &'a str) -> &'a str {
        &source[self.bytes.start..self.bytes.end]
    }

    #[inline]
    pub fn total(&self) -> usize {
        self.bytes.total()
    }
}

impl BlockKind {
    pub fn stringify<'a>(&self) -> &'a str {
        match self {
            BlockKind::Seq => "Seq",
            BlockKind::Space => "Space",
            BlockKind::OpenBracket => "OpenBracket",
            BlockKind::CloseBracket => "CloseBracket",
            BlockKind::ExclMark => "ExclMark",
            BlockKind::Colon => "Colon",
            BlockKind::EOL => "EOL",
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

impl Into<(usize, usize)> for Block {
    fn into(self) -> (usize, usize) {
        self.bytes.into()
    }
}

#[cfg(debug_assertions)]
impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = {
            let kind_str = self.kind.stringify();
            let len = kind_str.len();
            let diff = 10 - len;
            format!("{:?}{}", self.kind, " ".repeat(diff))
        };

        let at = self.at;
        write!(f, "{}", at)?;
        if at < 10 {
            write!(f, " ")?;
        }
        write!(f, " ")?;

        write!(
            f,
            "{} {:?} \"{}\"",
            kind,
            self.bytes,
            match self.kind {
                BlockKind::EOL => {
                    "\\n"
                }
                _ => self.capture(&self.source),
            }
        )
    }
}
