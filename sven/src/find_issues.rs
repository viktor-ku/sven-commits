use crate::{
    additive::Additive,
    at::{At, AtPos, AtTarget},
    issue::{Issue, IssueData, IssueSubject, Missing},
    weak_commit::{
        parse_header::{Token, TokenKind},
        WeakCommit,
    },
};
use anyhow::Result;

#[derive(Debug, Default)]
pub struct Paper<'a> {
    pub type_pocket: Option<&'a Token>,
    pub scope_pocket: Option<&'a Token>,
    pub breaking_pocket: Option<&'a Token>,
    pub colon_pocket: Option<&'a Token>,
    pub whitespace_pocket: Option<&'a Token>,
    pub desc_pocket: Option<&'a Token>,
}

fn find_header_issues(tokens: &[Token], issues: &mut Vec<Issue>) {
    println!("{:#?}", tokens);
    let mut id = Additive::new();

    if tokens.is_empty() || (tokens.len() == 1 && tokens.first().unwrap().kind == TokenKind::EOL) {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Header,
            data: IssueData::Missing(Missing {
                expected_at: At::start(),
            }),
        });
        return;
    }

    // if we have at least something, we consider everything to be desc
    let mut start = 0;

    let type_pocket = tokens.iter().find(|&token| token.kind == TokenKind::Seq);

    // if we find a seq we consider a type, then we push desc cursor, because type cannot be desc
    match type_pocket {
        Some(token) => {
            start = token.id + 1;
        }
        None => issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Type,
            data: IssueData::Missing(Missing {
                expected_at: At::after(AtTarget::Root),
            }),
        }),
    };

    // now that we found a type, we are looking for a colon
    let colon_pocket = tokens.iter().find(|&token| token.kind == TokenKind::Colon);

    // if we find the colon then we also move the desc, since colon is not the desc
    // (any colon or colon after seq?)
    match colon_pocket {
        Some(token) => {
            start = token.id + 1;
        }
        None => issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Colon,
            data: IssueData::Missing(Missing {
                expected_at: At::after(match type_pocket {
                    Some(token) => AtTarget::Token(token.id),
                    None => todo!(),
                }),
            }),
        }),
    }

    let whitespace_pocket = tokens
        .iter()
        .find(|&token| token.kind == TokenKind::Whitespace);

    match whitespace_pocket {
        Some(token) => {
            start = token.id + 1;
        }
        None => issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Whitespace,
            data: IssueData::Missing(Missing {
                expected_at: At::after(match colon_pocket {
                    Some(token) => AtTarget::Token(token.id),
                    None => todo!(),
                }),
            }),
        }),
    }

    // by this point we've pushed the start as far as we could, so we expect to get
    // pretty good idea of where exactly desc should start
    //
    // its only token can't be the EOL
    let desc = tokens.get(start..);

    println!("{:#?}", desc);

    match desc {
        Some(tokens) => {
            if let Some(first) = tokens.first() {
                if first.kind == TokenKind::EOL {
                    issues.push(Issue {
                        id: id.stamp(),
                        subject: IssueSubject::Desc,
                        data: IssueData::Missing(Missing {
                            expected_at: At::after(match whitespace_pocket {
                                Some(whitespace) => AtTarget::Token(whitespace.id),
                                None => AtTarget::Issue(issues.last().unwrap().id),
                            }),
                        }),
                    });
                }
            }
        }
        None => issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Desc,
            data: IssueData::Missing(Missing {
                expected_at: At::after(AtTarget::Issue(issues.last().unwrap().id)),
            }),
        }),
    };
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
        let expected = vec![Issue {
            id: 0,
            subject: IssueSubject::Header,
            data: IssueData::Missing(Missing {
                expected_at: At::start(),
            }),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_no_header() {
        let commit = "";
        let actual = find_issues(commit).unwrap();
        let expected = vec![Issue {
            id: 0,
            subject: IssueSubject::Header,
            data: IssueData::Missing(Missing {
                expected_at: At::start(),
            }),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn consider_first_ever_word_to_be_the_type() {
        let commit = r###"
colon missing after the type "colon"
"###
        .trim_start();
        let actual = find_issues(commit).unwrap();
        let expected = vec![Issue {
            id: 0,
            subject: IssueSubject::Colon,
            data: IssueData::Missing(Missing {
                expected_at: At::after(AtTarget::Token(0)),
            }),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn missing_type_and_desc_if_only_colon() {
        let commit = r###"
:
# no type (no seq at all anywhere)
# no desc (because no tokens where desc should have started, although initially desc is everything)
"###
        .trim_start();
        let actual = find_issues(commit).unwrap();
        let expected = vec![
            Issue {
                id: 0,
                subject: IssueSubject::Type,
                data: IssueData::Missing(Missing {
                    expected_at: At::start(),
                }),
            },
            Issue {
                id: 1,
                subject: IssueSubject::Whitespace,
                data: IssueData::Missing(Missing {
                    expected_at: At::after(AtTarget::Token(0)),
                }),
            },
            Issue {
                id: 2,
                subject: IssueSubject::Desc,
                data: IssueData::Missing(Missing {
                    expected_at: At::after(AtTarget::Issue(1)),
                }),
            },
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
            Issue {
                id: 0,
                subject: IssueSubject::Type,
                data: IssueData::Missing(Missing {
                    expected_at: At::start(),
                }),
            },
            Issue {
                id: 1,
                subject: IssueSubject::Desc,
                data: IssueData::Missing(Missing {
                    expected_at: At::after(AtTarget::Token(1)),
                }),
            },
        ];
        assert_eq!(actual, expected);
    }
}
