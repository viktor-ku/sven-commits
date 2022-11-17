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

    let mut paper = Paper::default();
    let mut desc_start = 0;

    for token in tokens {
        match token.kind {
            TokenKind::Seq => match paper.type_pocket {
                Some(_) => {
                    if desc_start > 0 {
                        paper.desc_pocket = Some(token);
                    }
                }
                None => paper.type_pocket = Some(token),
            },
            TokenKind::Whitespace => {
                desc_start += 1;
                match paper.whitespace_pocket {
                    Some(_) => {}
                    None => paper.whitespace_pocket = Some(token),
                }
            }
            TokenKind::Colon => match paper.whitespace_pocket {
                Some(_) => {}
                None => paper.colon_pocket = Some(token),
            },
            _ => {}
        }
    }

    println!("{:#?}", paper);

    if paper.type_pocket.is_none() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Type,
            data: IssueData::Missing(Missing {
                expected_at: At {
                    pos: AtPos::After,
                    target: AtTarget::Root,
                },
            }),
        });
    }
    if paper.colon_pocket.is_none() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Colon,
            data: IssueData::Missing(Missing {
                expected_at: At {
                    pos: AtPos::After,
                    target: AtTarget::Token(paper.type_pocket.unwrap().id),
                },
            }),
        });
    }
    if paper.whitespace_pocket.is_none() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Whitespace,
            data: IssueData::Missing(Missing {
                expected_at: At {
                    pos: AtPos::After,
                    target: AtTarget::Token(paper.colon_pocket.unwrap().id),
                },
            }),
        });
    }
    if paper.desc_pocket.is_none() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Desc,
            data: IssueData::Missing(Missing {
                expected_at: At {
                    pos: AtPos::After,
                    target: AtTarget::Issue(2),
                },
            }),
        });
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
        let expected = vec![Issue {
            id: 1,
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
            id: 1,
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
            id: 1,
            subject: IssueSubject::Colon,
            data: IssueData::Missing(Missing {
                expected_at: At {
                    pos: AtPos::After,
                    target: AtTarget::Token(1),
                },
            }),
        }];
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
            Issue {
                id: 1,
                subject: IssueSubject::Type,
                data: IssueData::Missing(Missing {
                    expected_at: At::start(),
                }),
            },
            Issue {
                id: 2,
                subject: IssueSubject::Whitespace,
                data: IssueData::Missing(Missing {
                    expected_at: At {
                        pos: AtPos::After,
                        target: AtTarget::Token(1),
                    },
                }),
            },
            Issue {
                id: 3,
                subject: IssueSubject::Desc,
                data: IssueData::Missing(Missing {
                    expected_at: At {
                        pos: AtPos::After,
                        target: AtTarget::Issue(2),
                    },
                }),
            },
        ];
        assert_eq!(actual, expected);
    }

    //     #[test]
    //     fn missing_type_and_desc_when_nothing_after_colon_then_whitespace() {
    //         let commit = r###"
    // :
    // # note there is an expected WHITESPACE (" ") character at the end of the header above
    // "###
    //         .trim_start();
    //         println!("{:#?}", commit);
    //         let actual = find_issues(commit).unwrap();
    //         let expected = vec![
    //             Issue::Missing(Missing {
    //                 subject: Subject::Type,
    //                 at: 0,
    //             }),
    //             Issue::Missing(Missing {
    //                 subject: Subject::Desc,
    //                 at: 2,
    //             }),
    //         ];
    //         assert_eq!(actual, expected);
    //     }
}
