use crate::{domain::Domain, weak_commit::BytesRange};
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub id: usize,
    pub found_at: usize,
    pub val: Val,
    pub bytes: BytesRange,
    pub info: Info,

    #[cfg(debug_assertions)]
    pub source: String,
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

#[cfg(debug_assertions)]
impl Display for Val {
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
        let val = {
            let val = format!("{}", self.val);
            let len = val.len();
            let diff = 12 - len;
            format!("{}{}", val, " ".repeat(diff))
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

        write!(f, "{} {} {:?}", val, domain, self.bytes)?;

        match self.val {
            Val::Root => {}
            Val::EOL => {
                write!(f, " \\n")?;
            }
            _ => {
                write!(f, " \"{}\"", self.capture(&self.source))?;
            }
        };

        write!(f, "")
    }
}
