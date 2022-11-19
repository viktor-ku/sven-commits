use crate::at::At;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IssueSubject {
    Type,
    Header,
    Colon,
    Desc,
    Whitespace,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Missing {
    pub expected_at: At,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Misplaced {
    pub expected_at: At,
    pub found_at: At,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IssueData {
    Missing(Missing),
    Misplaced(Misplaced),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Issue {
    pub id: usize,
    pub subject: IssueSubject,
    pub data: IssueData,
}
