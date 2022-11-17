use crate::at::At;

#[derive(Debug, PartialEq, Eq)]
pub enum IssueSubject {
    Type,
    Header,
    Colon,
    Desc,
    Whitespace,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Missing {
    pub expected_at: At,
}

#[derive(Debug, PartialEq, Eq)]
pub enum IssueData {
    Missing(Missing),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Issue {
    pub id: usize,
    pub subject: IssueSubject,
    pub data: IssueData,
}
