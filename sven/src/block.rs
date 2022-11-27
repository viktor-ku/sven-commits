use crate::{domain::Domain, weak_commit::BytesRange};
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    /// An id for the block created with a space for inserting other
    /// blocks in the middle
    pub id: usize,

    /// Actual index by which this block could be found
    pub found_at: usize,

    /// Kind of the block from the general point of view right after the parsing,
    /// includes e.g. Colon which represents a colon (":"), but it has nothing to do
    /// with the conventional commit colon in header because it might not be unique in
    /// the set of blocks, but also it's not necessary represents the actual colon that
    /// comes after the conventional commit type or scope
    pub kind: Kind,

    /// Bytes range taken by the block
    pub bytes: BytesRange,

    /// Actual information about the block in the context of conventional commit
    pub info: Info,

    #[cfg(debug_assertions)]
    pub source: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Kind {
    /// Any sequence of any utf8 characters, excluding other kinds of token
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
    /// If any particular block is identified as a part of conventional commit
    /// structure then it gets assigned a certain subject within the specification
    pub domain: Option<Domain>,
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
impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Root => "Root",
                Self::Seq => "Seq",
                Self::Space => "Space",
                Self::OpenBracket => "OpenBracket",
                Self::CloseBracket => "CloseBracket",
                Self::ExclMark => "ExclMark",
                Self::Colon => "Colon",
                Self::EOL => "EOL",
            }
        )
    }
}

#[cfg(debug_assertions)]
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = {
            let kind = format!("{}", self.kind);
            let len = kind.len();
            let diff = 12 - len;
            format!("{}{}", kind, " ".repeat(diff))
        };
        let domain = {
            let domain = match self.info.domain {
                Some(domain) => format!("{}", domain),
                None => "-".to_string(),
            };
            let len = domain.len();
            let diff = 9 - len;
            format!("{}{}", domain, " ".repeat(diff))
        };

        let at = self.found_at;
        write!(f, "{}", at)?;
        if at < 10 {
            write!(f, " ")?;
        }
        write!(f, " ")?;

        write!(f, "{} {} {:?}", kind, domain, self.bytes)?;

        match self.kind {
            Kind::Root => {}
            Kind::EOL => {
                write!(f, " \\n")?;
            }
            _ => {
                write!(f, " \"{}\"", self.capture(&self.source))?;
            }
        };

        write!(f, "")
    }
}
