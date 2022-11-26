#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Subject {
    Root,
    Kind,
    Colon,
    Space,
    Desc,
}
