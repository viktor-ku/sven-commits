use anyhow::{bail, Result};
use pest::Parser;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct CommitParser;

pub fn commit_parser(commit: &str) -> Result<()> {
    match CommitParser::parse(Rule::Commit, commit) {
        Ok(parsed) => {
            println!("{:#?}", parsed);
            Ok(())
        }
        Err(e) => {
            bail!("{:#?}", e)
        }
    }
}

#[cfg(test)]
mod commits {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn must_start_with_a_type() {
        commit_parser(": prevent wrong commits").unwrap();
        assert_eq!(1, 2);
    }
}
