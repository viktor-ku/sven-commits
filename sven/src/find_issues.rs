use crate::{
    additive::Additive,
    at::At,
    issue::{Issue, IssueData, IssueSubject, Misplaced, Missing},
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
    let paper_type = 0;
    let paper_colon = 1;
    let paper_whitespace = 2;
    let paper_desc = 3;
    let mut paper: [Option<usize>; 4] = [None; 4];

    // find first occurences of every paper token, except the desc
    for token in tokens {
        match token.kind {
            TokenKind::Seq => match paper[paper_type] {
                Some(_) => {}
                None => paper[paper_type] = Some(token.id),
            },
            TokenKind::Colon => match paper[paper_colon] {
                Some(_) => {}
                None => paper[paper_colon] = Some(token.id),
            },
            TokenKind::Whitespace => match paper[paper_whitespace] {
                Some(_) => {}
                None => paper[paper_whitespace] = Some(token.id),
            },
            _ => {}
        }
    }

    // this is our first attempt at figuring out where the desc starts, that is
    // after the at most far token we found + 1
    paper[paper_desc] = paper.iter().flatten().max().map(|x| x + 1);

    println!("paper {:?}", paper);

    if paper == [None, None, None, None] {
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

    if paper[paper_type].is_none() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Type,
            data: IssueData::Missing(Missing {
                expected_at: At::start(),
            }),
        });
    }
    if paper[paper_colon].is_none() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Colon,
            data: IssueData::Missing(Missing {
                expected_at: match paper[paper_type] {
                    Some(id) => At::after_token(id),
                    None => match issues.last() {
                        Some(issue) => At::after_issue(issue.id),
                        None => unreachable!(),
                    },
                },
            }),
        });
    }
    if paper[paper_whitespace].is_none() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Whitespace,
            data: IssueData::Missing(Missing {
                expected_at: match paper[paper_colon] {
                    Some(id) => At::after_token(id),
                    None => match issues.last() {
                        Some(issue) => At::after_issue(issue.id),
                        None => unreachable!(),
                    },
                },
            }),
        });
    }
    if paper[paper_desc].unwrap() == tokens.len() - 1 {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Desc,
            data: IssueData::Missing(Missing {
                expected_at: match paper[paper_whitespace] {
                    Some(id) => At::after_token(id),
                    None => match issues.last() {
                        Some(issue) => At::after_issue(issue.id),
                        None => unreachable!(),
                    },
                },
            }),
        });
    }

    match (paper[paper_type], paper[paper_colon]) {
        (Some(type_id), Some(colon_id)) => {
            debug_assert!(type_id != colon_id);

            if type_id > colon_id {
                issues.push(Issue {
                    id: id.stamp(),
                    subject: IssueSubject::Type,
                    data: IssueData::Misplaced(Misplaced {
                        expected_at: At::start(),
                        found_at: At::exactly_token(tokens[paper[paper_type].unwrap()].id),
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

    match (paper[paper_colon], paper[paper_whitespace]) {
        (Some(colon_id), Some(whitespace_id)) => {
            debug_assert!(colon_id != whitespace_id);

            if colon_id > whitespace_id {
                issues.push(Issue {
                    id: id.stamp(),
                    subject: IssueSubject::Colon,
                    data: IssueData::Misplaced(Misplaced {
                        expected_at: At::after_token(paper_type),
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
        }
    }
}
