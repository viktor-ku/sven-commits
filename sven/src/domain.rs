#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum Domain {
    Root,
    None,
    Type,
    Scope(Scope),
    Breaking,
    Colon,
    Space,
    Desc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum Scope {
    OpenBracket,
    Scope,
    CloseBracket,
}

impl Default for Domain {
    fn default() -> Self {
        Self::None
    }
}
