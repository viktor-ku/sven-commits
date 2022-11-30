use self::{
    parse_header::parse_header,
    parser::{CRule, CommitParser},
};
use crate::{block::Block, bytes::Bytes};
use anyhow::Result;
use pest::Parser;

pub mod parse_header;
mod parser;

mod row;
pub use row::Row;

#[derive(Debug, PartialEq)]
pub struct WeakCommit {
    pub header: Vec<Block>,
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
                                        bytes: Bytes::new(span.start(), span.end()),
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

        let header = match rows.first() {
            Some(row) => {
                let header_str = row.bytes.capture(&commit);
                parse_header(header_str.expect("could not extract header string"))?
            }
            None => Vec::new(),
        };

        Ok(Self { rows, header })
    }
}

#[cfg(test)]
mod producing {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn singleline() {
        let actual = WeakCommit::parse("fix(app)!: me").unwrap();
        let expected = vec![Row {
            row: 1,
            blank: 0,
            bytes: Bytes::new(0, 13),
        }];
        assert_eq!(actual.rows, expected);
    }

    #[test]
    fn multiline() {
        let actual = WeakCommit::parse("one\n\ntwo\n\nthree").unwrap();
        let expected = vec![
            Row {
                row: 1,
                blank: 0,
                bytes: Bytes::new(0, 4),
            },
            Row {
                row: 2,
                blank: 1,
                bytes: Bytes::new(4, 5),
            },
            Row {
                row: 3,
                blank: 0,
                bytes: Bytes::new(5, 9),
            },
            Row {
                row: 4,
                blank: 1,
                bytes: Bytes::new(9, 10),
            },
            Row {
                row: 5,
                blank: 0,
                bytes: Bytes::new(10, 15),
            },
        ];
        assert_eq!(actual.rows, expected);
    }

    #[test]
    fn multiline_utf8() {
        let actual = WeakCommit::parse("раз\nдва").unwrap();
        let expected = vec![
            Row {
                row: 1,
                blank: 0,
                bytes: Bytes::new(0, 7),
            },
            Row {
                row: 2,
                blank: 0,
                bytes: Bytes::new(7, 13),
            },
        ];
        assert_eq!(actual.rows, expected);
    }
}
