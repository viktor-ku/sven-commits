use std::fmt::Display;

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
