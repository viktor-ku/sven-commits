use crate::{
    additive::Additive,
    at::At,
    issue::{Issue, IssueData, IssueSubject, Misplaced, Missing},
    paper::Paper,
    weak_commit::{
        parse_header::{Token, TokenKind},
        WeakCommit,
    },
};
use anyhow::Result;

pub fn find_issues(commit: &str) -> Result<Vec<Issue>> {
    let weak_commit = WeakCommit::parse(commit)?;
    let mut issues = Vec::new();

    find_header_issues(&weak_commit.header, &mut issues);

    Ok(issues)
}

fn find_header_issues(tokens: &[Token], issues: &mut Vec<Issue>) {
    println!("tokens {:#?}", tokens);
    let mut paper = Paper::new();

    // find first occurences of every paper token, except the desc
    for token in tokens {
        match token.kind {
            TokenKind::Seq => match paper.type_id {
                Some(_) => {}
                None => paper.type_id = Some(token.id),
            },
            TokenKind::Colon => match paper.colon_id {
                Some(_) => {}
                None => paper.colon_id = Some(token.id),
            },
            TokenKind::Whitespace => match paper.space_id {
                Some(_) => {}
                None => paper.space_id = Some(token.id),
            },
            _ => {}
        }
    }

    // this is our first attempt at figuring out where the desc starts, that is
    // after the at most far token we found + 1
    paper.desc_id = [paper.type_id, paper.colon_id, paper.space_id, paper.desc_id]
        .iter()
        .flatten()
        .max()
        .map(|x| x + 1);

    println!("{:?}", paper);

    if paper.is_empty() {
        issues.push(Issue {
            id: 0,
            subject: IssueSubject::Header,
            data: IssueData::Missing(Missing {
                expected_at: At::start(),
            }),
        });
        return;
    }

    let mut id = Additive::new();

    if paper.type_id.is_none() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Type,
            data: IssueData::Missing(Missing {
                expected_at: At::start(),
            }),
        });
    }
    if paper.colon_id.is_none() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Colon,
            data: IssueData::Missing(Missing {
                expected_at: match paper.type_id {
                    Some(id) => At::after_token(id),
                    None => match issues.last() {
                        Some(issue) => At::after_issue(issue.id),
                        None => unreachable!(),
                    },
                },
            }),
        });
    }
    if paper.space_id.is_none() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Whitespace,
            data: IssueData::Missing(Missing {
                expected_at: match paper.colon_id {
                    Some(id) => At::after_token(id),
                    None => match issues.last() {
                        Some(issue) => At::after_issue(issue.id),
                        None => unreachable!(),
                    },
                },
            }),
        });
    }
    if paper.desc_id.unwrap() == tokens.len() - 1 {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Desc,
            data: IssueData::Missing(Missing {
                expected_at: match paper.space_id {
                    Some(id) => At::after_token(id),
                    None => match issues.last() {
                        Some(issue) => At::after_issue(issue.id),
                        None => unreachable!(),
                    },
                },
            }),
        });
    }

    match (paper.type_id, paper.colon_id) {
        (Some(type_id), Some(colon_id)) => {
            debug_assert!(type_id != colon_id);

            if type_id > colon_id {
                issues.push(Issue {
                    id: id.stamp(),
                    subject: IssueSubject::Type,
                    data: IssueData::Misplaced(Misplaced {
                        expected_at: At::start(),
                        found_at: At::exactly_token(tokens[paper.type_id.unwrap()].id),
                    }),
                });
                issues.push(Issue {
                    id: id.stamp(),
                    subject: IssueSubject::Colon,
                    data: IssueData::Misplaced(Misplaced {
                        expected_at: At::after_token(type_id),
                        found_at: At::start(),
                    }),
                });
            }
        }
        _ => {}
    }

    match (paper.colon_id, paper.space_id) {
        (Some(colon_id), Some(space_id)) => {
            debug_assert!(colon_id != space_id);

            if colon_id > space_id {
                issues.push(Issue {
                    id: id.stamp(),
                    subject: IssueSubject::Colon,
                    data: IssueData::Misplaced(Misplaced {
                        expected_at: At::after_token(todo!()),
                        found_at: At::exactly_token(colon_id),
                    }),
                });
            }
        }
        _ => {}
    }
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
        println!("commit {:?}", commit);
        let actual = find_issues(commit).unwrap();
        assert_eq!(actual, Vec::new());
    }

    #[test]
    fn eol_no_header() {
        let commit = "\n";
        println!("commit {}", commit);
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
        println!("commit {:?}", commit);
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
        println!("commit {:?}", commit);
        let actual = find_issues(commit).unwrap();
        let expected = vec![Issue {
            id: 0,
            subject: IssueSubject::Colon,
            data: IssueData::Missing(Missing {
                expected_at: At::after_token(0),
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
        println!("commit {:?}", commit);
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
                    expected_at: At::after_token(0),
                }),
            },
            Issue {
                id: 2,
                subject: IssueSubject::Desc,
                data: IssueData::Missing(Missing {
                    expected_at: At::after_issue(1),
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
        println!("commit {:?}", commit);
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
                    expected_at: At::after_token(1),
                }),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn whitespace_only() {
        let commit = " \n";
        println!("commit {:?}", commit);
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
                subject: IssueSubject::Colon,
                data: IssueData::Missing(Missing {
                    expected_at: At::after_issue(0),
                }),
            },
            Issue {
                id: 2,
                subject: IssueSubject::Desc,
                data: IssueData::Missing(Missing {
                    expected_at: At::after_token(0),
                }),
            },
        ];
        assert_eq!(actual, expected);
    }

    mod misplaced {
        use super::*;
        use pretty_assertions::assert_eq;

        #[inline]
        fn assert_misplaced(issues: &[Issue], subject: IssueSubject, data: IssueData) {
            match issues.iter().find(|&issue| {
                issue.subject == subject
                    && match (issue.data, data) {
                        (IssueData::Misplaced { .. }, IssueData::Misplaced { .. }) => true,
                        _ => false,
                    }
            }) {
                Some(actual) => {
                    assert_eq!(actual.subject, subject);
                    assert_eq!(actual.data, data);
                }
                None => {
                    panic!("could not find an expected issue");
                }
            }
        }

        mod types {
            use super::*;

            #[test]
            fn after_colon() {
                let commit = r###"
:otherwise desc
"###
                .trim_start();
                println!("commit {:?}", commit);
                let actual = find_issues(commit).unwrap();
                assert_misplaced(
                    &actual,
                    IssueSubject::Type,
                    IssueData::Misplaced(Misplaced {
                        expected_at: At::start(),
                        found_at: At::exactly_token(1),
                    }),
                );
            }

            #[test]
            fn after_whitespace() {
                let commit = r###"
: otherwise perfect commit
"###
                .trim_start();
                println!("commit {:?}", commit);
                let actual = find_issues(commit).unwrap();
                assert_misplaced(
                    &actual,
                    IssueSubject::Type,
                    IssueData::Misplaced(Misplaced {
                        expected_at: At::start(),
                        found_at: At::exactly_token(2),
                    }),
                );
            }
        }

        mod colon {
            use super::*;

            #[test]
            fn before_type() {
                let commit = r###"
:before type clearly
"###
                .trim_start();
                println!("commit {:?}", commit);
                let actual = find_issues(commit).unwrap();
                assert_misplaced(
                    &actual,
                    IssueSubject::Colon,
                    IssueData::Misplaced(Misplaced {
                        expected_at: At::after_token(1),
                        found_at: At::start(),
                    }),
                );
            }

            #[test]
            fn after_whitespace() {
                let commit = r###"
type :desc?
"###
                .trim_start();
                println!("commit {:?}", commit);
                let actual = find_issues(commit).unwrap();
                assert_misplaced(
                    &actual,
                    IssueSubject::Colon,
                    IssueData::Misplaced(Misplaced {
                        expected_at: At::after_token(0),
                        found_at: At::exactly_token(2),
                    }),
                );
            }

            #[test]
            fn after_whitespace_spaced() {
                let commit = " type :desc?\n";
                println!("commit {:?}", commit);
                let actual = find_issues(commit).unwrap();
                assert_misplaced(
                    &actual,
                    IssueSubject::Colon,
                    IssueData::Misplaced(Misplaced {
                        expected_at: At::after_token(1),
                        found_at: At::exactly_token(3),
                    }),
                );
            }
        }
    }
}
