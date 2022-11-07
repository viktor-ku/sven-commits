#[derive(Debug, PartialEq, Eq)]
pub enum TypeIssue {
    NotFound,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Issue {
    Type(TypeIssue),
}
