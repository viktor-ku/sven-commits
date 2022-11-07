use std::fmt::Display;

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
#[derive(Debug)]
pub struct ConventionalCommit<'c> {
    pub header: CommitHeader<'c>,
    /// There is no particular use of the body defined in the specification,
    /// so our approach is to just store the entire bytes of the body starting
    /// from the first unicode symbol of the 3rd line (1st line for the header,
    /// second line is always just EOL), ending with the last unicode symbol of the
    /// last line of the body paragraph.
    pub body: &'c [u8],
    pub footers: &'c [CommitFooter<'c>],
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
