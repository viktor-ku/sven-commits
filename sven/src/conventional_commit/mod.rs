use crate::weak_commit::WeakCommit;
use std::fmt::Display;

mod issue;
pub use issue::{Issue, TypeIssue};

/// [Conventional Commits Specification](https://www.conventionalcommits.org/en/v1.0.0/)
///
/// Once this struct has been obtained, it might be safely assumed to have a
/// fully compatible commit at hand.
///
/// It is assumed that the specification is supporting utf8
/// within commits, therefore users can write their commits in any language.
/// This structure guarantees that the commit at hand, even if not written in
/// english, still contains necessary information such as _"fix"_, _"feat"_, etc.
///
/// There is no direct relation with the original commit message.
/// That is, it might have been modified to fit (e.g. mapped set of unicode
/// symbols to specific keywords within the spec), or used as is.
///
/// Every text (a str really) contained in this struct is expected to be trimmed
/// down because when `Display`'ing this struct it will format data in the
/// expected for the specification way.
#[derive(Debug)]
pub struct ConventionalCommit<'c> {
    pub header: CommitHeader<'c>,
    /// If there is any body (get it?) it should start with the first utf8
    /// char of the 3rd line (1st for the header, 2nd is just EOL) and end
    /// with the last char of the last paragraph (char before EOL or EOI)
    pub body: Option<&'c str>,
    pub footers: &'c [CommitFooter<'c>],
}

impl<'c> ConventionalCommit<'c> {
    pub fn find_issues(weak_commit: WeakCommit) -> Vec<Issue> {
        let mut v = Vec::new();

        let weak_header = weak_commit.parse_header().unwrap();

        if weak_header.kind.is_none() {
            v.push(Issue::Type(TypeIssue::NotFound));
        }

        v
    }
}

#[derive(Debug)]
pub struct CommitHeader<'c> {
    pub kind: &'c str,
    pub scope: Option<&'c str>,
    pub desc: &'c str,
    pub breaking_change: bool,
}

#[derive(Debug)]
pub enum CommitFooter<'c> {
    Simple(&'c str, &'c str),
    BreakingChange(&'c str),
}

impl Display for CommitFooter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitFooter::Simple(k, v) => {
                write!(f, "{}: {}", k, v)
            }
            CommitFooter::BreakingChange(v) => {
                write!(f, "BREAKING CHANGE: {}", v)
            }
        }
    }
}

impl Display for CommitHeader<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)?;

        if let Some(scope) = self.scope {
            write!(f, "({})", scope)?;
        }

        if self.breaking_change {
            write!(f, "!")?;
        }

        write!(f, ": {}", self.desc)
    }
}

impl Display for ConventionalCommit<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.header)?;

        if let Some(body) = self.body {
            write!(f, "\n{}\n", body)?;
        }

        if !self.footers.is_empty() {
            write!(f, "\n")?;
            for footer in self.footers {
                write!(f, "{}\n", footer)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod find_issues {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn no_issues() {
        let commit = r###"
fix(refs)!: a simple fix

Раз два три

This test proves utf8 works

Refs: #1001
BREAKING CHANGE: supports many footers
"###
        .trim_start();
        let actual = ConventionalCommit::find_issues(WeakCommit::parse(commit).unwrap());
        let expected = Vec::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn type_not_found() {
        let commit = r###"
imagine nothing
"###
        .trim_start();
        let actual = ConventionalCommit::find_issues(WeakCommit::parse(commit).unwrap());
        let expected = vec![Issue::Type(TypeIssue::NotFound)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn type_not_found_only_colon() {
        let commit = r###"
:imagine
"###
        .trim_start();
        let actual = ConventionalCommit::find_issues(WeakCommit::parse(commit).unwrap());
        let expected = vec![Issue::Type(TypeIssue::NotFound)];
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod display {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn display_header() {
        let actual = ConventionalCommit {
            header: CommitHeader {
                kind: "fix",
                scope: None,
                desc: "a simple fix",
                breaking_change: false,
            },
            body: None,
            footers: &[],
        };
        let expected = r###"
fix: a simple fix
"###
        .trim_start();
        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn display_header_body() {
        let actual = ConventionalCommit {
            header: CommitHeader {
                kind: "fix",
                scope: None,
                desc: "a simple fix",
                breaking_change: false,
            },
            body: Some("Very simple commit body message"),
            footers: &[],
        };
        let expected = r###"
fix: a simple fix

Very simple commit body message
"###
        .trim_start();
        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn display_header_body_footer() {
        let actual = ConventionalCommit {
            header: CommitHeader {
                kind: "fix",
                scope: None,
                desc: "a simple fix",
                breaking_change: false,
            },
            body: Some("Very simple commit body message"),
            footers: &[CommitFooter::Simple("Refs", "#1001")],
        };
        let expected = r###"
fix: a simple fix

Very simple commit body message

Refs: #1001
"###
        .trim_start();
        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn display_header_footer() {
        let actual = ConventionalCommit {
            header: CommitHeader {
                kind: "fix",
                scope: None,
                desc: "a simple fix",
                breaking_change: false,
            },
            body: None,
            footers: &[CommitFooter::Simple("Refs", "#1001")],
        };
        let expected = r###"
fix: a simple fix

Refs: #1001
"###
        .trim_start();
        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn display_header_many_footer() {
        let actual = ConventionalCommit {
            header: CommitHeader {
                kind: "fix",
                scope: None,
                desc: "a simple fix",
                breaking_change: false,
            },
            body: None,
            footers: &[
                CommitFooter::Simple("Refs", "#1001"),
                CommitFooter::BreakingChange("supports many footers"),
            ],
        };
        let expected = r###"
fix: a simple fix

Refs: #1001
BREAKING CHANGE: supports many footers
"###
        .trim_start();
        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn display_header_many_body_many_footer() {
        let actual = ConventionalCommit {
            header: CommitHeader {
                kind: "fix",
                scope: Some("refs"),
                desc: "a simple fix",
                breaking_change: true,
            },
            body: Some("Раз два три\n\nThis test proves utf8 works"),
            footers: &[
                CommitFooter::Simple("Refs", "#1001"),
                CommitFooter::BreakingChange("supports many footers"),
            ],
        };
        let expected = r###"
fix(refs)!: a simple fix

Раз два три

This test proves utf8 works

Refs: #1001
BREAKING CHANGE: supports many footers
"###
        .trim_start();
        assert_eq!(format!("{}", actual), expected);
    }
}

#[cfg(test)]
mod header {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn display() {
        let actual = CommitHeader {
            kind: "fix",
            scope: None,
            breaking_change: false,
            desc: "a simple fix",
        };
        let expected = "fix: a simple fix";
        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn display_scope() {
        let actual = CommitHeader {
            kind: "fix",
            scope: Some("color"),
            breaking_change: false,
            desc: "account for the shadows",
        };
        let expected = "fix(color): account for the shadows";
        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn display_breaking_change() {
        let actual = CommitHeader {
            kind: "feat",
            scope: None,
            breaking_change: true,
            desc: "enable new feature",
        };
        let expected = "feat!: enable new feature";
        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn display_breaking_change_and_scope() {
        let actual = CommitHeader {
            kind: "feat",
            scope: Some("wallet"),
            breaking_change: true,
            desc: "require set of keys",
        };
        let expected = "feat(wallet)!: require set of keys";
        assert_eq!(format!("{}", actual), expected);
    }
}

#[cfg(test)]
mod footer {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn display() {
        let actual = CommitFooter::Simple("Refs", "#1001");
        let expected = "Refs: #1001";
        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn display_breaking_change() {
        let actual = CommitFooter::BreakingChange("Uses different version now");
        let expected = "BREAKING CHANGE: Uses different version now";
        assert_eq!(format!("{}", actual), expected);
    }
}
