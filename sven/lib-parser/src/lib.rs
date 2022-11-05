use anyhow::Result;
use pest::Parser;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct CommitParser;

/// Indicates a position of something:
/// - first element is a row, starts with 1,
/// - second element is a column, starts with 1
#[derive(Debug)]
pub struct Pos(pub u32, pub u32);

#[derive(Debug)]
pub enum CommitToken {
    /// Indicates the start of the commit
    Start(Pos),

    /// Indicates the end of the commit
    End(Pos),

    /// Emphasize where the whitespace " " MUST go
    Whitespace(Pos),

    /// Emphasize where the semicolon ":" MUST go
    Semicolon(Pos),
}

#[derive(Debug)]
pub struct WeakCommit {
    pub tokens: Vec<CommitToken>,
}

#[derive(Debug)]
struct Row<'row> {
    pub start: usize,
    pub end: usize,
    pub value: &'row str,
}

pub fn commit_parser(commit: &str) -> Result<()> {
    let mut rows: Vec<Row> = Vec::new();

    match CommitParser::parse(Rule::Lines, commit) {
        Ok(rules) => {
            // println!("{:#?}", rules);
            for rule in rules {
                match rule.as_rule() {
                    Rule::Lines => {
                        for rule in rule.into_inner() {
                            match rule.as_rule() {
                                Rule::Row | Rule::RowEOL => {
                                    let span = rule.as_span();
                                    let value = rule.as_str();
                                    rows.push(Row {
                                        start: span.start(),
                                        end: span.end(),
                                        value,
                                    });
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

    println!("rows {:#?}", rows);

    Ok(())
}

#[cfg(test)]
mod commits {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn must_start_with_a_type() {
        let res = commit_parser("один\nдва\n\n\n\nтри").unwrap();
        let res = commit_parser("fix(app)!: me").unwrap();
        println!("{:#?}", res);
        assert_eq!(1, 2);
    }
}
