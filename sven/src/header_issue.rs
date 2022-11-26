pub mod header {
    use crate::{at::At, weak_commit::BytesRange};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum IssueSubject {
        Kind,
        Header,
        Colon,
        Desc,
        Space,
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
}
