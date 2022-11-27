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
            Subject::Root => write!(f, "Root"),
            Subject::Type => write!(f, "Type"),
            Subject::Scope => write!(f, "Scope"),
            Subject::Breaking => write!(f, "Breaking"),
            Subject::Colon => write!(f, "Colon"),
            Subject::Space => write!(f, "Space"),
            Subject::Desc => write!(f, "Desc"),
        }
    }
}
