use std::fmt::Display;

#[derive(Debug)]
pub struct CommitHeader<'c> {
    pub kind: &'c str,
    pub scope: Option<&'c str>,
    pub desc: &'c str,
    pub breaking_change: bool,
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
