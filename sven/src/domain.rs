#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Domain {
    Root,
    Type,
    Scope(Scope),
    Breaking,
    Colon,
    Space,
    Desc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Scope {
    OpenBracket,
    Scope,
    CloseBracket,
}
