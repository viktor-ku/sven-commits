use anyhow::Result;
use pest::Parser;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct CommitParser;

#[derive(Debug, PartialEq)]
pub struct WeakCommit<'a> {
    pub rows: Vec<Row<'a>>,
}

impl<'a> WeakCommit<'a> {
    pub fn parse(commit: &'a str) -> Result<Self> {
        let mut rows: Vec<Row> = Vec::new();
        let mut row_n: usize = 1;

        match CommitParser::parse(Rule::Lines, commit) {
            Ok(rules) => {
                for rule in rules {
                    match rule.as_rule() {
                        Rule::Lines => {
                            for rule in rule.into_inner() {
                                match rule.as_rule() {
                                    Rule::Row | Rule::RowEOL => {
                                        let span = rule.as_span();
                                        let value = rule.as_str();
                                        rows.push(Row {
                                            value,
                                            row: row_n,
                                            range_bytes: (span.start(), span.end()),
                                            blank: Row::probe_blank_line(value),
                                        });
                                        row_n += 1;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }

        Ok(Self { rows })
    }
}

#[derive(Debug, PartialEq)]
pub struct Row<'row> {
    /// Consists of two integers indicating the start byte index
    /// of the row and the end byte index of the row from the start of the
    /// input.
    pub range_bytes: (usize, usize),

    /// The row starting 1.
    pub row: usize,

    /// An actual row str
    pub value: &'row str,

    /// 1 for the new line,
    /// 0 for any other character
    pub blank: u8,
}

impl<'row> Row<'row> {
    fn probe_blank_line(value: &'row str) -> u8 {
        match CommitParser::parse(Rule::ProbeBlankLine, value) {
            Ok(rules) => {
                for rule in rules {
                    match rule.as_rule() {
                        Rule::ProbeEOL => return 1,
                        Rule::ProbeChar => return 0,
                        _ => unreachable!(),
                    };
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        };

        unreachable!()
    }
}

#[cfg(test)]
mod commits {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        let actual = WeakCommit::parse("fix(app)!: me").unwrap();
        let expected = WeakCommit {
            rows: vec![Row {
                range_bytes: (0, 13),
                row: 1,
                value: "fix(app)!: me",
                blank: 0,
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
                    range_bytes: (0, 4),
                    row: 1,
                    value: "one\n",
                    blank: 0,
                },
                Row {
                    range_bytes: (4, 5),
                    row: 2,
                    value: "\n",
                    blank: 1,
                },
                Row {
                    range_bytes: (5, 9),
                    row: 3,
                    value: "two\n",
                    blank: 0,
                },
                Row {
                    range_bytes: (9, 10),
                    row: 4,
                    value: "\n",
                    blank: 1,
                },
                Row {
                    range_bytes: (10, 15),
                    row: 5,
                    value: "three",
                    blank: 0,
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
                    range_bytes: (0, 7),
                    row: 1,
                    value: "раз\n",
                    blank: 0,
                },
                Row {
                    range_bytes: (7, 13),
                    row: 2,
                    value: "два",
                    blank: 0,
                },
            ],
        };
        assert_eq!(actual, expected);
    }
}
