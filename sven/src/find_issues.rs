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
    let colon = tokens.iter().find(|token| **token == Token::Colon);

    if colon.is_none() {
        issues.push(Issue::Header(HeaderIssue::NotFound));
        return;
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
    fn no_colon_no_header() {
        let commit = r###"
no colon means we do not consider this to be a header at all
"###
        .trim_start();
        let actual = find_issues(WeakCommit::parse(commit).unwrap()).unwrap();
        let expected = vec![Issue::Header(HeaderIssue::NotFound)];
        assert_eq!(actual, expected);
    }
}
