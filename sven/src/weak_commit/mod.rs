use self::parser::{CRule, CommitParser};
use anyhow::Result;
use pest::Parser;

mod bytes_range;
pub use bytes_range::BytesRange;

pub mod parse_header;
mod parser;

mod row;
pub use row::Row;

#[derive(Debug, PartialEq)]
pub struct WeakCommit {
    pub rows: Vec<Row>,
}

impl WeakCommit {
    pub fn parse(commit: &str) -> Result<Self> {
        let mut rows: Vec<Row> = Vec::new();
        let mut row_n: usize = 1;

        let rules = CommitParser::parse(CRule::Lines, commit)?;

        for rule in rules {
            match rule.as_rule() {
                CRule::Lines => {
                    for rule in rule.into_inner() {
                        match rule.as_rule() {
                            CRule::Row | CRule::RowEOL => {
                                let span = rule.as_span();
                                let value = rule.as_str();
                                if !value.is_empty() {
                                    rows.push(Row {
                                        row: row_n,
                                        blank: Row::probe_blank_line(value),
                                        bytes: BytesRange {
                                            start: span.start(),
                                            end: span.end(),
                                        },
                                    });
                                    row_n += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Self { rows })
    }
}

#[cfg(test)]
mod producing {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn singleline() {
        let actual = WeakCommit::parse("fix(app)!: me").unwrap();
        let expected = WeakCommit {
            rows: vec![Row {
                row: 1,
                blank: 0,
                bytes: BytesRange { start: 0, end: 13 },
            }],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiline() {
        let actual = WeakCommit::parse("one\n\ntwo\n\nthree").unwrap();
        let expected = WeakCommit {
            rows: vec![
                Row {
                    row: 1,
                    blank: 0,
                    bytes: BytesRange { start: 0, end: 4 },
                },
                Row {
                    row: 2,
                    blank: 1,
                    bytes: BytesRange { start: 4, end: 5 },
                },
                Row {
                    row: 3,
                    blank: 0,
                    bytes: BytesRange { start: 5, end: 9 },
                },
                Row {
                    row: 4,
                    blank: 1,
                    bytes: BytesRange { start: 9, end: 10 },
                },
                Row {
                    row: 5,
                    blank: 0,
                    bytes: BytesRange { start: 10, end: 15 },
                },
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiline_utf8() {
        let actual = WeakCommit::parse("раз\nдва").unwrap();
        let expected = WeakCommit {
            rows: vec![
                Row {
                    row: 1,
                    blank: 0,
                    bytes: BytesRange { start: 0, end: 7 },
                },
                Row {
                    row: 2,
                    blank: 0,
                    bytes: BytesRange { start: 7, end: 13 },
                },
            ],
        };
        assert_eq!(actual, expected);
    }
}
