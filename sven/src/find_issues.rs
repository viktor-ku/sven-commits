use crate::weak_commit::{Token, TokenKind, WeakCommit};
use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum IssueSubject {
    Type,
    Header,
    Colon,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotFoundIssue {
    pub subject: IssueSubject,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MisplacedIssue {
    pub subject: IssueSubject,
    pub expected_at: usize,
    pub found_at: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Issue {
    NotFound(NotFoundIssue),
    Misplaced(MisplacedIssue),
}

fn find_header_issues(tokens: &[Token], issues: &mut Vec<Issue>) {
    // try to find the first colon
    //
    // if we don't then consider the entire header to make no sense in the context
    // of the conventional commit structure
    let colon = tokens.iter().find(|&token| token.kind == TokenKind::Colon);

    if colon.is_none() {
        issues.push(Issue::NotFound(NotFoundIssue {
            subject: IssueSubject::Header,
        }));
        return;
    }

    let colon = colon.expect("already checked that the colon is present");

    // by this point we should know we have something other than EOL in our
    // header, as well as we have a colon

    // we know we should find exactly one word at the beginning
    // if we don't then it makes no sense to advance because there is simply no type
    // to work with
    let mut iter = tokens.iter();
    if let Some(token) = iter.next() {
        match token.kind {
            TokenKind::Word => {}
            _ => {
                issues.push(Issue::NotFound(NotFoundIssue {
                    subject: IssueSubject::Type,
                }));
                return;
            }
        }
    }

    // we registered first word to be the type
    // we expect Colon or OpenBracket or ExclMark right after
    if let Some(token) = iter.next() {
        match token.kind {
            TokenKind::Colon | TokenKind::OpenBracket | TokenKind::ExclMark => {}
            _ => issues.push(Issue::Misplaced(MisplacedIssue {
                subject: IssueSubject::Colon,
                // to know expected_at we should know the boundaries of the type we already
                // captured
                expected_at: 0,
                // to know the found_at we should know the boundaries of the colon
                found_at: 0,
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
        let expected = vec![Issue::NotFound(NotFoundIssue {
            subject: IssueSubject::Header,
        })];
        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_no_header() {
        let commit = "";
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        let expected = vec![Issue::NotFound(NotFoundIssue {
            subject: IssueSubject::Header,
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
        let expected = vec![Issue::NotFound(NotFoundIssue {
            subject: IssueSubject::Header,
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
        let expected = vec![Issue::NotFound(NotFoundIssue {
            subject: IssueSubject::Type,
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
        let expected = vec![Issue::NotFound(NotFoundIssue {
            subject: IssueSubject::Type,
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
        let expected = vec![Issue::Misplaced(MisplacedIssue {
            subject: IssueSubject::Colon,
            expected_at: 4,
            found_at: 14,
        })];
        assert_eq!(actual, expected);
    }
}
