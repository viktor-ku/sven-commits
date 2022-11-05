use anyhow::Result;
use pest::Parser;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct CommitParser;

#[derive(Debug)]
pub struct CommitLike<'a> {
    pub rows: Vec<Row<'a>>,
}

impl<'a> CommitLike<'a> {
    /// Parser takes an arbitrary utf8 compatible string and
    /// returns some rows (being `Vec<Row>`), amongst other meta data
    /// trying to describe the contents of the string.
    ///
    /// Each `Row` corresponds to a new line, even when it's a blank line,
    /// of some text. It's up to the user of this parser to decide how
    /// to interpret these rows. Sensible example of it would be to try
    /// and assume that the first row should be the header of the commit.
    ///
    /// This function does not perform any kind of checks and does not assume
    /// anything about the specification itself, rather, it finds all the rows
    /// and fills up the data in a most convenient way, e.g. counts the number
    /// of rows, probes for the blank lines, gives a way to inspect the shape,
    /// etc.
    ///
    /// ## `blank` field
    ///
    /// Each `Row` has a `blank` field, which indicates whether it's a blank
    /// line (such line consists only out of the new line) or not.
    ///
    /// If you map all rows in a way that takes only `blank` fields,
    /// a typical commit message might look like this:
    /// > `0 1 0 0 0 1 0 0 0 1 0 0 0`
    /// - a header
    /// - two paragraphs of text
    /// - and a three footers
    ///
    /// ...or just this
    /// > `0`
    /// which is the most common - being just the header.
    ///
    /// > 6. ...The body MUST begin one blank line after the description.
    /// So we actually expect the second element (if there more than two elements)
    /// to be 0.
    ///
    /// Seems like we never expect two or more consecutive 0's as it would
    /// idicate two or more balnk lines just being there.
    ///
    /// > 7. A commit body is free-form and MAY consist of any number of
    /// > newline separated paragraphs.
    ///
    /// Having said that, if we are able to get to the number 2 by summing
    /// up all the "blank" fields from the rows vector, resetting the value to
    /// 1 every time then we found an issue (too many pointless blank lines)
    ///
    /// Checking the footer also becomes a trivial task: travel from the end of the
    /// rows looking for the first 1, after which consider footers to be over.
    ///
    /// Assume everything from the end first "1" after the footers going backwards
    /// as well as everything from the first "1" after the header to be the "body"
    /// when verifying the commit structure.
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

        Ok(CommitLike { rows })
    }
}

#[derive(Debug)]
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
    fn must_start_with_a_type() {
        let res = CommitLike::parse("один\nдва\n\n\n\nтри").unwrap();
        println!("{:#?}", res);
        let res = CommitLike::parse("fix(app)!: me").unwrap();
        println!("{:#?}", res);
        assert_eq!(1, 2);
    }
}
