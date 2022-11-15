use crate::weak_commit::{
    parse_header::{parse_header, Token, TokenKind},
    WeakCommit,
};
use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum Subject {
    Type,
    Header,
    Colon,
    Desc,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Missing {
    pub subject: Subject,
    /// nth byte, starting from 0 (column)
    pub at: usize,
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
    println!("{:#?}", tokens);

    if tokens.is_empty() || (tokens.len() == 1 && tokens.first().unwrap().kind == TokenKind::EOL) {
        issues.push(Issue::Missing(Missing {
            subject: Subject::Header,
            at: 0,
        }));
        return;
    }

    let type_token = tokens.iter().find(|&token| token.kind == TokenKind::Seq);
    let colon_token = tokens.iter().find(|&token| token.kind == TokenKind::Colon);

    if type_token.is_none() {
        issues.push(Issue::Missing(Missing {
            subject: Subject::Type,
            at: 0,
        }));
    }

    if colon_token.is_none() && type_token.is_some() {
        issues.push(Issue::Missing(Missing {
            subject: Subject::Colon,
            at: type_token.unwrap().bytes.end,
        }));
    }

    if colon_token.is_some() {
        let mut iter = tokens.iter();

        iter.find(|&token| token.kind == TokenKind::Colon).unwrap();

        // todo!: handle no more tokens after colon

        if let Some(token) = iter.next() {
            if token.kind == TokenKind::Whitespace {
                if let Some(token) = iter.next() {
                    if token.kind == TokenKind::EOL {
                        issues.push(Issue::Missing(Missing {
                            subject: Subject::Desc,
                            at: token.bytes.start,
                        }));
                    }
                }
            } else if token.kind == TokenKind::EOL {
                issues.push(Issue::Missing(Missing {
                    subject: Subject::Desc,
                    at: token.bytes.end,
                }));
            }
        }

        // by this point iter is at the start of the Desc
    }
}

pub fn find_issues(commit: &str) -> Result<Vec<Issue>> {
    let weak_commit = WeakCommit::parse(commit)?;
    let mut v = Vec::new();

    find_header_issues(&weak_commit.header, &mut v);

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
        let actual = find_issues(commit).unwrap();
        assert_eq!(actual, Vec::new());
    }

    #[test]
    fn eol_no_header() {
        let commit = "\n";
        let actual = find_issues(commit).unwrap();
        let expected = vec![Issue::Missing(Missing {
            subject: Subject::Header,
            at: 0,
        })];
        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_no_header() {
        let commit = "";
        let actual = find_issues(commit).unwrap();
        let expected = vec![Issue::Missing(Missing {
            subject: Subject::Header,
            at: 0,
        })];
        assert_eq!(actual, expected);
    }

    #[test]
    fn consider_first_ever_word_to_be_the_type() {
        let commit = r###"
colon missing after the type "colon"
"###
        .trim_start();
        let actual = find_issues(commit).unwrap();
        let expected = vec![Issue::Missing(Missing {
            subject: Subject::Colon,
            at: 5,
        })];
        assert_eq!(actual, expected);
    }

    #[test]
    fn missing_type_and_desc_if_only_colon() {
        let commit = r###"
:
"###
        .trim_start();
        let actual = find_issues(commit).unwrap();
        let expected = vec![
            Issue::Missing(Missing {
                subject: Subject::Type,
                at: 0,
            }),
            Issue::Missing(Missing {
                subject: Subject::Desc,
                at: 2,
            }),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn missing_type_and_desc_when_nothing_after_colon_then_whitespace() {
        let commit = r###"
: 
# note there is an expected WHITESPACE (" ") character at the end of the header above
"###
        .trim_start();
        println!("{:#?}", commit);
        let actual = find_issues(commit).unwrap();
        let expected = vec![
            Issue::Missing(Missing {
                subject: Subject::Type,
                at: 0,
            }),
            Issue::Missing(Missing {
                subject: Subject::Desc,
                at: 2,
            }),
        ];
        assert_eq!(actual, expected);
    }
}
