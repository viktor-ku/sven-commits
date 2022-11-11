use std::fmt::Display;

mod header;
pub use header::CommitHeader;

mod footer;
pub use footer::CommitFooter;

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
