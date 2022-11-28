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
