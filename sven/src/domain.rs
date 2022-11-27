use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Domain {
    Root,
    Type,
    Scope,
    Breaking,
    Colon,
    Space,
    Desc,
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root => write!(f, "Root"),
            Self::Type => write!(f, "Type"),
            Self::Scope => write!(f, "Scope"),
            Self::Breaking => write!(f, "Breaking"),
            Self::Colon => write!(f, "Colon"),
            Self::Space => write!(f, "Space"),
            Self::Desc => write!(f, "Desc"),
        }
    }
}
