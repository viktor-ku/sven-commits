use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Subject {
    Root,
    Type,
    Scope,
    Breaking,
    Colon,
    Space,
    Desc,
}

impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::Root => write!(f, "root"),
            Subject::Type => write!(f, "type"),
            Subject::Scope => write!(f, "scope"),
            Subject::Breaking => write!(f, "breaking"),
            Subject::Colon => write!(f, "colon"),
            Subject::Space => write!(f, "space"),
            Subject::Desc => write!(f, "desc"),
        }
    }
}
