use crate::weak_commit::{Token, TokenKind, WeakCommit};
use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum Subject {
    Type,
    Header,
    Colon,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Missing {
    pub subject: Subject,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Misplaced {
    pub subject: Subject,
    /// nth byte, starting from 0
    pub expected_at: usize,
    /// nth byte, starting from 0
    pub found_at: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Issue {
    Missing(Missing),
    Misplaced(Misplaced),
}

fn find_header_issues(tokens: &[Token], issues: &mut Vec<Issue>) {
    let colon = tokens.iter().find(|&token| token.kind == TokenKind::Colon);

    if colon.is_none() {
        issues.push(Issue::Missing(Missing {
            subject: Subject::Header,
        }));
        return;
    }

    let colon = colon.expect("already checked that the colon is present");

    let mut iter = tokens.iter();
    let type_token: &Token = match iter.next() {
        None => return,
        Some(token) => match token.kind {
            TokenKind::Word => token,
            _ => {
                issues.push(Issue::Missing(Missing {
                    subject: Subject::Type,
                }));
                return;
            }
        },
    };

    if let Some(token) = iter.next() {
        match token.kind {
            TokenKind::Colon | TokenKind::OpenBracket | TokenKind::ExclMark => {}
            _ => issues.push(Issue::Misplaced(Misplaced {
                subject: Subject::Colon,
                expected_at: type_token.end,
                found_at: colon.start,
            })),
        }
    }
}

pub fn find_issues(weak_commit: WeakCommit) -> Result<Vec<Issue>> {
    let mut v = Vec::new();

    let tokens = weak_commit.parse_header()?;
    find_header_issues(&tokens, &mut v);

    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn perfect_reference_no_issues() {
        let commit = r###"
fix(refs)!: a simple fix

Раз два три

This test proves utf8 works

Refs: #1001
BREAKING CHANGE: supports many footers
"###
        .trim_start();
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        assert_eq!(actual, Vec::new());
    }

    #[test]
    fn eol_no_header() {
        let commit = "\n";
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        let expected = vec![Issue::Missing(Missing {
            subject: Subject::Header,
        })];
        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_no_header() {
        let commit = "";
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        let expected = vec![Issue::Missing(Missing {
            subject: Subject::Header,
        })];
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_colon_no_header() {
        let commit = r###"
no colon means we do not consider this to be a header at all
"###
        .trim_start();
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        let expected = vec![Issue::Missing(Missing {
            subject: Subject::Header,
        })];
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_word_before_colon_no_type() {
        let commit = r###"
: this means we have no type
"###
        .trim_start();
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        let expected = vec![Issue::Missing(Missing {
            subject: Subject::Type,
        })];
        assert_eq!(actual, expected);
    }

    #[test]
    fn space_at_the_beginning_no_type() {
        let commit = r###"
 : space at the beginning instead of a word also means we have no type
# it also makes no sense to say that the colon has been misplaced since
# there is no type
"###
        .trim_start();
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        let expected = vec![Issue::Missing(Missing {
            subject: Subject::Type,
        })];
        assert_eq!(actual, expected);
    }

    #[test]
    fn misplaced_colon() {
        let commit = r###"
one two three: expected colon, scope or "!" after the type "one", got "..."
"###
        .trim_start();
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        let expected = vec![Issue::Misplaced(Misplaced {
            subject: Subject::Colon,
            expected_at: 3,
            found_at: 13,
        })];
        assert_eq!(actual, expected);
    }
}
