use anyhow::Result;

use crate::weak_commit::{Token, WeakCommit};

#[derive(Debug, PartialEq, Eq)]
pub enum TypeIssue {
    NotFound,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HeaderIssue {
    NotFound,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Issue {
    Type(TypeIssue),
    Header(HeaderIssue),
}

fn find_header_issues(tokens: &[Token], issues: &mut Vec<Issue>) {
    // try to find the first colon
    //
    // if we don't then consider the entire header to make no sense in the context
    // of the conventional commit structure
    //
    //

    let colon = tokens.iter().find(|&token| *token == Token::Colon);

    if colon.is_none() {
        issues.push(Issue::Header(HeaderIssue::NotFound));
        return;
    }

    let _colon = colon.expect("already ensured the colon is some");

    // by this point we should know we have something other than EOL in our
    // header, as well as we have a colon

    // we know we should find exactly one word at the beginning
    if let Some(token) = tokens.first() {
        match token {
            Token::Word(_) => {}
            _ => {
                issues.push(Issue::Type(TypeIssue::NotFound));
            }
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
    fn it_works() {
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
        let expected = vec![Issue::Header(HeaderIssue::NotFound)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_no_header() {
        let commit = "";
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        let expected = vec![Issue::Header(HeaderIssue::NotFound)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_colon_no_header() {
        let commit = r###"
no colon means we do not consider this to be a header at all
"###
        .trim_start();
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        let expected = vec![Issue::Header(HeaderIssue::NotFound)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_word_before_colon_no_type() {
        let commit = r###"
: this means we have no type
"###
        .trim_start();
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        let expected = vec![Issue::Type(TypeIssue::NotFound)];
        assert_eq!(actual, expected);
    }
}
