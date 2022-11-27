use crate::{
    additive::Additive,
    at::At,
    block::{Block, BlockKind},
    header_issue::header::{Issue, IssueData, IssueSubject, Misplaced, Missing},
    paper::Paper,
};

pub fn find_header_issues(blocks: &[Block]) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut paper = Paper::new();
    let mut id = Additive::new();

    println!("blocks {:#?}", blocks);

    // find first occurences of every paper token, except the desc
    for block in blocks {
        match block.kind {
            BlockKind::Seq => {
                if paper.kind.is_missing() {
                    paper.kind.found_at = Some(block.at);
                }
            }
            BlockKind::Colon => {
                if paper.colon.is_missing() {
                    paper.colon.found_at = Some(block.at);
                }
            }
            BlockKind::Space => {
                if paper.space.is_missing() {
                    paper.space.found_at = Some(block.at);
                }
            }
            _ => {}
        }
    }
    // this is our first attempt at figuring out where the desc starts, that is
    // after the at most far token we found + 1
    let desc_start = [
        paper.kind.found_at,
        paper.colon.found_at,
        paper.space.found_at,
        paper.desc.found_at,
    ]
    .iter()
    .flatten()
    .max()
    .map(|x| x + 1);
    match desc_start {
        Some(desc_start) => {
            paper.desc.found_at = Some(desc_start);
        }
        None => {}
    };

    paper.build_map();

    println!("{:#?}", paper);

    if paper.is_empty() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Header,
            data: IssueData::Missing(Missing {
                expected_at: At::start(),
            }),
        });
        return issues;
    }

    if paper.colon.is_missing() {
        issues.push(Issue {
            id: id.stamp(),
            subject: IssueSubject::Colon,
            data: IssueData::Missing(Missing {
                expected_at: At::after_token(
                    paper
                        .find_pencil(paper.colon.prev.unwrap())
                        .found_at
                        .unwrap(),
                ),
            }),
        });
    }

    issues
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weak_commit::WeakCommit;
    use pretty_assertions::assert_eq;

    fn with_commit(commit: &str) -> Vec<Issue> {
        println!("commit {:?}", commit);
        let w = WeakCommit::parse(commit).unwrap();
        find_header_issues(&w.header)
    }

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
        let actual = with_commit(commit);
        assert_eq!(actual, Vec::new());
    }

    mod no_header {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn only_eol() {
            let actual = with_commit("\n");
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
        fn completely_empty() {
            let actual = with_commit("");
            let expected = vec![Issue {
                id: 0,
                subject: IssueSubject::Header,
                data: IssueData::Missing(Missing {
                    expected_at: At::start(),
                }),
            }];
            assert_eq!(actual, expected);
        }
    }

    mod missing {
        use super::*;
        use pretty_assertions::assert_eq;

        #[inline]
        fn assert_missing(issues: &[Issue], subject: IssueSubject, data: IssueData) {
            match issues.iter().find(|&issue| {
                issue.subject == subject
                    && match (issue.data, data) {
                        (IssueData::Missing { .. }, IssueData::Missing { .. }) => true,
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

        #[test]
        fn colon_after_type() {
            let commit = r###"
colon missing after the type "colon"
"###
            .trim_start();
            let actual = with_commit(commit);
            assert_missing(
                &actual,
                IssueSubject::Colon,
                IssueData::Missing(Missing {
                    expected_at: At::after_token(0),
                }),
            );
        }

        #[test]
        fn all_but_colon() {
            let commit = r###"
:
# no type (no seq at all anywhere)
# no desc (because no tokens where desc should have started, although initially desc is everything)
"###
            .trim_start();
            let actual = with_commit(commit);
            assert_missing(
                &actual,
                IssueSubject::Kind,
                IssueData::Missing(Missing {
                    expected_at: At::start(),
                }),
            );
            assert_missing(
                &actual,
                IssueSubject::Space,
                IssueData::Missing(Missing {
                    expected_at: At::after_token(0),
                }),
            );
            assert_missing(
                &actual,
                IssueSubject::Desc,
                IssueData::Missing(Missing {
                    expected_at: At::after_issue(1),
                }),
            );
        }

        #[test]
        fn type_desc() {
            let commit = r###"
: 
# note there is an expected WHITESPACE (" ") character at the end of the header above
"###
            .trim_start();
            let actual = with_commit(commit);
            assert_missing(
                &actual,
                IssueSubject::Kind,
                IssueData::Missing(Missing {
                    expected_at: At::start(),
                }),
            );
            assert_missing(
                &actual,
                IssueSubject::Desc,
                IssueData::Missing(Missing {
                    expected_at: At::after_token(1),
                }),
            );
        }

        #[test]
        fn all_but_space() {
            let commit = " \n";
            let actual = with_commit(commit);
            assert_missing(
                &actual,
                IssueSubject::Kind,
                IssueData::Missing(Missing {
                    expected_at: At::start(),
                }),
            );
            assert_missing(
                &actual,
                IssueSubject::Colon,
                IssueData::Missing(Missing {
                    expected_at: At::after_issue(0),
                }),
            );
            assert_missing(
                &actual,
                IssueSubject::Desc,
                IssueData::Missing(Missing {
                    expected_at: At::after_token(0),
                }),
            );
        }
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
                let actual = with_commit(commit);
                assert_misplaced(
                    &actual,
                    IssueSubject::Kind,
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
                let actual = with_commit(commit);
                assert_misplaced(
                    &actual,
                    IssueSubject::Kind,
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
                let actual = with_commit(commit);
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
                let actual = with_commit(commit);
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
                let actual = with_commit(commit);
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
