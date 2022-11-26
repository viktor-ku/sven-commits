use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Subject {
    Root,
    Kind,
    Colon,
    Space,
    Desc,
}

impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::Root => write!(f, "root"),
            Subject::Kind => write!(f, "type"),
            Subject::Colon => write!(f, "colon"),
            Subject::Space => write!(f, "space"),
            Subject::Desc => write!(f, "desc"),
        }
    }
}
